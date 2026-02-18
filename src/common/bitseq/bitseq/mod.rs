use rawspeed_common_bit_manip::bit_manip::{
    ExtractLowBitsConstraints, extract_low_bits,
};
use rawspeed_common_generic_num::generic_num::common::Bitwidth;

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

pub trait BitSeqConstraints:
    Clone + Copy + Bitwidth + PartialEq + ExtractLowBitsConstraints
{
}
impl<T> BitSeqConstraints for T where
    T: Clone + Copy + Bitwidth + PartialEq + ExtractLowBitsConstraints
{
}

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
    fn new_unchecked(len: BitLen, value: T) -> Self {
        assert!(*len <= T::BITWIDTH);
        Self {
            storage: value,
            len,
        }
    }

    #[must_use]
    #[inline]
    pub fn new(len: BitLen, value: T) -> Option<Self> {
        let val = Self::new_unchecked(BitLen::new(T::BITWIDTH), value)
            .trunc_or_self(len);

        if val.zext() != value {
            return None;
        }

        Some(val)
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

    #[inline]
    #[must_use]
    fn trunc(self, new_len: BitLen) -> Self {
        assert!(*new_len <= T::BITWIDTH);
        assert!(*new_len < *self.len());
        let bits = extract_low_bits(self.zext(), *new_len);
        Self::new_unchecked(new_len, bits)
    }

    #[inline]
    #[must_use]
    fn trunc_or_self(self, new_len: BitLen) -> Self {
        assert!(*new_len <= T::BITWIDTH);
        assert!(*new_len <= *self.len());
        if *new_len < *self.len() {
            self.trunc(new_len)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests;
