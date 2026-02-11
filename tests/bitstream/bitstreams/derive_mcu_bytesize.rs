use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerBase;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerCacheFillImpl;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerReplenisher;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerReplenisherStorage;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerTraits;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderTrait;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;

fn round_down_to_multiple_of(val: usize, mult: usize) -> usize {
    assert_ne!(mult, 0);
    mult * (val / mult)
}

fn get_as_valid_bitstreamslice<T>(input: &[u8]) -> BitStreamSlice<'_, T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
{
    let mcu_size = size_of::<<T as BitStreamTraits>::MCUByteArrayType>();
    let len = round_down_to_multiple_of(input.len(), mcu_size);
    let input = input.get(..len).unwrap();
    input.try_into().unwrap()
}

#[inline]
#[must_use]
pub fn derive_mcu_bytesize<BitOrder>() -> usize
where
    BitOrder:
        Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
    for<'a> BitStreamerBase<'a, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
    for<'a> BitStreamerReplenisherStorage<'a, BitOrder>:
        BitStreamerReplenisher<'a, BitOrder>,
    <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
    u64: From<
        <<BitOrder as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage,
    >,
{
    let input: [u8; 255] =
        core::array::from_fn(|i| u8::try_from(1 + i).unwrap());

    for mcu_bytesize in 1_usize.. {
        const BITS_PER_BYTE: u32 = 8;
        let mcu_getter = |bs: &mut BitStreamerBase<'_, BitOrder>| {
            let mut elts = vec![];
            for _i in 0..mcu_bytesize {
                bs.fill(BITS_PER_BYTE).unwrap();
                let elt = bs.peek_bits_no_fill(BITS_PER_BYTE).zext();
                assert_ne!(elt, 0);
                elts.push(elt);
                bs.skip_bits_no_fill(BITS_PER_BYTE);
            }
            assert_eq!(elts.len(), mcu_bytesize);
            elts
        };
        let mut full_bs = BitStreamerBase::<BitOrder>::new(
            get_as_valid_bitstreamslice(input.as_slice()),
        );
        let _first_mcu = mcu_getter(&mut full_bs);
        let second_mcu = mcu_getter(&mut full_bs);
        let mut new_bs = BitStreamerBase::<BitOrder>::new(
            get_as_valid_bitstreamslice(input.get(mcu_bytesize..).unwrap()),
        );
        let new_first_mcu = mcu_getter(&mut new_bs);
        if new_first_mcu == second_mcu {
            return mcu_bytesize;
        }
    }
    unreachable!()
}
