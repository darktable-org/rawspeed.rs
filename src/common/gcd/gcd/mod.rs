use rawspeed_common_exact_ops::exact_ops::shr::CheckedShrExact;
use rawspeed_common_generic_num::generic_num::common::TrailingZeros;

#[must_use]
#[inline]
fn maxmin<T>(v1: T, v2: T) -> (T, T)
where
    T: Clone + Copy + Ord,
{
    (core::cmp::max(v1, v2), core::cmp::min(v1, v2))
}

#[must_use]
#[inline]
fn decompose_binary<T>(val: T) -> (T, u32)
where
    T: Clone + Copy + TrailingZeros + CheckedShrExact<Output = Option<T>>,
{
    let val_tz = val.trailing_zeros();
    let val = CheckedShrExact::checked_shr_exact(val, val_tz).unwrap();
    (val, val_tz)
}

mod naive {
    use super::maxmin;
    use rawspeed_common_generic_num::generic_num::common::{
        ConstZero, IsMultipleOf,
    };

    #[cfg_attr(not(test), expect(dead_code))]
    #[expect(clippy::upper_case_acronyms)]
    pub trait GCD {
        #[must_use]
        fn gcd(a: Self, b: Self) -> Self;
    }

    impl<T> GCD for T
    where
        T: Clone + Copy + ConstZero + Ord + IsMultipleOf + TryFrom<usize>,
        usize: From<T> + From<u8>,
        <T as TryFrom<usize>>::Error: core::fmt::Debug,
    {
        #[inline]
        fn gcd(a: Self, b: Self) -> Self {
            let (a, b) = maxmin(a, b);
            if b == T::ZERO {
                return a;
            }
            (usize::from(1_u8)..=b.into())
                .rev()
                .map(|d| T::try_from(d).unwrap())
                .find(|d| a.is_multiple_of(*d) && b.is_multiple_of(*d))
                .unwrap()
        }
    }
}

mod sub {
    use super::maxmin;
    use rawspeed_common_generic_num::generic_num::{
        arith::CheckedSub, common::ConstZero,
    };

    #[cfg_attr(not(test), expect(dead_code))]
    #[expect(clippy::upper_case_acronyms)]
    pub trait GCD {
        #[must_use]
        fn gcd(a: Self, b: Self) -> Self;
    }

    impl<T> GCD for T
    where
        T: Clone + Copy + Ord + ConstZero + CheckedSub<Output = Option<T>>,
    {
        #[inline]
        fn gcd(mut a: Self, mut b: Self) -> Self {
            loop {
                (a, b) = maxmin(a, b);
                if b == T::ZERO {
                    return a;
                }
                a = a.checked_sub(b).unwrap();
            }
        }
    }
}

mod euclid {
    use super::maxmin;
    use rawspeed_common_generic_num::generic_num::{
        arith::CheckedRem, common::ConstZero,
    };

    #[cfg_attr(not(test), expect(dead_code))]
    #[expect(clippy::upper_case_acronyms)]
    pub trait GCD {
        #[must_use]
        fn gcd(a: Self, b: Self) -> Self;
    }

    impl<T> GCD for T
    where
        T: Clone + Copy + Ord + ConstZero + CheckedRem<Output = Option<T>>,
    {
        #[inline]
        fn gcd(a: Self, b: Self) -> Self {
            let (mut a, mut b) = maxmin(a, b);
            while b != T::ZERO {
                (a, b) = (b, a.checked_rem(b).unwrap());
            }
            a
        }
    }
}

mod binary {
    use super::{decompose_binary, maxmin};
    use rawspeed_common_exact_ops::exact_ops::{
        shl::CheckedShlExact, shr::CheckedShrExact,
    };
    use rawspeed_common_generic_num::generic_num::{
        arith::{CheckedMul, CheckedSub},
        common::{ConstOne, ConstZero, TrailingZeros},
    };

    pub trait GCD {
        #[must_use]
        fn gcd(a: Self, b: Self) -> Self;
    }

    impl<T> GCD for T
    where
        T: Clone
            + Copy
            + Ord
            + ConstZero
            + ConstOne
            + TrailingZeros
            + CheckedShrExact<Output = Option<T>>
            + CheckedSub<Output = Option<T>>
            + CheckedShlExact<Output = Option<T>>
            + CheckedMul<Output = Option<T>>,
    {
        #[inline]
        fn gcd(a: Self, b: Self) -> Self {
            let (a, b) = maxmin(a, b);
            if b == T::ZERO {
                return a;
            }
            let (mut a, mut b, d) = {
                let (a, a_tz) = decompose_binary(a);
                let (b, b_tz) = decompose_binary(b);
                let d = core::cmp::min(a_tz, b_tz);
                (a, b, d)
            };
            while a != b {
                (a, b) = maxmin(a, b);
                a = a.checked_sub(b).unwrap();
                (a, _) = decompose_binary(a);
            }
            a.checked_mul(
                CheckedShlExact::checked_shl_exact(T::ONE, d).unwrap(),
            )
            .unwrap()
        }
    }
}

pub use binary::GCD;

#[cfg(test)]
mod tests;
