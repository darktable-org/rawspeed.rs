use core::marker::PhantomData;
use core::ops::RangeFull;
use rawspeed_common::common::Bitwidth;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_memory_bitstream::bitstream::BitOrderTrait;
use rawspeed_memory_bitstream::bitstream::BitStreamTraits;
use rawspeed_memory_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;
use rawspeed_memory_variable_length_load::variable_length_load::VariableLengthLoad;

pub trait CopyFromSlice {
    fn copy_from_slice_(&mut self, src: &[u8]);
}

impl CopyFromSlice for [u8] {
    fn copy_from_slice_(&mut self, src: &[u8]) {
        self.copy_from_slice(src);
    }
}

pub trait LoadFromSlice<T>
where
    T: Default + core::ops::IndexMut<RangeFull>,
    <T as core::ops::Index<RangeFull>>::Output: CopyFromSlice,
{
    fn load_from_slice(&self) -> T;
}

impl<T> LoadFromSlice<T> for [u8]
where
    T: Default + core::ops::IndexMut<RangeFull>,
    <T as core::ops::Index<RangeFull>>::Output: CopyFromSlice,
{
    fn load_from_slice(&self) -> T {
        let mut out: T = Default::default();
        out[..].copy_from_slice_(self);
        out
    }
}

pub trait FromNeBytes {
    type Output;

    #[allow(clippy::wrong_self_convention)]
    fn from_ne_bytes(self) -> Self::Output;
}

impl FromNeBytes for [u8; 4] {
    type Output = u32;
    fn from_ne_bytes(self) -> Self::Output {
        Self::Output::from_ne_bytes(self)
    }
}

pub trait BitStreamerTraits {
    const TAG: BitOrder;
    const MAX_PROCESS_BYTES: usize;
    type MaxProcessByteArray; // = [u8; _];
}

#[derive(Debug)]
pub struct BitStreamerReplenisher<'a, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
{
    input: &'a [u8],
    pos: usize,
    _phantom_data: PhantomData<T>,
}

impl<'a, T> BitStreamerReplenisher<'a, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
{
    #[allow(dead_code)]
    #[must_use]
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            _phantom_data: PhantomData,
        }
    }

    const fn get_pos(&self) -> usize {
        self.pos
    }

    fn mark_num_bytes_as_consumed(&mut self, num_bytes: usize) {
        self.pos += num_bytes;
    }

    fn get_input(&self) -> Result<T::MaxProcessByteArray, &'static str> {
        let mut tmp: T::MaxProcessByteArray = Default::default();

        // Do we have T::MAX_PROCESS_BYTE_ARRAY or more bytes left in
        // the input buffer? If so, then we can just read from said buffer.
        if let Some(chunk) = self
            .input
            .get(self.pos..)
            .and_then(|s| s.get(..T::MAX_PROCESS_BYTES))
        {
            tmp[..].copy_from_slice_(chunk);
            return Ok(tmp);
        }

        // We have to use intermediate buffer,
        // either because the input is running out of bytes,
        // or because we want to  enforce bounds checking.

        // Note that in order to keep all fill-level invariants
        // we must allow to over-read past-the-end a bit.
        if self.get_pos() > self.input.len() + 2 * T::MAX_PROCESS_BYTES {
            let err: &'static str = "Buffer overflow read in BitStreamer";
            return Err(err);
        }

        tmp[..].variable_length_load(self.input, self.pos);
        Ok(tmp)
    }
}

pub trait BitStreamerDefaultCacheFillImpl<T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
{
    fn fill_cache_impl(&mut self, input: T::MaxProcessByteArray) -> usize;
}

pub trait BitStreamerCacheFillImpl<T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
{
    #[allow(dead_code)]
    fn fill_cache_impl(&mut self, input: T::MaxProcessByteArray) -> usize;
}

pub trait BitStreamerUseDefaultCacheFillImpl {}

#[derive(Debug)]
pub struct BitStreamerBase<'a, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
    T::StreamFlow: BitStreamCache,
{
    replenisher: BitStreamerReplenisher<'a, T>,
    cache: T::StreamFlow,
    _phantom_data: PhantomData<T>,
}

impl<T> BitStreamerDefaultCacheFillImpl<T> for BitStreamerBase<'_, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
    T::StreamFlow: BitStreamCache,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull> +  core::ops::Index<std::ops::Range<usize>>,
    <T::MaxProcessByteArray as core::ops::Index<std::ops::Range<usize>>>::Output:
            LoadFromSlice<T::ChunkByteArrayType>,
    <T::ChunkByteArrayType as core::ops::Index<RangeFull>>::Output: CopyFromSlice,
    T::ChunkByteArrayType:
            Default + core::ops::IndexMut<RangeFull>            + FromNeBytes,
    T::ChunkType: Bitwidth
        + From<<T::ChunkByteArrayType as FromNeBytes>::Output>
        + SwapBytes,
    u64: From<T::ChunkType>,
{
    fn fill_cache_impl(&mut self, input: T::MaxProcessByteArray) -> usize {
        let stream_chunk_bitwidth: usize = T::ChunkType::BITWIDTH;
        assert!(stream_chunk_bitwidth >= 1);
        assert!(stream_chunk_bitwidth % 8 == 0);

        assert!(8 * T::MAX_PROCESS_BYTES >= stream_chunk_bitwidth);
        assert!(8 * T::MAX_PROCESS_BYTES % stream_chunk_bitwidth == 0);

        let num_chunks_needed =
            (8 * T::MAX_PROCESS_BYTES) / stream_chunk_bitwidth;
        assert!(num_chunks_needed >= 1);

        for i in 0..num_chunks_needed {
            let slice = &input
                [i * T::MAX_PROCESS_BYTES..(i + 1) * T::MAX_PROCESS_BYTES];
            let chunk = LoadFromSlice::<T::ChunkByteArrayType>::load_from_slice(slice);
            let chunk = chunk.from_ne_bytes();
            let chunk: T::ChunkType = chunk.into();
            let chunk = chunk
                .get_byte_swapped(T::CHUNK_ENDIANNESS != get_host_endianness());
            self.cache.push(chunk.into(), stream_chunk_bitwidth);
        }
        T::MAX_PROCESS_BYTES
    }
}

impl<T> BitStreamerCacheFillImpl<T> for BitStreamerBase<'_, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
    T::StreamFlow: BitStreamCache,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull> +  core::ops::Index<std::ops::Range<usize>>,
    <T::MaxProcessByteArray as core::ops::Index<std::ops::Range<usize>>>::Output:
            LoadFromSlice<T::ChunkByteArrayType>,
    <T::ChunkByteArrayType as core::ops::Index<RangeFull>>::Output: CopyFromSlice,
    T::ChunkByteArrayType:
            Default + core::ops::IndexMut<RangeFull>
            + FromNeBytes,
    T::ChunkType: Bitwidth
        + From<<T::ChunkByteArrayType as FromNeBytes>::Output>
        + SwapBytes,
    u64: From<T::ChunkType>,
{
    fn fill_cache_impl(&mut self, input: T::MaxProcessByteArray) -> usize {
        BitStreamerDefaultCacheFillImpl::fill_cache_impl(self, input)
    }
}

impl<'a, T> BitStreamerBase<'a, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull>,
    <T::MaxProcessByteArray as core::ops::Index<RangeFull>>::Output:
        CopyFromSlice + VariableLengthLoad,
    T::StreamFlow: BitStreamCache,
    T::MaxProcessByteArray: Default + core::ops::IndexMut<RangeFull> +  core::ops::Index<std::ops::Range<usize>>,
    <T::MaxProcessByteArray as core::ops::Index<std::ops::Range<usize>>>::Output:
            LoadFromSlice<T::ChunkByteArrayType>,
    <T::ChunkByteArrayType as core::ops::Index<RangeFull>>::Output: CopyFromSlice,
    T::ChunkByteArrayType:
            Default + core::ops::IndexMut<RangeFull>
            + FromNeBytes,
    T::ChunkType: Bitwidth
        + From<<T::ChunkByteArrayType as FromNeBytes>::Output>
        + SwapBytes,
    u64: From<T::ChunkType>,
    T::StreamFlow: Default,
{
    #[allow(dead_code)]
    #[must_use]
    pub fn new(input: &'a [u8]) -> Self
    {
        Self {
            replenisher: BitStreamerReplenisher::new(input),
            cache: Default::default(),
            _phantom_data: PhantomData,
        }
    }

    pub fn fill(&mut self, nbits: usize) -> Result<(), &'static str> {
        assert!(nbits != 0);

        if self.cache.fill_level() >= nbits {
            return Ok(());
        }

        let input = self.replenisher.get_input()?;
        let num_bytes =
            BitStreamerCacheFillImpl::<T>::fill_cache_impl(self, input);
        self.replenisher.mark_num_bytes_as_consumed(num_bytes);
        assert!(self.cache.fill_level() >= nbits);
        Ok(())
    }

    pub fn peek_bits(&mut self, nbits: usize) -> Result<u64, &'static str> {
        self.fill(nbits)?;
        let bits = self.cache.peek(nbits);
        Ok(bits)
    }

    pub fn skip_bits(&mut self, nbits: usize) -> Result<(), &'static str> {
        self.fill(nbits)?;
        self.cache.skip(nbits);
        Ok(())
    }

    pub fn get_bits(&mut self, nbits: usize) -> Result<u64, &'static str> {
        let bits = self.peek_bits(nbits)?;
        self.skip_bits(nbits)?;
        Ok(bits)
    }
}

mod lsb;
mod msb;
mod msb32;
