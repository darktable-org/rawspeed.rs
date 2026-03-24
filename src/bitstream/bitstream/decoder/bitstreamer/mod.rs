use rawspeed_bitstream_bitstreambytesequencereader::bitstreambytesequencereader::{BitStreamByteSequenceDefaultReader, BitStreamByteSequenceRead};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrder, BitOrderTrait, BitStreamTraits,
};
use rawspeed_common_bitseq::bitseq::{BitLen, BitSeq};
use rawspeed_common_generic_num::generic_num::{
    bit_transmutation::ConcatBytesNe, common::Bitwidth,
};
use rawspeed_memory_endianness::endianness::{SwapBytes, get_host_endianness};

pub trait BitStreamerTraits
where
    for<'a> Self::MaxProcessByteArray: Default
        + core::ops::IndexMut<core::ops::RangeFull, Output = [u8]>
        + TryFrom<&'a [u8]>,
    for<'a> <Self::MaxProcessByteArray as TryFrom<&'a [u8]>>::Error:
        core::fmt::Debug,
{
    const TAG: BitOrder;
    const MAX_PROCESS_BYTES: usize;
    type MaxProcessByteArray; // = [u8; _];
}

pub trait BitStreamerDefaultCacheFillImpl<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
{
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits>::MaxProcessByteArray,
    ) -> usize;
}

pub trait BitStreamerUseDefaultCacheFillImpl {}

pub trait BitStreamerCacheFillImpl<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
{
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits>::MaxProcessByteArray,
    ) -> usize;
}

#[derive(Debug)]
pub struct BitStreamerBase<'a, T, R = BitStreamByteSequenceDefaultReader<'a, T>>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamCache,
    R: BitStreamByteSequenceRead<T>,
{
    reader: R,
    cache: T::StreamFlow,
    _phantom_data: core::marker::PhantomData<T>,
    _lifetime_phantom_data: core::marker::PhantomData<&'a u8>,
}

impl<T, R> BitStreamerDefaultCacheFillImpl<T> for BitStreamerBase<'_, T, R>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    R: BitStreamByteSequenceRead<T>,
    <T as BitStreamTraits>::StreamFlow: BitStreamCache,
    <T as BitStreamerTraits>::MaxProcessByteArray:
        core::ops::Index<core::ops::Range<usize>, Output = [u8]>,
    for<'b> <T as BitStreamTraits>::ChunkByteArrayType: TryFrom<&'b [u8]>,
    for<'b> <<T as BitStreamTraits>::ChunkByteArrayType as TryFrom<&'b [u8]>>::Error:
        core::fmt::Debug,
    T::ChunkByteArrayType: ConcatBytesNe,
    <T::ChunkByteArrayType as ConcatBytesNe>::Output: Bitwidth + SwapBytes,
    <T::StreamFlow as BitStreamCache>::Storage:
        From<<T::ChunkByteArrayType as ConcatBytesNe>::Output>,
{
    #[inline]
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits>::MaxProcessByteArray,
    ) -> usize {
        let stream_chunk_bitwidth: usize =
            <T::ChunkByteArrayType as ConcatBytesNe>::Output::BITWIDTH
                .try_into()
                .unwrap();
        assert!(stream_chunk_bitwidth >= 1);
        assert!(stream_chunk_bitwidth.is_multiple_of(8));

        assert!(8 * T::MAX_PROCESS_BYTES >= stream_chunk_bitwidth);
        assert!(
            (8 * T::MAX_PROCESS_BYTES).is_multiple_of(stream_chunk_bitwidth)
        );

        let num_chunks_needed =
            (8 * T::MAX_PROCESS_BYTES) / stream_chunk_bitwidth;
        assert!(num_chunks_needed >= 1);

        for i in 0..num_chunks_needed {
            let slice = &input[i * (stream_chunk_bitwidth / 8)
                ..(i + 1) * (stream_chunk_bitwidth / 8)];
            let chunk = <T::ChunkByteArrayType>::try_from(slice).unwrap();
            let chunk = chunk.concat_bytes_ne();
            let chunk = chunk
                .get_byte_swapped(T::CHUNK_ENDIANNESS != get_host_endianness());
            let bits = BitSeq::new(
                BitLen::new(stream_chunk_bitwidth.try_into().unwrap()),
                chunk.into(),
            )
            .unwrap();
            self.cache.push(bits);
        }
        T::MAX_PROCESS_BYTES
    }
}

impl<T, R> BitStreamerCacheFillImpl<T> for BitStreamerBase<'_, T, R>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits
        + BitStreamerUseDefaultCacheFillImpl,
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    R: BitStreamByteSequenceRead<T>,
    <T as BitStreamTraits>::StreamFlow: BitStreamCache,
    <T as BitStreamerTraits>::MaxProcessByteArray:
        core::ops::Index<core::ops::Range<usize>, Output = [u8]>,
    for<'b> <T as BitStreamTraits>::ChunkByteArrayType: TryFrom<&'b [u8]>,
    for<'b> <<T as BitStreamTraits>::ChunkByteArrayType as TryFrom<&'b [u8]>>::Error:
        core::fmt::Debug,
    T::ChunkByteArrayType: ConcatBytesNe,
    <T::ChunkByteArrayType as ConcatBytesNe>::Output: Bitwidth + SwapBytes,
    <T::StreamFlow as BitStreamCache>::Storage:
        From<<T::ChunkByteArrayType as ConcatBytesNe>::Output>,
{
    #[inline]
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits>::MaxProcessByteArray,
    ) -> usize {
        BitStreamerDefaultCacheFillImpl::fill_cache_impl(self, input)
    }
}

impl<T, R> BitStreamerBase<'_, T, R>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    R: BitStreamByteSequenceRead<T>,
    Self: BitStreamerCacheFillImpl<T>,
    <T as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
    BitSeq<u64>: From<
        BitSeq<<<T as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage>,
    >,
{
    #[inline]
    #[must_use]
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            cache: Default::default(),
            _phantom_data: core::marker::PhantomData,
            _lifetime_phantom_data: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn fill(&mut self, nbits: u32) -> Result<(), &'static str> {
        assert!(nbits != 0);

        if self.cache.fill_level() >= nbits {
            return Ok(());
        }

        let input = self.reader.peek_input()?;
        let num_bytes =
            BitStreamerCacheFillImpl::<T>::fill_cache_impl(self, input);
        self.reader.mark_num_bytes_as_consumed(num_bytes);
        assert!(self.cache.fill_level() >= nbits);
        Ok(())
    }

    #[inline]
    pub fn peek_bits_no_fill(&mut self, nbits: u32) -> BitSeq<u64> {
        self.cache.peek(nbits).into()
    }

    #[inline]
    pub fn skip_bits_no_fill(&mut self, nbits: u32) {
        self.cache.skip(nbits);
    }
}

mod jpeg;
mod lsb;
mod msb;
mod msb16;
mod msb32;
