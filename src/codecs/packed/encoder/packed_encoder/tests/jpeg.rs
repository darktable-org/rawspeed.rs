use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_std::coord_common::{RowLength, RowPitch};
use rawspeed_std_ndslice::array2dref::Array2DRef;

use crate::packed_encoder::{ExtraPadding, NumBytes, Packer};

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
