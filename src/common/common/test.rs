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
                    for num_bits in 0_usize..<$t>::BITWIDTH {
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
#[allow(clippy::cognitive_complexity)]
fn extract_high_bits_allones_input_test() {
    macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0_usize..<$t>::BITWIDTH {
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
                            input_reconstructed |= extract_high_bits(bits, 1);
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
            input: 0b1110_0111_u8,
            num_bits: 1,
            output: 0b0000_0001_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 2,
            output: 0b0000_0011_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 6,
            output: 0b0011_1001_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 7,
            output: 0b0111_0011_u8,
        },
    ];
    for p in pats {
        assert_eq!(p.output, extract_high_bits(p.input, p.num_bits));
    }
}

#[test]
#[should_panic(expected = "num_bits <= T::BITWIDTH")]
fn extract_high_bits_too_many_bits_test() {
    extract_high_bits(0_u8, 9);
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
                    for num_bits in 0_usize..<$t>::BITWIDTH {
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
#[allow(clippy::cognitive_complexity)]
fn extract_low_bits_allones_input_test() {
    macro_rules! test {
            ($($t:ty)+) => {
                $(
                    for num_bits in 0_usize..<$t>::BITWIDTH {
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
                                extract_low_bits(bits, 1) << i
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
            input: 0b1110_0111_u8,
            num_bits: 1,
            output: 0b0000_0001_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 2,
            output: 0b0000_0011_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 6,
            output: 0b0010_0111_u8,
        },
        Pat {
            input: 0b1110_0111_u8,
            num_bits: 7,
            output: 0b0110_0111_u8,
        },
    ];
    for p in pats {
        assert_eq!(p.output, extract_low_bits(p.input, p.num_bits));
    }
}

#[test]
#[should_panic(expected = "num_bits <= T::BITWIDTH")]
fn extract_low_bits_too_many_bits_test() {
    extract_low_bits(0_u8, 9);
}

//--------------------------------------------------------------------------
