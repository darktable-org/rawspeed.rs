use rawspeed_common_generic_num::generic_num::common::{ConstZero, Integer};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct Bound<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    domain: T,
}

impl<T> Bound<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    #[inline]
    #[must_use]
    pub fn new(domain: T) -> Option<Self> {
        if domain <= ConstZero::ZERO {
            return None;
        }
        Some(Self { domain })
    }

    #[inline]
    #[must_use]
    pub fn domain(&self) -> &T {
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::hint::assert_unchecked(self.domain > ConstZero::ZERO);
        }
        &self.domain
    }
}

impl<T> core::ops::Deref for Bound<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.domain()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct BoundUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    domain: Bound<T>,
    value: T,
}

impl<T> BoundUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    #[inline]
    #[must_use]
    pub fn new(domain: Bound<T>, value: T) -> Option<Self> {
        if value >= *domain {
            return None;
        }
        Some(Self { domain, value })
    }

    #[inline]
    #[must_use]
    pub fn value(&self) -> &T {
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::hint::assert_unchecked(self.value < *self.domain);
        }
        &self.value
    }

    #[inline]
    pub const fn domain(&self) -> &Bound<T> {
        &self.domain
    }
}

impl<T> core::ops::Deref for BoundUnsigned<T>
where
    T: Integer + PartialEq + PartialOrd + Clone + Copy,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

#[cfg(test)]
mod tests;
