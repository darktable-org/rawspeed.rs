use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;

use crate::packedbitstreamslice::{
    PackedBitstreamSlice, PackedBitstreamSliceError,
    PackedBitstreamSliceWrongSizeError,
};

type T = BitOrderMSB16;

#[test]
fn unsufficient_test() {
    let input: [_; 4] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    assert_eq!(
        PackedBitstreamSlice::<_, 3>::new(bss).err(),
        Some(PackedBitstreamSliceError::WrongSize(
            PackedBitstreamSliceWrongSizeError {
                actual_len: 4,
                expected_multiplicity: 6
            }
        ))
    );
}

#[test]
fn too_much_test() {
    let input: [_; 8] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    assert_eq!(
        PackedBitstreamSlice::<_, 3>::new(bss).err(),
        Some(PackedBitstreamSliceError::WrongSize(
            PackedBitstreamSliceWrongSizeError {
                actual_len: 8,
                expected_multiplicity: 6
            }
        ))
    );
}

#[test]
fn sufficient_for_1_packed_mcu_test() {
    let input: [_; 6] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    let r = PackedBitstreamSlice::<_, 6>::new(bss);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().get_slice().get_bytes().len(), 6);
}

#[test]
fn sufficient_for_2_packed_mcu_test() {
    let input: [_; 12] = core::array::from_fn(|_| 0);
    let bss = BitStreamSlice::<T>::new(&input).unwrap();
    let r = PackedBitstreamSlice::<_, 12>::new(bss);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().get_slice().get_bytes().len(), 12);
}
