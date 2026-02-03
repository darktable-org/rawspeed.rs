use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;

use crate::packedbitstream::{
    PackedBitstreamSlice, PackedBitstreamSliceError,
    PackedBitstreamSliceWrongSizeError,
};

type T = BitOrderMSB;

#[test]
fn unsufficient_test() {
    let input: [_; 2] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    assert_eq!(
        PackedBitstreamSlice::<_, 3>::new(bss).err(),
        Some(PackedBitstreamSliceError::WrongSize(
            PackedBitstreamSliceWrongSizeError {
                actual: 2,
                expected: 3
            }
        ))
    );
}

#[test]
fn too_much_test() {
    let input: [_; 4] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    assert_eq!(
        PackedBitstreamSlice::<_, 3>::new(bss).err(),
        Some(PackedBitstreamSliceError::WrongSize(
            PackedBitstreamSliceWrongSizeError {
                actual: 4,
                expected: 3
            }
        ))
    );
}

#[test]
fn u1_test() {
    test!(unpack [0b1010_1010] as 1-bits: [1, 0, 1, 0, 1, 0, 1, 0]    );
}
#[test]
fn u2_test() {
    test!(unpack [0b0110_1100] as 2-bits: [1, 2, 3, 0]    );
}
#[test]
fn u3_test() {
    test!(unpack [0b0010_1001, 0b1100_1011, 0b1011_1000] as 3-bits: [1, 2, 3, 4, 5, 6, 7, 0]    );
}
#[test]
fn u4_test() {
    test!(unpack [0b0001_0010] as 4-bits: [1, 2]    );
}
#[test]
fn u5_test() {
    test!(unpack [0b0000_1000, 0b1000_0110, 0b0100_0010, 0b1001_1000, 0b1110_1000] as 5-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u6_test() {
    test!(unpack [0b0000_0100, 0b0010_0000, 0b1100_0100] as 6-bits: [1, 2, 3, 4]    );
}
#[test]
fn u7_test() {
    test!(unpack [0b0000_0010, 0b0000_1000, 0b0001_1000, 0b0100_0000, 0b1010_0001, 0b1000_0011, 0b1000_1000] as 7-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u8_test() {
    test!(unpack [0b0000_0001] as 8-bits: [1]    );
}
#[test]
fn u9_test() {
    test!(unpack [0b0000_0000, 0b1000_0000, 0b1000_0000, 0b0110_0000, 0b0100_0000, 0b0010_1000, 0b0001_1000, 0b0000_1110, 0b0000_1000] as 9-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u10_test() {
    test!(unpack [0b0000_0000, 0b0100_0000, 0b0010_0000, 0b0000_1100, 0b0000_0100] as 10-bits: [1, 2, 3, 4]    );
}
#[test]
fn u11_test() {
    test!(unpack [0b0000_0000, 0b0010_0000, 0b0000_1000, 0b0000_0001, 0b1000_0000, 0b0100_0000, 0b0000_1010, 0b0000_0001, 0b1000_0000, 0b0011_1000, 0b0000_1000] as 11-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u12_test() {
    test!(unpack [0b0000_0000, 0b0001_0000, 0b0000_0010] as 12-bits: [1, 2]    );
}
#[test]
fn u13_test() {
    test!(unpack [0b0000_0000, 0b0000_1000, 0b0000_0000, 0b1000_0000, 0b0000_0110, 0b0000_0000, 0b0100_0000, 0b0000_0010, 0b1000_0000, 0b0001_1000, 0b0000_0000, 0b1110_0000, 0b0000_1000] as 13-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u14_test() {
    test!(unpack [0b0000_0000, 0b0000_0100, 0b0000_0000, 0b0010_0000, 0b0000_0000, 0b1100_0000, 0b0000_0100] as 14-bits: [1, 2, 3, 4]    );
}
#[test]
fn u15_test() {
    test!(unpack [0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_1000, 0b0000_0000, 0b0001_1000, 0b0000_0000, 0b0100_0000, 0b0000_0000, 0b1010_0000, 0b0000_0001, 0b1000_0000, 0b0000_0011, 0b1000_0000, 0b0000_1000] as 15-bits: [1, 2, 3, 4, 5, 6, 7, 8]    );
}
#[test]
fn u16_test() {
    test!(unpack [0b0000_0000, 0b0000_0001] as 16-bits: [1]    );
}
