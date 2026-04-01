use rawspeed_bitstream_bitstreambytesequencereader::bitstreambytesequencereader::{BitStreamByteSequenceDefaultReader, BitStreamByteSequenceRead, BitStreamByteSequenceRewind};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::{BitStreamCache, BitStreamCacheData, BitStreamFlowTrait};
use rawspeed_bitstream_bitstreamposition::bitstreamposition::BitstreamPosition;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_common_bitseq::bitseq::{BitLen, BitSeq, BitSeqConstraints};
use rawspeed_common_generic_num::generic_num::bit_transmutation::ConcatBytesNe;
use rawspeed_memory_endianness::endianness::{SwapBytes, get_host_endianness};

pub trait BitStreamerTraits<ByteArray> {
    type ByteArray;
}

pub trait BitStreamerDefaultCacheFillImpl<T, ByteArray>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits<ByteArray>,
{
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits<ByteArray>>::ByteArray,
    ) -> usize where;
}

pub trait BitStreamerUseDefaultCacheFillImpl {}

pub trait BitStreamerCacheFillImpl<T, ByteArray>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits<ByteArray>,
{
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits<ByteArray>>::ByteArray,
    ) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub struct BitStreamerBase<
    'a,
    T,
    R = BitStreamByteSequenceDefaultReader<'a, T>,
    ByteArray = [u8; 4],
> where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
{
    reader: R,
    cache:
        <<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache,
    _phantom_data: core::marker::PhantomData<T>,
    _phantom_data2: core::marker::PhantomData<ByteArray>,
    _lifetime_phantom_data: core::marker::PhantomData<&'a u8>,
}

impl<T, R, ByteArray> BitStreamerDefaultCacheFillImpl<T, ByteArray>
    for BitStreamerBase<'_, T, R, ByteArray>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits+ BitStreamerTraits<ByteArray>,
    <T as BitStreamerTraits<ByteArray>>::ByteArray: core::ops::Index<core::ops::RangeFull, Output = [u8]> + ConcatBytesNe,
    <<T as BitStreamerTraits<ByteArray>>::ByteArray as ConcatBytesNe>::Output: BitStreamCacheData,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64> + BitStreamFlowTrait<<<T as BitStreamerTraits<ByteArray>>::ByteArray as ConcatBytesNe>::Output>,
    for<'b> <T as BitStreamTraits>::MCUByteArrayType: TryFrom<&'b [u8]> + ConcatBytesNe,
    for<'b> <<T as BitStreamTraits>::MCUByteArrayType as TryFrom<&'b [u8]>>::Error: core::fmt::Debug,
    <<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output: SwapBytes,
    <<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<<<T as BitStreamerTraits<ByteArray>>::ByteArray as ConcatBytesNe>::Output>>::Cache as BitStreamCache>::Storage: From<<<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output>,
    <<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage: From<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<<<T as BitStreamerTraits<ByteArray>>::ByteArray as ConcatBytesNe>::Output>>::Cache as BitStreamCache>::Storage>,
{
    #[inline]
        fn fill_cache_impl(
            &mut self,
            input: <T as BitStreamerTraits<ByteArray>>::ByteArray,
        ) -> usize where
     {
        let mut cache: <<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait< <<T as BitStreamerTraits<ByteArray>>::ByteArray as ConcatBytesNe>::Output >>::Cache = Default::default();
        let chunks =
            input[..].chunks_exact(size_of::<
                <T as BitStreamTraits>::MCUByteArrayType,
            >());
        assert!(chunks.remainder().is_empty());
        for chunk in chunks
            .map(|chunk| {
                <T as BitStreamTraits>::MCUByteArrayType::try_from(chunk)
                    .unwrap()
            })
            .map(ConcatBytesNe::concat_bytes_ne)
            .map(|chunk| {
                chunk.get_byte_swapped(
                    T::CHUNK_ENDIANNESS != get_host_endianness(),
                )
            })
        {
            let bits_per_mcu = 8 * size_of_val(&chunk);
            let bits_per_mcu = bits_per_mcu.try_into().unwrap();
            let bits =
                BitSeq::new(BitLen::new(bits_per_mcu), chunk.into()).unwrap();
            cache.push(bits);
        }
        let cache = cache.peek(cache.size());
        self.cache.push(cache.promote());
        input[..].len()
    }
}

impl<T, R, ByteArray> BitStreamerCacheFillImpl<T, ByteArray>
    for BitStreamerBase<'_, T, R, ByteArray>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits<ByteArray>
        + BitStreamerUseDefaultCacheFillImpl,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    Self: BitStreamerDefaultCacheFillImpl<T, ByteArray>,
{
    #[inline]
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits<ByteArray>>::ByteArray,
    ) -> usize {
        BitStreamerDefaultCacheFillImpl::fill_cache_impl(self, input)
    }
}

impl<T, R, ByteArray> BitStreamerBase<'_, T, R, ByteArray>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits<ByteArray>,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
{
    #[inline]
    #[must_use]
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            cache: Default::default(),
            _phantom_data: core::marker::PhantomData,
            _phantom_data2: core::marker::PhantomData,
            _lifetime_phantom_data: core::marker::PhantomData,
        }
    }

    #[inline]
    #[expect(clippy::needless_pass_by_value)]
    pub fn new_with_position(
        reader: R,
        pos: BitstreamPosition<T>,
    ) -> Result<Self, &'static str>
    where
        R: BitStreamByteSequenceRead<T> + BitStreamByteSequenceRewind<T>,
        Self: BitStream,
    {
        let mut reader = reader.rewind();
        reader.mark_num_bytes_as_consumed(pos.mcu_index());
        let mut bs = Self::new(reader);
        if pos.bit_index() != 0 {
            bs.fill(pos.bit_index())?;
            bs.skip_bits_no_fill(pos.bit_index());
        }
        Ok(bs)
    }

    #[inline]
    pub fn get_bitstream_position(&self) -> BitstreamPosition<T>
    where
        R: BitStreamByteSequenceRead<T>,
    {
        let div_ceil_ = |divident: usize, divisor: usize| {
            assert_ne!(divisor, 0);
            let quot_ceil = divident.div_ceil(divisor);
            let rounded_divident = quot_ceil.checked_mul(divisor).unwrap();
            let bias = rounded_divident.checked_sub(divident).unwrap();
            (quot_ceil, bias)
        };

        const { assert!(T::FIXED_SIZE_CHUNKS) };

        let bytes_per_mcu =
            size_of::<<T as BitStreamTraits>::MCUByteArrayType>();
        let bits_per_mcu = 8 * bytes_per_mcu;
        let next_load_pos = self.reader.get_pos();
        let num_bits_in_cache = self.cache.fill_level();
        let num_bits_in_cache = usize::try_from(num_bits_in_cache).unwrap();
        let (num_mcus_in_cache, num_skipped_bits) =
            div_ceil_(num_bits_in_cache, bits_per_mcu);
        let num_bytes_in_cache =
            num_mcus_in_cache.checked_mul(bytes_per_mcu).unwrap();
        let closest_prev_load_pos =
            next_load_pos.checked_sub(num_bytes_in_cache).unwrap();
        let num_skipped_bits = num_skipped_bits.try_into().unwrap();
        BitstreamPosition::new(closest_prev_load_pos, num_skipped_bits)
    }
}

pub trait BitStream
where
    Self::T: BitSeqConstraints + core::fmt::Debug,
{
    type T;

    fn fill(&mut self, nbits: u32) -> Result<(), &'static str>;

    fn peek_bits_no_fill(&mut self, nbits: u32) -> BitSeq<Self::T>;

    fn skip_bits_no_fill(&mut self, nbits: u32);
}

impl<T, R, ByteArray> BitStream for BitStreamerBase<'_, T, R, ByteArray>
where
    T: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits<ByteArray>,
    R: BitStreamByteSequenceRead<T>,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    for<'a> <T as BitStreamerTraits<ByteArray>>::ByteArray: Default
        + core::ops::IndexMut<core::ops::RangeFull, Output = [u8]>
        + TryFrom<&'a [u8]>,
    for<'a> <<T as BitStreamerTraits<ByteArray>>::ByteArray as TryFrom<&'a [u8]>>::Error: core::fmt::Debug,
    Self: BitStreamerCacheFillImpl<T, ByteArray>,
    <<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage: core::fmt::Debug,
{
    type T = <<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage;

    #[inline]
    fn fill(&mut self, nbits: u32) -> Result<(), &'static str> {
        assert!(nbits != 0);

        if self.cache.fill_level() >= nbits {
            return Ok(());
        }

        let input = self.reader.peek_input()?;
        let num_bytes = BitStreamerCacheFillImpl::fill_cache_impl(self, input);
        self.reader.mark_num_bytes_as_consumed(num_bytes);
        Ok(())
    }

    #[inline]
    fn peek_bits_no_fill(
        &mut self,
        nbits: u32,
    ) -> BitSeq<Self::T>
    {
        self.cache.peek(nbits)
    }

    #[inline]
    fn skip_bits_no_fill(&mut self, nbits: u32) {
        self.cache.skip(nbits);
    }
}

mod jpeg;
mod lsb;
mod msb;
mod msb16;
mod msb32;

#[cfg(test)]
mod tests;
