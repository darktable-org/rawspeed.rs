//------------------------------------------------------------------------------

pub trait Integer {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}

pub trait Bitwidth {
    const BITWIDTH: usize;
}

macro_rules! impl_bitwidth {
    ($($t:ty)+) => {
        $(
            impl Bitwidth for $t {
                const BITWIDTH: usize = <$t>::BITS as usize;
            }
        )+
    };
}

impl_bitwidth!(u8 u16 u32 u64);

pub trait ConstZero {
    const ZERO: Self;
}

impl ConstZero for u8 {
    const ZERO: Self = 0;
}
impl ConstZero for u16 {
    const ZERO: Self = 0;
}
impl ConstZero for u32 {
    const ZERO: Self = 0;
}
impl ConstZero for u64 {
    const ZERO: Self = 0;
}

//------------------------------------------------------------------------------

#[inline]
pub fn extract_high_bits<
    T: Integer + ConstZero + Bitwidth + core::ops::Shr<usize, Output = T>,
>(
    value: T,
    num_bits: usize,
) -> T {
    if num_bits == 0 {
        return <T>::ZERO;
    }
    assert!(num_bits <= T::BITWIDTH);
    let num_low_bits_to_skip = T::BITWIDTH - num_bits;
    assert!(num_low_bits_to_skip < T::BITWIDTH);
    value >> num_low_bits_to_skip
}

#[inline]
pub fn extract_low_bits<
    T: Integer
        + ConstZero
        + Bitwidth
        + core::ops::Shl<usize, Output = T>
        + core::ops::Shr<usize, Output = T>,
>(
    value: T,
    num_bits: usize,
) -> T {
    if num_bits == 0 {
        return <T>::ZERO;
    }
    assert!(num_bits <= T::BITWIDTH);
    let num_high_padding_bits = T::BITWIDTH - num_bits;
    assert!(num_high_padding_bits < T::BITWIDTH);
    (value << num_high_padding_bits) >> num_high_padding_bits
}

//------------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod test;
