use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerBase;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerCacheFillImpl;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerTraits;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderTrait;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;
use rawspeed_common_generic_num::generic_num::bit_transmutation::FromNeBytes;
use rawspeed_common_generic_num::generic_num::common::Bitwidth;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_fixed_length_load::fixed_length_load::CopyFromSlice;
use rawspeed_memory_fixed_length_load::fixed_length_load::LoadFromSlice;
use rawspeed_memory_variable_length_load::variable_length_load::VariableLengthLoad;

#[inline]
#[must_use]
pub fn derive_mcu_bytesize<BitOrder>() -> usize
where
        BitOrder: BitOrderTrait + BitStreamTraits + BitStreamerTraits,
        for<'a> BitStreamerBase<'a, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
        <BitOrder::MaxProcessByteArray as core::ops::Index<
            core::ops::RangeFull,
        >>::Output: CopyFromSlice + VariableLengthLoad,
        BitOrder::StreamFlow: Default + BitStreamCache,
        BitOrder::MaxProcessByteArray: Default
            + core::ops::IndexMut<core::ops::RangeFull>
            + core::ops::Index<core::ops::Range<usize>>,
        <BitOrder::MaxProcessByteArray as core::ops::Index<
            core::ops::Range<usize>,
        >>::Output: LoadFromSlice<BitOrder::ChunkByteArrayType>,
        <BitOrder::ChunkByteArrayType as core::ops::Index<
            core::ops::RangeFull,
        >>::Output: CopyFromSlice,
        BitOrder::ChunkByteArrayType:
            Default + core::ops::IndexMut<core::ops::RangeFull> + FromNeBytes,
        <BitOrder::ChunkByteArrayType as FromNeBytes>::Output: Bitwidth
            + From<<BitOrder::ChunkByteArrayType as FromNeBytes>::Output>
            + SwapBytes,
        u64: From<<BitOrder::StreamFlow as BitStreamCache>::Storage>,
{
    let input: [u8; 255] =
        core::array::from_fn(|i| u8::try_from(1 + i).unwrap());

    for mcu_bytesize in 1_usize.. {
        const BITS_PER_BYTE: usize = 8;
        let mcu_getter = |bs: &mut BitStreamerBase<'_, BitOrder>| {
            let mut elts = vec![];
            for _i in 0..mcu_bytesize {
                bs.fill(BITS_PER_BYTE).unwrap();
                let elt = bs.peek_bits_no_fill(BITS_PER_BYTE);
                assert_ne!(elt, 0);
                elts.push(elt);
                bs.skip_bits_no_fill(BITS_PER_BYTE);
            }
            assert_eq!(elts.len(), mcu_bytesize);
            elts
        };
        let mut full_bs = BitStreamerBase::<BitOrder>::new(&input);
        let _first_mcu = mcu_getter(&mut full_bs);
        let second_mcu = mcu_getter(&mut full_bs);
        let mut new_bs = BitStreamerBase::<BitOrder>::new(
            input.get(mcu_bytesize..).unwrap(),
        );
        let new_first_mcu = mcu_getter(&mut new_bs);
        if new_first_mcu == second_mcu {
            return mcu_bytesize;
        }
    }
    unreachable!()
}
