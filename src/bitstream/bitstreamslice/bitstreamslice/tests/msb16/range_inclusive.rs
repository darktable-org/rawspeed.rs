use super::*;

#[test]
fn index_test() {
    let index = MCUIndex::<T>::new(1)..=MCUIndex::<T>::new(2);
    let index_usize = MCURange::from(index).try_into();
    assert_eq!(index_usize, Ok(2..6));
}

#[test]
fn index_almost_ov_begin_test() {
    let index = MCUIndex::<T>::new(usize::MAX / 2)
        ..=MCUIndex::<T>::new((usize::MAX / 2) - 1);
    let index_usize = MCURange::from(index).try_into();
    assert_eq!(index_usize, Ok((usize::MAX - 1)..(usize::MAX - 1)));
}

#[test]
fn index_ov_begin_test() {
    let index = MCUIndex::<T>::new(1 + (usize::MAX / 2))
        ..=MCUIndex::<T>::new((usize::MAX / 2) - 1);
    let index_usize: Result<core::ops::Range<_>, MCUIndexByteOverflow> =
        MCURange::from(index).try_into();
    assert_eq!(index_usize, Err(MCUIndexByteOverflow));
}

#[test]
fn index_almost_ov_end_test() {
    let index =
        MCUIndex::<T>::new(1)..=MCUIndex::<T>::new((usize::MAX / 2) - 1);
    let index_usize = MCURange::from(index).try_into();
    assert_eq!(index_usize, Ok(2..(usize::MAX - 1)));
}

#[test]
fn index_ov_end_in_inc_test() {
    let index = MCUIndex::<T>::new(1)..=MCUIndex::<T>::new(usize::MAX);
    let index_usize: Result<core::ops::Range<_>, MCUIndexByteOverflow> =
        MCURange::from(index).try_into();
    assert_eq!(index_usize, Err(MCUIndexByteOverflow));
}

#[test]
fn index_ov_end_in_scale_test() {
    let index = MCUIndex::<T>::new(1)..=MCUIndex::<T>::new(usize::MAX / 2);
    let index_usize: Result<core::ops::Range<_>, MCUIndexByteOverflow> =
        MCURange::from(index).try_into();
    assert_eq!(index_usize, Err(MCUIndexByteOverflow));
}

macro_rules! test {
        ($($range:tt[$lb:literal .. $ub:literal] == $res:tt,)+) => {
            $(
                let input: [u8; _] = $range;
                let expected_output: [u8; _] = $res;
                let slice = BitStreamSlice::<T>::new(&input).unwrap();
                let expected_subslice = BitStreamSlice::<T>::new(&expected_output).unwrap();
                let index = MCUIndex::<T>::new($lb)..=MCUIndex::<T>::new($ub-1);
                let new_slice = slice.get(index);
                assert_eq!(new_slice, Some(expected_subslice));
            )+
        };
    }

#[test]
#[expect(clippy::cognitive_complexity)]
fn slice_test() {
    test!(
        //
        [1, 2][0..1] == [1, 2],
        //
        [1, 2, 3, 4][0..2] == [1, 2, 3, 4],
        [1, 2, 3, 4][0..1] == [1, 2],
        [1, 2, 3, 4][1..2] == [3, 4],
        //
        [1, 2, 3, 4, 5, 6][0..3] == [1, 2, 3, 4, 5, 6],
        [1, 2, 3, 4, 5, 6][0..2] == [1, 2, 3, 4],
        [1, 2, 3, 4, 5, 6][1..3] == [3, 4, 5, 6],
        [1, 2, 3, 4, 5, 6][0..1] == [1, 2],
        [1, 2, 3, 4, 5, 6][1..2] == [3, 4],
        [1, 2, 3, 4, 5, 6][2..3] == [5, 6],
    );
}
