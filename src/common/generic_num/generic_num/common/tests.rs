use super::*;

#[test]
#[expect(clippy::assertions_on_constants)]
#[expect(clippy::absurd_extreme_comparisons)]
fn usize_fits_into_u64() {
    assert!(usize::BITS <= u64::BITS);
    let usize_max: Result<u64, _> = usize::MAX.try_into();
    usize_max.unwrap();
    assert!(usize_max.unwrap() <= u64::MAX);
}

//--------------------------------------------------------------------------

#[test]
fn bitwidth_zero_test() {
    macro_rules! test {
            ($($t:ty)+) => {
                $(
                    assert_eq!(<$t>::BITWIDTH, <$t>::BITWIDTH);
                )+
            };
        }

    test!(u8);
    test!(u16);
    test!(u32);
    test!(u64);
}

//--------------------------------------------------------------------------
