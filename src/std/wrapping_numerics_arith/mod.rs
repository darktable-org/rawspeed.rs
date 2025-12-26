use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};
use rawspeed_common::{
    common::Integer,
    generic_arith::{CheckedAdd, CheckedRem},
};

impl<T> core::ops::Add<T> for WrappingUnsigned<T>
where
    T: Integer
        + PartialEq
        + PartialOrd
        + Clone
        + Copy
        + CheckedAdd<Output = Option<T>>
        + CheckedRem<Output = Option<T>>,
{
    type Output = Option<Self>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let domain = self.domain();
        let lhs = **self;
        let rhs = rhs.checked_rem(**domain).unwrap();
        let sum = lhs.checked_add(rhs)?;
        let rem = sum.checked_rem(**domain).unwrap();
        Some(WrappingUnsigned::new(
            BoundUnsigned::new(*domain, rem).unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests;
