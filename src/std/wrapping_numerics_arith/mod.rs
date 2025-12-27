use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};
use rawspeed_common::{
    common::{Integer, Max},
    generic_arith::{BorrowingSub, CarryingAdd, CheckedRem},
};

#[inline]
#[must_use]
fn rem_of_wide_lhs<T>((lhs, carry): (T, bool), rhs: T) -> T
where
    T: Integer
        + Max
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + PartialEq
        + core::fmt::Debug
        + PartialOrd
        + Copy
        + From<u8>
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

impl<T> core::ops::Add<T> for WrappingUnsigned<T>
where
    T: Integer
        + Max
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + Clone
        + From<u8>
        + Copy
        + CarryingAdd<Output = (T, bool)>
        + BorrowingSub<Output = (T, bool)>
        + CheckedRem<Output = Option<T>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let domain = self.domain();
        let lhs = **self;
        let rhs = rhs.checked_rem(**domain).unwrap();
        let (sum, carry) = lhs.carrying_add(rhs, false);
        let rem = rem_of_wide_lhs((sum, carry), **domain);
        WrappingUnsigned::new(BoundUnsigned::new(*domain, rem).unwrap())
    }
}

#[cfg(test)]
mod tests;
