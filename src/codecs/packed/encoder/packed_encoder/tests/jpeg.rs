use crate::packed_encoder::ExtraPadding;
use crate::packed_encoder::NumBytes;
use crate::packed_encoder::Packer;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_std::array2dref::Array2DRef;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;

#[test]
#[should_panic(expected = "not implemented: Bit order JPEG is not packable!")]
fn vec_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::JPEG;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = Cursor::new(vec![]);
    let _packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
}
