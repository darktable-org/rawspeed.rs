use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};
use rawspeed_common::{
    common::{CastUnsigned, ConstZero, Integer, Max, Min},
    generic_arith::{
        BorrowingSub, CarryingAdd, CheckedAdd, CheckedRem, WrappingAdd,
    },
};

#[inline]
#[must_use]
fn rem_of_wide_lhs<T>((lhs, carry): (T, bool), rhs: T) -> T
where
    T: core::fmt::Debug
        + Copy
        + PartialOrd
        + From<u8>
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + Integer
        + Max
        + CheckedRem<Output = Option<T>>
        + BorrowingSub<Output = (T, bool)>,
{
    if !carry {
        return lhs.checked_rem(rhs).unwrap();
    }

    assert!(rhs >= (T::MAX >> 1) + T::from(2_u8));
    let lhs_hi = u8::from(carry);
    let (mut lhs_hi, mut lhs_lo): (T, T) = (lhs_hi.into(), lhs);
    let (rhs_hi, rhs_lo): (T, T) = (0.into(), rhs);

    let mut borrow = false;
    (lhs_lo, borrow) = lhs_lo.borrowing_sub(rhs_lo, borrow);
    assert!(borrow);
    (lhs_hi, borrow) = lhs_hi.borrowing_sub(rhs_hi, borrow);
    assert!(!borrow);
    assert_eq!(lhs_hi, 0.into());
    assert!(lhs_lo < rhs);
    lhs_lo
}

#[inline]
#[must_use]
fn is_signed_type<T>() -> bool
where
    T: ConstZero + Min + PartialOrd,
{
    T::MIN < T::ZERO
}

#[inline]
#[must_use]
fn normalize_rhs<T, I>(rhs: I, domain: &crate::bound_numerics::Bound<T>) -> T
where
    T: Copy
        + PartialOrd
        + TryFrom<I>
        + Integer
        + CheckedRem<Output = Option<T>>
        + WrappingAdd<Output = T>,
    <T as TryFrom<I>>::Error: core::fmt::Debug,
    I: Copy
        + PartialOrd
        + ConstZero
        + Min
        + TryFrom<T>
        + CheckedRem<Output = Option<I>>
        + CheckedAdd<Output = Option<I>>
        + CastUnsigned<Output = T>,
{
    if let Ok(rhs) = rhs.try_into() {
        if is_signed_type::<I>() {
            return rhs;
        }
        return rhs.checked_rem(**domain).unwrap();
    }

    if let Ok(ibound) = (**domain).try_into() {
        return rhs
            .checked_rem(ibound)
            .unwrap()
            .checked_add(ibound)
            .unwrap()
            .try_into()
            .unwrap();
    }

    rhs.cast_unsigned().wrapping_add(**domain)
}

impl<T, I> core::ops::Add<I> for WrappingUnsigned<T>
where
    T: core::fmt::Debug
        + Copy
        + PartialOrd
        + From<u8>
        + TryFrom<I>
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + Integer
        + Max
        + CarryingAdd<Output = (T, bool)>
        + BorrowingSub<Output = (T, bool)>
        + CheckedRem<Output = Option<T>>
        + WrappingAdd<Output = T>,
    <T as TryFrom<I>>::Error: core::fmt::Debug,
    I: Copy
        + PartialOrd
        + ConstZero
        + Min
        + TryFrom<T>
        + CheckedRem<Output = Option<I>>
        + CheckedAdd<Output = Option<I>>
        + CastUnsigned<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: I) -> Self::Output {
        let domain = self.domain();
        let rhs = normalize_rhs(rhs, domain);
        let lhs = **self;
        let (sum, carry) = lhs.carrying_add(rhs, false);
        let rem = rem_of_wide_lhs((sum, carry), **domain);
        WrappingUnsigned::new(BoundUnsigned::new(*domain, rem).unwrap())
    }
}

#[cfg(test)]
mod tests;
