use rawspeed_common_generic_num::generic_num::common::Integer;

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
