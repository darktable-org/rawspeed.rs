use rawspeed_common_generic_num::generic_num::{
    arith::{BorrowingSub, CarryingAdd, CheckedAdd, CheckedRem, WrappingAdd},
    common::{CastUnsigned, ConstOne, ConstZero, Integer, Max},
};

use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SumAndCarry<T>((T, bool));

struct SumWithZeroCarry<T>(T);

impl<T> core::ops::Deref for SumWithZeroCarry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct HasNonZeroCarry;

impl<T> TryFrom<SumAndCarry<T>> for SumWithZeroCarry<T> {
    type Error = HasNonZeroCarry;

    fn try_from(value: SumAndCarry<T>) -> Result<Self, Self::Error> {
        let (lhs, carry) = (value.0.0, value.0.1);
        if carry {
            return Err(HasNonZeroCarry);
        }
        Ok(Self(lhs))
    }
}

impl<T> core::ops::Sub<T> for SumAndCarry<T>
where
    T: core::fmt::Debug
        + Copy
        + PartialOrd
        + ConstZero
        + From<bool>
        + BorrowingSub<Output = (T, bool)>,
{
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        let (lhs, carry) = (self.0.0, self.0.1);

        let (mut lhs_hi, mut lhs_lo): (T, T) = (carry.into(), lhs);
        let (rhs_hi, rhs_lo): (T, T) = (T::ZERO, rhs);

        let mut borrow = false;
        (lhs_lo, borrow) = lhs_lo.borrowing_sub(rhs_lo, borrow);
        (lhs_hi, borrow) = lhs_hi.borrowing_sub(rhs_hi, borrow);
        assert!(!borrow);
        assert_eq!(lhs_hi, T::ZERO);
        lhs_lo
    }
}

#[expect(clippy::missing_trait_methods)]
impl<T> PartialOrd<SumAndCarry<T>> for SumAndCarry<T>
where
    T: PartialEq<T> + PartialOrd<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.0.1.partial_cmp(&other.0.1) {
            None => unreachable!(),
            Some(core::cmp::Ordering::Equal) => {}
            Some(ord) => return Some(ord),
        }
        self.0.0.partial_cmp(&other.0.0)
    }
}

impl<T> core::ops::Rem<T> for SumAndCarry<T>
where
    Self: core::ops::Sub<T, Output = T> + PartialOrd<SumAndCarry<T>>,
    T: core::fmt::Debug
        + Copy
        + PartialOrd
        + ConstZero
        + ConstOne
        + From<bool>
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + Max
        + CheckedRem<Output = Option<T>>,
    SumWithZeroCarry<T>: TryFrom<SumAndCarry<T>>,
    <SumWithZeroCarry<T> as TryFrom<SumAndCarry<T>>>::Error: core::fmt::Debug,
{
    type Output = T;

    fn rem(self, rhs: T) -> Self::Output {
        if self < SumAndCarry((rhs, false)) {
            let lhs = SumWithZeroCarry::try_from(self).unwrap();
            return lhs.0;
        }
        self - rhs
    }
}

trait NormalizeRHS: CastUnsigned {
    fn normalize(
        rhs: Self,
        domain: <Self as CastUnsigned>::Output,
    ) -> <Self as CastUnsigned>::Output;
}

impl<T> NormalizeRHS for T
where
    T: Clone
        + Copy
        + CastUnsigned
        + TryFrom<<T as CastUnsigned>::Output>
        + CheckedRem<Output = Option<T>>
        + CheckedAdd<Output = Option<T>>,
    <T as CastUnsigned>::Output: Clone
        + Copy
        + TryFrom<T>
        + CheckedRem<Output = Option<<T as CastUnsigned>::Output>>
        + WrappingAdd<Output = <T as CastUnsigned>::Output>,
    <<T as CastUnsigned>::Output as TryFrom<T>>::Error: core::fmt::Debug,
{
    fn normalize(
        rhs: Self,
        domain: <Self as CastUnsigned>::Output,
    ) -> <Self as CastUnsigned>::Output {
        if let Ok(rhs) = <T as CastUnsigned>::Output::try_from(rhs) {
            return rhs.checked_rem(domain).unwrap();
        }

        if let Ok(ibound) = domain.try_into() {
            return rhs
                .checked_rem(ibound)
                .unwrap()
                .checked_add(ibound)
                .unwrap()
                .try_into()
                .unwrap();
        }

        rhs.cast_unsigned().wrapping_add(domain)
    }
}

impl<T, I> core::ops::Add<I> for WrappingUnsigned<T>
where
    I: CastUnsigned<Output = T> + NormalizeRHS,
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
    T: CarryingAdd<Output = (T, bool)>,
    SumAndCarry<T>: core::ops::Rem<T, Output = T>,
{
    type Output = Self;

    #[inline]
    #[expect(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: I) -> Self::Output {
        let domain = self.domain();
        let rhs = NormalizeRHS::normalize(rhs, **domain);
        let lhs = **self;
        let sum = SumAndCarry(lhs.carrying_add(rhs, false));
        let rem = sum % **domain;
        WrappingUnsigned::new(BoundUnsigned::new(*domain, rem).unwrap())
    }
}

#[cfg(test)]
mod tests;
