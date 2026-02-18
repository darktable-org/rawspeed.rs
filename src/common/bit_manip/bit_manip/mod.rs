use rawspeed_common_generic_num::generic_num::common::{
    Bitwidth, ConstZero, Integer,
};

#[inline]
pub fn extract_high_bits<T: Integer + core::ops::Shr<u32, Output = T>>(
    value: T,
    num_bits: u32,
) -> T {
    if num_bits == 0 {
        return <T>::ZERO;
    }
    assert!(num_bits <= T::BITWIDTH);
    let num_low_bits_to_skip = T::BITWIDTH - num_bits;
    assert!(num_low_bits_to_skip < T::BITWIDTH);
    value >> num_low_bits_to_skip
}

pub trait ExtractLowBitsConstraints:
    Bitwidth
    + ConstZero
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
{
}
impl<T> ExtractLowBitsConstraints for T where
    T: Bitwidth
        + ConstZero
        + core::ops::Shl<u32, Output = Self>
        + core::ops::Shr<u32, Output = Self>
{
}

#[inline]
pub fn extract_low_bits<T>(value: T, num_bits: u32) -> T
where
    T: ExtractLowBitsConstraints,
{
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
