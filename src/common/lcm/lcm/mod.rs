use rawspeed_common_generic_num::generic_num::common::Max;

#[must_use]
#[inline]
fn maxmin<T>(v1: T, v2: T) -> (T, T)
where
    T: Clone + Copy + Ord,
{
    (core::cmp::max(v1, v2), core::cmp::min(v1, v2))
}

#[allow(clippy::allow_attributes)]
#[allow(dead_code)]
#[inline]
fn multiples_of<T>(v: T) -> impl Iterator<Item = T>
where
    T: Clone + Copy + Max + TryFrom<usize>,
    usize: From<T>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
{
    (usize::from(v)..=usize::from(T::MAX))
        .step_by(usize::from(v))
        .map(|v| T::try_from(v).unwrap())
}

mod naive {
    use super::{maxmin, multiples_of};
    use rawspeed_common_generic_num::generic_num::common::{
        ConstZero, IsMultipleOf, Max,
    };

    #[cfg_attr(not(test), expect(dead_code))]
    #[expect(clippy::upper_case_acronyms)]
    pub trait LCM {
        type Output;
        #[must_use]
        fn lcm(a: Self, b: Self) -> Option<Self::Output>;
    }

    impl<T> LCM for T
    where
        T: Clone + Copy + ConstZero + Ord + Max + IsMultipleOf + TryFrom<usize>,
        usize: From<T>,
        <T as TryFrom<usize>>::Error: core::fmt::Debug,
    {
        type Output = Self;
        #[inline]
        fn lcm(a: Self, b: Self) -> Option<Self> {
            let (a, b) = maxmin(a, b);
            if b == T::ZERO {
                return Some(T::ZERO);
            }
            multiples_of(a).find(|d| d.is_multiple_of(b))
        }
    }
}

mod intersect {
    use super::{maxmin, multiples_of};
    use rawspeed_common_generic_num::generic_num::common::{ConstZero, Max};

    #[cfg_attr(not(test), expect(dead_code))]
    #[expect(clippy::upper_case_acronyms)]
    pub trait LCM {
        type Output;
        #[must_use]
        fn lcm(a: Self, b: Self) -> Option<Self::Output>;
    }

    impl<T> LCM for T
    where
        T: Clone + Copy + ConstZero + Ord + Max + TryFrom<usize>,
        usize: From<T> + From<u8>,
        <T as TryFrom<usize>>::Error: core::fmt::Debug,
    {
        type Output = Self;
        #[inline]
        fn lcm(a: Self, b: Self) -> Option<Self> {
            let (a, b) = maxmin(a, b);
            if b == T::ZERO {
                return Some(T::ZERO);
            }

            let mut a_multiples = multiples_of(a);
            let mut b_multiples = multiples_of(b);

            let mut a_multiple = a_multiples.next();
            let mut b_multiple = b_multiples.next();
            while let Some(a_val) = a_multiple
                && let Some(b_val) = b_multiple
            {
                match a_val.cmp(&b_val) {
                    core::cmp::Ordering::Less => {
                        a_multiple = a_multiples.next();
                    }
                    core::cmp::Ordering::Greater => {
                        b_multiple = b_multiples.next();
                    }
                    core::cmp::Ordering::Equal => return Some(a_val),
                }
            }
            None
        }
    }
}

mod via_gcd {
    use super::maxmin;
    use rawspeed_common_exact_ops::exact_ops::div::CheckedDivExact;
    use rawspeed_common_gcd::gcd::GCD;
    use rawspeed_common_generic_num::generic_num::{
        arith::CheckedMul, common::ConstZero,
    };

    pub trait LCM {
        type Output;
        #[must_use]
        fn lcm(a: Self, b: Self) -> Option<Self::Output>;
    }

    impl<T> LCM for T
    where
        T: Clone
            + Copy
            + ConstZero
            + Ord
            + GCD
            + CheckedDivExact<Output = Option<T>>
            + CheckedMul<Output = Option<T>>,
    {
        type Output = Self;
        #[inline]
        fn lcm(a: Self, b: Self) -> Option<Self> {
            let (a, b) = maxmin(a, b);
            if b == T::ZERO {
                return Some(T::ZERO);
            }
            let gcd = <T as GCD>::gcd(a, b);
            let a_steps = CheckedDivExact::checked_div_exact(b, gcd).unwrap();
            a.checked_mul(a_steps)
        }
    }
}

pub mod constant {
    macro_rules! _lcm {
        ($a:expr, $b:expr) => {{
            let (a, b) = ($a, $b);
            let (a, b) = { if a > b { (a, b) } else { (b, a) } };
            if b == 0 {
                Some(0)
            } else {
                let mut i = 1;
                loop {
                    let Some(d) = a.checked_mul(i) else {
                        break None;
                    };
                    if d.is_multiple_of(b) {
                        break Some(d);
                    }
                    i += 1;
                }
            }
        }};
    }
    #[cfg_attr(not(test), expect(unused_imports))]
    pub(crate) use _lcm as lcm;
}

pub use via_gcd::LCM;

#[cfg(test)]
mod tests;
