use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache as _;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderJPEG, BitStreamTraits,
};
use rawspeed_common_bitseq::bitseq::{BitLen, BitSeq};
use rawspeed_memory_endianness::endianness::{SwapBytes, get_host_endianness};

use crate::bitstreamer::{
    BitStreamByteSequenceRead, BitStreamerBase, BitStreamerCacheFillImpl,
    BitStreamerTraits, ConcatBytesNe,
};

pub trait DoubleLengthByteArray {
    type ByteArray;
}

impl DoubleLengthByteArray for [u8; 1] {
    type ByteArray = [u8; 2];
}
impl DoubleLengthByteArray for [u8; 2] {
    type ByteArray = [u8; 4];
}
impl DoubleLengthByteArray for [u8; 4] {
    type ByteArray = [u8; 8];
}

impl<ByteArray> BitStreamerTraits<ByteArray> for BitOrderJPEG
where
    ByteArray: DoubleLengthByteArray,
{
    type ByteArray = <ByteArray as DoubleLengthByteArray>::ByteArray;
}

type T = BitOrderJPEG;

impl<R, ByteArray> BitStreamerCacheFillImpl<T, ByteArray>
    for BitStreamerBase<'_, T, ByteArray, R>
where
    R: BitStreamByteSequenceRead,
    for<'a> ByteArray: Default
        + core::ops::Index<core::ops::RangeFull, Output = [u8]>
        + TryFrom<&'a [u8]>
        + ConcatBytesNe
        + DoubleLengthByteArray,
    <ByteArray as ConcatBytesNe>::Output: SwapBytes,
    u64: From<<ByteArray as ConcatBytesNe>::Output>,
    for<'a> <ByteArray as TryFrom<&'a [u8]>>::Error: core::fmt::Debug,
    <T as BitStreamerTraits<ByteArray>>::ByteArray: core::ops::Index<usize, Output = u8>
        + core::ops::Index<core::ops::Range<usize>, Output = [u8]>
        + core::ops::Index<core::ops::RangeFull, Output = [u8]>,
{
    #[inline]
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits<ByteArray>>::ByteArray,
    ) -> usize {
        let n = ByteArray::default()[..].len();
        assert!(input[..].len().is_multiple_of(2));
        let chunk: ByteArray = input[0..n].try_into().unwrap();
        if chunk[..].iter().all(|byte| *byte != 0xFF_u8) {
            let chunk = chunk.concat_bytes_ne();
            let chunk = chunk.get_byte_swapped(
                <T as BitStreamTraits>::CHUNK_ENDIANNESS
                    != get_host_endianness(),
            );
            let bits_per_chunk = 8 * size_of_val(&chunk);
            let bits_per_chunk = bits_per_chunk.try_into().unwrap();
            self.cache.push(
                BitSeq::new(BitLen::new(bits_per_chunk), chunk.into()).unwrap(),
            );
            return n;
        }
        let mut p = 0;
        for i in 0..n {
            let prev_cache = self.cache;
            let num_bytes_needed = n - i;

            // Pre-execute most common case, where next byte is 'normal'/non-FF
            let c0 = input[p];
            let bits = BitSeq::new(BitLen::new(8), c0.into()).unwrap();
            self.cache.push(bits);
            if c0 != 0xFF {
                p += 1;
                continue; // Got normal byte.
            }
            // Found FF -> pre-execute case of FF/00, which represents an FF data byte
            let c1 = input[p + 1];
            if c1 == 0x00 {
                // Got FF/00, where 0x00 is a stuffing byte
                // (that should be ignored) so 0xFF is a normal byte. All good.
                p += 2;
                continue;
            }

            // Found FF/xx with xx != 00. This is the end of stream marker.
            self.cache = prev_cache;
            let zeros = BitSeq::new(
                BitLen::new(self.cache.size() - self.cache.fill_level()),
                0,
            )
            .unwrap();
            self.cache.push(zeros);

            p = self.reader.get_remaining_size() + num_bytes_needed;
            break;
        }
        p
    }
}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerJPEG<'a> = BitStreamerBase<'a, T>;

#[cfg(test)]
mod tests;
