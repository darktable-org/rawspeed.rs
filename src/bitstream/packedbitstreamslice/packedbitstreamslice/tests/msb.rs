use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;

use crate::packedbitstreamslice::{
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
                actual_len: 2,
                expected_multiplicity: 3
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
                actual_len: 4,
                expected_multiplicity: 3
            }
        ))
    );
}

#[test]
fn sufficient_for_1_packed_mcu_test() {
    let input: [_; 3] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    let r = PackedBitstreamSlice::<_, 3>::new(bss);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().get_slice().get_bytes().len(), 3);
}

#[test]
fn sufficient_for_2_packed_mcu_test() {
    let input: [_; 6] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    let r = PackedBitstreamSlice::<_, 3>::new(bss);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().get_slice().get_bytes().len(), 6);
}
