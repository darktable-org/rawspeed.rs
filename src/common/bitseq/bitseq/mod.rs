use rawspeed_common_generic_num::generic_num::common::{ActiveBits, Bitwidth};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitLen {
    val: u32,
}

impl BitLen {
    #[must_use]
    #[inline]
    pub const fn new(val: u32) -> Self {
        Self { val }
    }
}

impl core::ops::Deref for BitLen {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

pub trait BitSeqConstraints: Clone + Copy + Bitwidth + ActiveBits {}
impl<T: Clone + Copy + Bitwidth + ActiveBits> BitSeqConstraints for T {}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BitSeq<T>
where
    T: BitSeqConstraints,
{
    storage: T,
    len: BitLen,
}

impl<T> BitSeq<T>
where
    T: BitSeqConstraints,
{
    #[must_use]
    #[inline]
    pub fn new(len: BitLen, value: T) -> Option<Self> {
        assert!(*len <= T::BITWIDTH);
        if value.active_bits() > *len {
            return None;
        }

        Some(Self {
            storage: value,
            len,
        })
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> BitLen {
        assert!(*self.len <= T::BITWIDTH);
        self.len
    }

    #[inline]
    #[must_use]
    pub const fn zext(self) -> T {
        self.storage
    }
}

#[cfg(test)]
mod tests;
