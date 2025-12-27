//------------------------------------------------------------------------------

pub trait Integer: Sized + Bitwidth + ConstZero {}

macro_rules! impl_simple_trait {
    (impl $tr:ident for $($t:ty$(,)?)+) => {
        $(
            impl $tr for $t {}
        )+
    };
}

impl_simple_trait!(impl Integer for u8, u16, u32, u64);

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

pub trait Max {
    const MAX: Self;
}

macro_rules! impl_max {
    ($($t:ty)+) => {
        $(
            impl Max for $t {
                const MAX: $t = <$t>::MAX;
            }
        )+
    };
}

impl_max!(u8 u16 u32 u64);

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
pub fn extract_high_bits<T: Integer + core::ops::Shr<usize, Output = T>>(
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
mod tests;
