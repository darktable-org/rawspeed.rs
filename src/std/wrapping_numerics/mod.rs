use rawspeed_common_generic_num::generic_num::common::Integer;

use crate::bound_numerics::BoundUnsigned;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    value: BoundUnsigned<T>,
}

impl<T> WrappingUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    #[inline]
    #[must_use]
    pub const fn new(value: BoundUnsigned<T>) -> Self {
        Self { value }
    }

    #[inline]
    #[must_use]
    pub const fn value(&self) -> &BoundUnsigned<T> {
        &self.value
    }
}

impl<T> core::ops::Deref for WrappingUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    type Target = BoundUnsigned<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

#[cfg(test)]
mod tests;
