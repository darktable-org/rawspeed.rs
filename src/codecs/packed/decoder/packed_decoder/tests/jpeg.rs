use crate::packed_decoder::Unpacker;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

#[test]
#[should_panic(expected = "not implemented: Bit order JPEG is not unpackable!")]
fn u8_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::JPEG;
    type T = u8;
    const NUM_BITS: usize = 8;
    let input: Vec<u8> = vec![0];
    let bytes = Array2DRef::new(&input, RowLength::new(1), RowPitch::new(1));
    let mut storage: Vec<T> = vec![0];
    let mut img =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
}
