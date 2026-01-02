use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};
use rawspeed_common_generic_num::generic_num::{
    arith::{BorrowingSub, CarryingAdd, CheckedAdd, CheckedRem, WrappingAdd},
    common::{CastUnsigned, ConstZero, Integer, Max, Min},
};

#[derive(Debug, Clone, Copy)]
struct SumAndCarry<T>((T, bool));

struct SumWithZeroCarry<T>(T);

impl<T> core::ops::Deref for SumWithZeroCarry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        + From<u8>
        + BorrowingSub<Output = (T, bool)>,
{
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        let (lhs, carry) = (self.0.0, self.0.1);
        assert!(carry);

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
}

impl<T> core::ops::Rem<T> for SumAndCarry<T>
where
    Self: core::ops::Sub<T, Output = T>,
    T: core::fmt::Debug
        + Copy
        + PartialOrd
        + From<u8>
        + core::ops::Shr<i32, Output = T>
        + core::ops::Add<Output = T>
        + Max
        + CheckedRem<Output = Option<T>>,
    SumWithZeroCarry<T>: TryFrom<SumAndCarry<T>>,
{
    type Output = T;

    fn rem(self, rhs: T) -> Self::Output {
        if let Ok(lhs) = SumWithZeroCarry::try_from(self) {
            return lhs.checked_rem(rhs).unwrap();
        }
        assert!(rhs >= (T::MAX >> 1) + T::from(2_u8));
        self - rhs
    }
}

trait TypeSignednessCheck {
    fn is_signed() -> bool;
}

impl<T> TypeSignednessCheck for T
where
    T: ConstZero + Min + PartialOrd,
{
    fn is_signed() -> bool {
        T::MIN < T::ZERO
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
        + TypeSignednessCheck
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
        if let Ok(rhs) = rhs.try_into() {
            if <T as TypeSignednessCheck>::is_signed() {
                return rhs;
            }
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
