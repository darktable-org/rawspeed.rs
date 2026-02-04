use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;

use crate::bitstreamslice::{
    BitStreamSlice, BitStreamSliceError, MCUIndex, MCUIndexByteOverflow,
    MCURange,
};

type T = BitOrderMSB16;

#[test]
fn empty_test() {
    let input: [u8; _] = [];
    assert_eq!(
        BitStreamSlice::<T>::new(&input),
        Err(BitStreamSliceError::InputIsEmpty)
    );
}

#[test]
fn less_than_one_mcu_test() {
    let input: [u8; _] = [0];
    assert_eq!(
        BitStreamSlice::<T>::new(&input),
        Err(BitStreamSliceError::InputIsTruncated)
    );
}

#[test]
fn less_than_two_mcu_test() {
    let input: [u8; _] = [0, 0, 0];
    assert_eq!(
        BitStreamSlice::<T>::new(&input),
        Err(BitStreamSliceError::InputIsTruncated)
    );
}

#[test]
fn one_mcu_test() {
    let input: [u8; _] = [1, 2];
    let slice = BitStreamSlice::<T>::new(&input).unwrap();
    assert_eq!(slice.mcu_count(), 1);
    assert_eq!(slice.get_bytes(), input);
}

#[cfg(test)]
mod range;

#[cfg(test)]
mod range_inclusive;

#[cfg(test)]
mod range_from;
