use core::marker::PhantomData;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCacheBase;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderTrait;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;
use rawspeed_common::common::Bitwidth;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;

#[cfg(target_endian = "little")]
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCacheHighInLowOut;
#[cfg(not(target_endian = "little"))]
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCacheLowInHighOut;

pub trait BitVacuumer {}

pub trait BitVacuumerDefaultDrainImpl {
    fn drain_impl(&mut self) -> std::io::Result<()>;
}

pub trait BitVacuumerDrainImpl {
    fn drain_impl(&mut self) -> std::io::Result<()>;
}

pub trait BitVacuumerUseDefaultDrainImpl {}

#[derive(Debug)]
pub struct BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    T::StreamFlow: BitStreamCache + Default,
{
    cache: T::StreamFlow,
    writer: &'a mut W,
    _phantom_data: PhantomData<T>,
}

impl<T, W> BitVacuumerDefaultDrainImpl for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    T::StreamFlow: BitStreamCache + Default,
    T::ChunkType: Bitwidth
        + TryFrom<<T::StreamFlow as BitStreamCache>::Storage>
        + SwapBytes,
    u32: From<T::ChunkType>,
{
    #[inline]
    fn drain_impl(&mut self) -> std::io::Result<()> {
        let mut cache: BitStreamCacheBase<_, u32> = {
            #[cfg(target_endian = "little")]
            {
                BitStreamCacheHighInLowOut::<u32>::new()
            }
            #[cfg(not(target_endian = "little"))]
            {
                BitStreamCacheLowInHighOut::<u32>::new()
            }
        };

        assert!(self.cache.fill_level() >= cache.size());

        let stream_chunk_bitwidth: usize = T::ChunkType::BITWIDTH;

        assert!(cache.size() >= stream_chunk_bitwidth);
        assert!(cache.size().is_multiple_of(stream_chunk_bitwidth));
        let num_chunks_needed: usize = cache.size() / stream_chunk_bitwidth;
        assert!(num_chunks_needed >= 1);

        for _i in 0..num_chunks_needed {
            let Ok(chunk) = <T::ChunkType>::try_from(
                self.cache.peek(stream_chunk_bitwidth),
            ) else {
                panic!("lossless cast failed?")
            };
            self.cache.skip(stream_chunk_bitwidth);
            let chunk = chunk
                .get_byte_swapped(T::CHUNK_ENDIANNESS != get_host_endianness());
            cache.push(chunk.into(), stream_chunk_bitwidth);
        }
        let bytes = cache.peek(cache.size()).to_ne_bytes();
        self.writer.write_all(&bytes)
    }
}

impl<T, W> BitVacuumerDrainImpl for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits + BitVacuumerUseDefaultDrainImpl,
    W: std::io::Write,
    T::StreamFlow: BitStreamCache + Default,
    T::ChunkType: Bitwidth
        + TryFrom<<T::StreamFlow as BitStreamCache>::Storage>
        + SwapBytes,
    u32: From<T::ChunkType>,
{
    #[inline]
    fn drain_impl(&mut self) -> std::io::Result<()> {
        BitVacuumerDefaultDrainImpl::drain_impl(self)
    }
}

impl<'a, T, W> BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    Self: BitVacuumerDrainImpl,
    W: std::io::Write,
    T::StreamFlow: BitStreamCache + Default,
    T::ChunkType: Bitwidth
        + TryFrom<<T::StreamFlow as BitStreamCache>::Storage>
        + SwapBytes,
    u32: From<T::ChunkType>,
    <T::StreamFlow as BitStreamCache>::Storage: From<u64>,
{
    #[inline]
    pub fn new(writer: &'a mut W) -> Self
    where
        T::StreamFlow: Default,
    {
        Self {
            cache: Default::default(),
            writer,
            _phantom_data: PhantomData,
        }
    }

    #[inline]
    pub fn flush(mut self) -> std::io::Result<()> {
        self.drain()?;

        if self.cache.fill_level() == 0 {
            return Ok(());
        }

        // Pad with zero bits, so we can drain the partial chunk.
        self.put(/*bits=*/ 0, u32::BITWIDTH - self.cache.fill_level())?;
        assert!(self.cache.fill_level() == u32::BITWIDTH);

        self.drain()?;
        assert!(self.cache.fill_level() == 0);

        self.writer.flush()
    }

    #[inline]
    pub fn drain(&mut self) -> std::io::Result<()> {
        if self.cache.fill_level() < u32::BITWIDTH {
            return Ok(()); // NOTE: does not mean the cache is empty!
        }

        BitVacuumerDrainImpl::drain_impl(self)?;

        assert!(self.cache.fill_level() < u32::BITWIDTH);
        Ok(())
    }

    #[inline]
    pub fn put(&mut self, bits: u64, count: usize) -> std::io::Result<()> {
        // NOTE: count may be zero!
        self.drain()?;
        self.cache.push(bits.into(), count);
        Ok(())
    }
}

impl<T, W> Drop for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    T::StreamFlow: BitStreamCache + Default,
{
    #[inline]
    fn drop(&mut self) {
        const ERR: &str = "Unrecoverable Error: trying to drop \
            non-empty BitVacuumer. Did you forget to call `flush()`?";
        assert!((self.cache.fill_level() == 0), "{}", ERR);
    }
}

pub mod jpeg;
pub mod lsb;
pub mod msb;
pub mod msb16;
pub mod msb32;
