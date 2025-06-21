use core::marker::PhantomData;
use rawspeed_memory_bitstream::bitstream::BitOrderTrait;
use rawspeed_memory_bitstream::bitstream::BitStreamTraits;
use rawspeed_memory_bitstreamcache::bitstreamcache;
use rawspeed_memory_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_memory_bitstreamcache::bitstreamcache::Bitwidth;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;

pub trait BitVacuumerDefaultDrainImpl {
    fn drain_impl(&mut self) -> std::io::Result<()>;
}

pub trait BitVacuumerDrainImpl {
    #[allow(dead_code)]
    fn drain_impl(&mut self) -> std::io::Result<()>;
}

pub trait BitVacuumerUseDefaultDrainImpl {}

pub struct BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<T::ChunkType>
        + std::ops::Shl<usize>
        + std::ops::ShlAssign<usize>
        + std::ops::BitOrAssign,
    T::StreamFlow: bitstreamcache::BitStreamCache,
    T::ChunkType: Bitwidth + SwapBytes + TryFrom<u64>,
{
    cache: T::StreamFlow,
    writer: &'a mut W,
    _phantom_data: PhantomData<T>,
}

impl<'a, T, W> BitVacuumerDefaultDrainImpl for BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<T::ChunkType>
        + std::ops::Shl<usize>
        + std::ops::ShlAssign<usize>
        + std::ops::BitOrAssign,
    T::StreamFlow: bitstreamcache::BitStreamCache,
    T::ChunkType: Bitwidth + SwapBytes + TryFrom<u64>,
{
    fn drain_impl(&mut self) -> std::io::Result<()> {
        type WritebackCache = u32;

        assert!(self.cache.fill_level() >= WritebackCache::BITWIDTH);

        let stream_chunk_bitwidth: usize = T::ChunkType::BITWIDTH;

        assert!(WritebackCache::BITWIDTH >= stream_chunk_bitwidth);
        assert!(WritebackCache::BITWIDTH % stream_chunk_bitwidth == 0);
        let num_chunks_needed: usize =
            WritebackCache::BITWIDTH / stream_chunk_bitwidth;
        assert!(num_chunks_needed >= 1);

        let mut cache: WritebackCache = Default::default();
        for _i in 0..num_chunks_needed {
            let chunk = match <T::ChunkType>::try_from(
                self.cache.peek(stream_chunk_bitwidth),
            ) {
                Ok(t) => t,
                Err(_) => panic!("lossless cast failed?"),
            };
            self.cache.skip(stream_chunk_bitwidth);
            let chunk = chunk
                .get_byte_swapped(T::CHUNK_ENDIANNESS != get_host_endianness());
            let chunk: WritebackCache = chunk.into();
            let chunk: WritebackCache = {
                #[cfg(target_endian = "little")]
                {
                    chunk << (_i * stream_chunk_bitwidth)
                }
                #[cfg(not(target_endian = "little"))]
                {
                    if num_chunks_needed != 1 {
                        cache <<= stream_chunk_bitwidth;
                    }
                    chunk
                }
            };
            cache |= chunk;
        }
        let bytes = cache.to_ne_bytes();
        self.writer.write_all(&bytes)
    }
}

impl<'a, T, W> BitVacuumerDrainImpl for BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits + BitVacuumerUseDefaultDrainImpl,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<T::ChunkType>
        + std::ops::Shl<usize>
        + std::ops::ShlAssign<usize>
        + std::ops::BitOrAssign,
    T::StreamFlow: bitstreamcache::BitStreamCache,
    T::ChunkType: Bitwidth + SwapBytes + TryFrom<u64>,
{
    fn drain_impl(&mut self) -> std::io::Result<()> {
        BitVacuumerDefaultDrainImpl::drain_impl(self)
    }
}

impl<'a, T, W> BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    Self: BitVacuumerDrainImpl,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<T::ChunkType>
        + std::ops::Shl<usize>
        + std::ops::ShlAssign<usize>
        + std::ops::BitOrAssign,
    T::StreamFlow: bitstreamcache::BitStreamCache,
    T::ChunkType: Bitwidth + SwapBytes + TryFrom<u64>,
{
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

        Ok(())
    }

    #[allow(dead_code)]
    pub fn drain(&mut self) -> std::io::Result<()> {
        if self.cache.fill_level() < u32::BITWIDTH {
            return Ok(()); // NOTE: does not mean the cache is empty!
        }

        BitVacuumerDrainImpl::drain_impl(self)?;

        assert!(self.cache.fill_level() < u32::BITWIDTH);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn put(&mut self, bits: u64, count: usize) -> std::io::Result<()> {
        // NOTE: count may be zero!
        self.drain()?;
        self.cache.push(bits, count);
        Ok(())
    }
}

impl<'a, T, W> Drop for BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<T::ChunkType>
        + std::ops::Shl<usize>
        + std::ops::ShlAssign<usize>
        + std::ops::BitOrAssign,
    T::StreamFlow: bitstreamcache::BitStreamCache,
    T::ChunkType: Bitwidth + SwapBytes + TryFrom<u64>,
{
    fn drop(&mut self) {
        let err: &'static str = "Unrecoverable Error: trying to drop \
            non-empty BitVacuumer. Did you forget to call `flush()`?";
        if self.cache.fill_level() != 0 {
            panic!("{}", err)
        }
    }
}

mod jpeg;
mod lsb;
mod msb;
mod msb16;
mod msb32;
