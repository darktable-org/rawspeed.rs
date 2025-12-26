use crate::{
    bound_numerics::BoundUnsigned, wrapping_numerics::WrappingUnsigned,
};
use rawspeed_common::{
    common::Integer,
    generic_arith::{WrappingAdd, WrappingRem},
};

impl<T> core::ops::Add<T> for WrappingUnsigned<T>
where
    T: Integer
        + PartialEq
        + PartialOrd
        + Clone
        + Copy
        + WrappingAdd<Output = T>
        + WrappingRem<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self {
        let domain = self.domain();
        let lhs = **self;
        let rhs = rhs.wrapping_rem(**domain);
        let sum = lhs.wrapping_add(rhs);
        let rem = sum.wrapping_rem(**domain);
        WrappingUnsigned::new(BoundUnsigned::new(*domain, rem).unwrap())
    }
}

#[cfg(test)]
mod tests;
