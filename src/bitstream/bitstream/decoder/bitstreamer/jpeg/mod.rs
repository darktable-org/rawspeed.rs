use super::BitStreamerBase;
use super::BitStreamerCacheFillImpl;
use super::BitStreamerTraits;
use super::CopyFromSlice;
use super::FromNeBytes;
use super::LoadFromSlice;

use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderJPEG;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderTrait;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;
use rawspeed_common_generic_num::generic_num::common::Bitwidth;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;
use rawspeed_memory_variable_length_load::variable_length_load::VariableLengthLoad;

impl BitStreamerTraits for BitOrderJPEG {
    const TAG: BitOrder = BitOrder::JPEG;
    const MAX_PROCESS_BYTES: usize = 8;
    type MaxProcessByteArray = [u8; 8];
}

type T = BitOrderJPEG;

impl BitStreamerCacheFillImpl<T> for BitStreamerBase<'_, T>
where
    T: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    <<T as BitStreamerTraits>::MaxProcessByteArray as core::ops::Index<
        core::ops::RangeFull,
    >>::Output: CopyFromSlice + VariableLengthLoad,
    <T as BitStreamTraits>::StreamFlow: BitStreamCache,
    <T as BitStreamerTraits>::MaxProcessByteArray: Default
        + core::ops::IndexMut<core::ops::RangeFull>
        + core::ops::Index<core::ops::Range<usize>>,
    <<T as BitStreamerTraits>::MaxProcessByteArray as core::ops::Index<
        core::ops::Range<usize>,
    >>::Output: LoadFromSlice<<T as BitStreamTraits>::ChunkByteArrayType>,
    <<T as BitStreamTraits>::ChunkByteArrayType as core::ops::Index<
        core::ops::RangeFull,
    >>::Output: CopyFromSlice,
    <T as BitStreamTraits>::ChunkByteArrayType:
        Default + core::ops::IndexMut<core::ops::RangeFull> + FromNeBytes,
    <<T as BitStreamTraits>::ChunkByteArrayType as FromNeBytes>::Output: Bitwidth
        + From<<<T as BitStreamTraits>::ChunkByteArrayType as FromNeBytes>::Output> + SwapBytes,
    u64: From<<<T as BitStreamTraits>::ChunkByteArrayType as FromNeBytes>::Output>,
{
    #[inline]
    fn fill_cache_impl(
        &mut self,
        input: <T as BitStreamerTraits>::MaxProcessByteArray,
    ) -> usize {
        let chunk = LoadFromSlice::<[u8; 4]>::load_from_slice(&input[0..4]);
        if chunk.iter().all(|byte| *byte != 0xFF_u8) {
            type ChunkType = <<T as BitStreamTraits>::ChunkByteArrayType as FromNeBytes>::Output;
            let chunk = chunk.from_ne_bytes();
            let chunk = chunk.get_byte_swapped(
                <T as BitStreamTraits>::CHUNK_ENDIANNESS
                    != get_host_endianness(),
            );
            self.cache.push(
                chunk.into(),
                ChunkType::BITWIDTH,
            );
            return 4;
        }
        let mut p = 0;
        for i in 0..4 {
            let prev_cache = self.cache;
            let num_bytes_needed = 4 - i;

            // Pre-execute most common case, where next byte is 'normal'/non-FF
            let c0 = *input.get(p).unwrap();
            self.cache.push(c0.into(), 8);
            if c0 != 0xFF {
                p += 1;
                continue; // Got normal byte.
            }
            // Found FF -> pre-execute case of FF/00, which represents an FF data byte
            let c1 = *input.get(p + 1).unwrap();
            if c1 == 0x00 {
                // Got FF/00, where 0x00 is a stuffing byte
                // (that should be ignored) so 0xFF is a normal byte. All good.
                p += 2;
                continue;
            }

            // Found FF/xx with xx != 00. This is the end of stream marker.
            self.cache = prev_cache;
            self.cache.push(
                0,
                <T as BitStreamTraits>::StreamFlow::SIZE
                    - self.cache.fill_level(),
            );

            p = self.replenisher.get_remaining_size() + num_bytes_needed;
            break;
        }
        p
    }
}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerJPEG<'a> = BitStreamerBase<'a, BitOrderJPEG>;

#[cfg(test)]
mod tests;
