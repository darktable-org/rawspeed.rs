use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitStreamFlow {
    LowInHighOut,
    HighInLowOut,
}

pub trait BitStreamFlowTrait {}

pub trait BitStreamCache {
    fn new() -> Self;

    fn fill_level(&self) -> usize;

    fn push(&mut self, bits: u64, count: usize);
    fn peek(&self, count: usize) -> u64;
    fn skip(&mut self, count: usize);
}

pub struct BitStreamCacheBase<T: BitStreamFlowTrait> {
    // The actual bits stored in the cache
    cache: u64,

    // Bits left in cache
    fill_level: u32,

    _phantom_data: PhantomData<T>,
}

impl<T: BitStreamFlowTrait> BitStreamCacheBase<T> {
    // Width of cache, in bits
    const SIZE: usize = u64::BITWIDTH;

    // How many bits could be requested to be filled
    const MAX_GET_BITS: usize = u32::BITWIDTH;
}

mod high_in_low_out;
mod low_in_high_out;

pub use high_in_low_out::BitStreamCacheHighInLowOut;
pub use low_in_high_out::BitStreamCacheLowInHighOut;

//------------------------------------------------------------------------------

trait Integer {}
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

fn extract_high_bits<
    T: Integer + ConstZero + Bitwidth + std::ops::Shr<usize, Output = T>,
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

fn extract_low_bits<
    T: Integer
        + ConstZero
        + Bitwidth
        + std::ops::Shl<usize, Output = T>
        + std::ops::Shr<usize, Output = T>,
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

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    //--------------------------------------------------------------------------

    #[test]
    fn bitwidth_zero_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    assert_eq!(<$t>::BITWIDTH, <$t>::BITWIDTH);
                )+
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn extract_high_bits_zero_out_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        const NUM_BITS: usize = 0;
                        const ALLZEROS: $t = 0;
                        assert_eq!(ALLZEROS, extract_high_bits(input, NUM_BITS));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_high_bits_allzero_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0usize..<$t>::BITWIDTH {
                        const ALLZEROS: $t = 0;
                        assert_eq!(ALLZEROS, extract_high_bits(ALLZEROS, num_bits));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
    }

    #[test]
    fn extract_high_bits_passthrough_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        const NUM_BITS: usize = <$t>::BITWIDTH;
                        assert_eq!(input, extract_high_bits(input, NUM_BITS));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_high_bits_allones_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0usize..<$t>::BITWIDTH {
                        const ALLONES: $t = <$t>::MAX;
                        let res = extract_high_bits(ALLONES, num_bits);
                        assert_eq!((res.trailing_ones() as usize), num_bits);
                        assert_eq!(
                            (res.leading_zeros() as usize),
                            ((<$t>::BITWIDTH) - num_bits)
                        );
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
    }

    #[test]
    fn extract_high_bits_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        let mut bits = input;
                        let mut input_reconstructed: $t = 0;
                        for _ in 0..<$t>::BITWIDTH {
                            input_reconstructed <<= 1;
                            input_reconstructed |= extract_high_bits(bits, 1) as $t;
                            bits <<= 1;
                        }
                        assert_eq!(input_reconstructed, input);
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_high_bits_test() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Pat {
            input: u8,
            num_bits: usize,
            output: u8,
        }
        let pats = [
            Pat {
                input: 0b11100111u8,
                num_bits: 1,
                output: 0b00000001u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 2,
                output: 0b00000011u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 6,
                output: 0b00111001u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 7,
                output: 0b01110011u8,
            },
        ];
        for p in pats {
            assert_eq!(p.output, extract_high_bits(p.input, p.num_bits));
        }
    }

    #[test]
    #[should_panic(expected = "num_bits <= T::BITWIDTH")]
    fn extract_high_bits_too_many_bits_test() {
        extract_high_bits(0u8, 9);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn extract_low_bits_zero_out_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        const NUM_BITS: usize = 0;
                        const ALLZEROS: $t = 0;
                        assert_eq!(ALLZEROS, extract_low_bits(input, NUM_BITS));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_low_bits_allzero_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0usize..<$t>::BITWIDTH {
                        const ALLZEROS: $t = 0;
                        assert_eq!(ALLZEROS, extract_low_bits(ALLZEROS, num_bits));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
    }

    #[test]
    fn extract_low_bits_passthrough_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        const NUM_BITS: usize = <$t>::BITWIDTH;
                        assert_eq!(input, extract_low_bits(input, NUM_BITS));
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_low_bits_allones_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0usize..<$t>::BITWIDTH {
                        const ALLONES: $t = <$t>::MAX;
                        let res = extract_low_bits(ALLONES, num_bits);
                        assert_eq!((res.trailing_ones() as usize), num_bits);
                        assert_eq!(
                            (res.leading_zeros() as usize),
                            ((<$t>::BITWIDTH) - num_bits)
                        );
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
    }

    #[test]
    fn extract_low_bits_input_test() {
        macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for input in <$t>::MIN..<$t>::MAX {
                        let mut bits = input;
                        let mut input_reconstructed: $t = 0;
                        for i in 0..<$t>::BITWIDTH {
                            input_reconstructed |= (
                                extract_low_bits(bits, 1) as $t << i
                            );
                            bits >>= 1;
                        }
                        assert_eq!(input_reconstructed, input);
                    }
                )+
            };
        }

        test!(u8);
        test!(u16);
        // test!(u32);
        // test!(u64);
    }

    #[test]
    fn extract_low_bits_test() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Pat {
            input: u8,
            num_bits: usize,
            output: u8,
        }
        let pats = [
            Pat {
                input: 0b11100111u8,
                num_bits: 1,
                output: 0b00000001u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 2,
                output: 0b00000011u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 6,
                output: 0b00100111u8,
            },
            Pat {
                input: 0b11100111u8,
                num_bits: 7,
                output: 0b01100111u8,
            },
        ];
        for p in pats {
            assert_eq!(p.output, extract_low_bits(p.input, p.num_bits));
        }
    }

    #[test]
    #[should_panic(expected = "num_bits <= T::BITWIDTH")]
    fn extract_low_bits_too_many_bits_test() {
        extract_low_bits(0u8, 9);
    }
    //--------------------------------------------------------------------------
}
