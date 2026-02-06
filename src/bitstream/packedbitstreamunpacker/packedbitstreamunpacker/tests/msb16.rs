use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;
use rawspeed_bitstream_packedbitstreamslice::packedbitstreamslice::PackedBitstreamSlice;

use crate::packedbitstreamunpacker::{
    PackedBitstreamUnpacker, PackedBitstreamUnpackerError,
    PackedBitstreamUnpackerWrongSizeError,
};

type T = BitOrderMSB16;

#[test]
fn sufficient_for_1_packed_mcu_test() {
    let input: [_; 6] = core::array::from_fn(|_| 0);
    let s = BitStreamSlice::<T>::new(&input).unwrap();
    let s = PackedBitstreamSlice::<T, 3>::new(s).unwrap();
    PackedBitstreamUnpacker::new(s).unwrap();
}

#[test]
fn sufficient_for_2_packed_mcu_test() {
    let input: [_; 12] = core::array::from_fn(|_| 0);
    let s = BitStreamSlice::<T>::new(&input).unwrap();
    let s = PackedBitstreamSlice::<T, 3>::new(s).unwrap();
    assert_eq!(
        PackedBitstreamUnpacker::new(s).err(),
        Some(PackedBitstreamUnpackerError::WrongSize(
            PackedBitstreamUnpackerWrongSizeError {
                actual: 12,
                expected: 6
            }
        ))
    );
}

#[test]
fn u1_test() {
    test!(unpack [0b1010_1010, 0b1010_1010] as 1-bits: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]    );
}
#[test]
fn u2_test() {
    test!(unpack [0b0110_1100, 0b0110_1100] as 2-bits: [1, 2, 3, 0, 1, 2, 3, 0]    );
}
#[test]
fn u3_test() {
    test!(unpack [0b1100_1011, 0b0010_1001, 0b0010_1001, 0b1011_1000, 0b1011_1000, 0b1100_1011] as 3-bits: [1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0]    );
}
#[test]
fn u4_test() {
    test!(unpack [0b0011_0100, 0b0001_0010] as 4-bits: [1, 2, 3, 4]    );
}
#[test]
fn u5_test() {
    test!(unpack [0b1000_0110, 0b0000_1000, 0b1001_1000, 0b0100_0010, 0b0100_1010, 0b1110_1000, 0b1100_0110, 0b1001_0110, 0b1111_0000, 0b1011_1001] as 5-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u6_test() {
    test!(unpack [0b0010_0000, 0b0000_0100, 0b0001_0100, 0b1100_0100, 0b1100_1000, 0b0110_0001] as 6-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u7_test() {
    test!(unpack [0b0000_1000, 0b0000_0010, 0b0100_0000, 0b0001_1000, 0b1000_0011, 0b1010_0001, 0b0001_0010, 0b1000_1000, 0b0101_1000, 0b0010_1000, 0b1010_0011, 0b1100_0001, 0b1001_0000, 0b1000_0111] as 7-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u8_test() {
    test!(unpack [0b0000_0010, 0b0000_0001] as 8-bits: [1, 2]    );
}
#[test]
fn u9_test() {
    test!(unpack [0b1000_0000, 0b0000_0000, 0b0110_0000, 0b1000_0000, 0b0010_1000, 0b0100_0000, 0b0000_1110, 0b0001_1000, 0b0000_0100, 0b0000_1000, 0b1000_0001, 0b1000_0010, 0b1100_0000, 0b0110_0000, 0b0011_1000, 0b0110_1000, 0b0001_0000, 0b0001_1110] as 9-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u10_test() {
    test!(unpack [0b0100_0000, 0b0000_0000, 0b0000_1100, 0b0010_0000, 0b0000_0001, 0b0000_0100, 0b0110_0000, 0b0100_0000, 0b0000_1000, 0b0001_1100] as 10-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u11_test() {
    test!(unpack [0b0010_0000, 0b0000_0000, 0b0000_0001, 0b0000_1000, 0b0100_0000, 0b1000_0000, 0b0000_0001, 0b0000_1010, 0b0011_1000, 0b1000_0000, 0b0000_0001, 0b0000_1000, 0b0010_1000, 0b0010_0000, 0b1000_0000, 0b0000_0101, 0b0001_1010, 0b1100_0000, 0b1000_0000, 0b0000_0011, 0b0001_0000, 0b0111_1000] as 11-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u12_test() {
    test!(unpack [0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0010, 0b0000_0100, 0b0011_0000] as 12-bits: [1, 2, 3, 4]    );
}
#[test]
fn u13_test() {
    test!(unpack [0b0000_1000, 0b0000_0000, 0b1000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0110, 0b0000_0010, 0b0100_0000, 0b0001_1000, 0b1000_0000, 0b1110_0000, 0b0000_0000, 0b0000_0000, 0b0000_1000, 0b0000_0010, 0b0100_1000, 0b0001_0110, 0b1000_0000, 0b1100_0000, 0b0000_0000, 0b1000_0000, 0b0000_0110, 0b0000_0001, 0b0011_1000, 0b0001_0000, 0b1110_0000] as 13-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u14_test() {
    test!(unpack [0b0000_0100, 0b0000_0000, 0b0010_0000, 0b0000_0000, 0b1100_0000, 0b0000_0000, 0b0000_0000, 0b0000_0100, 0b0000_0000, 0b0001_0100, 0b0000_0001, 0b0110_0000, 0b0000_1000, 0b1100_0000] as 14-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u15_test() {
    test!(unpack [0b0000_0010, 0b0000_0000, 0b0000_1000, 0b0000_0000, 0b0001_1000, 0b0000_0000, 0b0100_0000, 0b0000_0000, 0b1010_0000, 0b0000_0000, 0b1000_0000, 0b0000_0001, 0b1000_0000, 0b0000_0011, 0b0000_0000, 0b0000_1000, 0b0000_0000, 0b0001_0010, 0b0000_0000, 0b0010_1000, 0b0000_0000, 0b0101_1000, 0b0000_0001, 0b1100_0000, 0b0000_0011, 0b1010_0000, 0b0000_0111, 0b1000_0000, 0b0001_0000, 0b1000_0000] as 15-bits: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]    );
}
#[test]
fn u16_test() {
    test!(unpack [0b0000_0001, 0b0000_0000] as 16-bits: [1]    );
}
