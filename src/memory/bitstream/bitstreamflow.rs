use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitStreamFlow {
    LowInHighOut,
    HighInLowOut,
}

pub trait BitStreamFlowTrait {}

pub struct BitStreamCacheBase<T: BitStreamFlowTrait> {
    // The actual bits stored in the cache
    cache: u64,

    // Bits left in cache
    fill_level: u32,

    _phantom_data: PhantomData<T>,
}

impl<T: BitStreamFlowTrait> BitStreamCacheBase<T> {
    // Width of cache, in bits
    const SIZE: usize = u64::BITWIDTH as usize;

    // How many bits could be requested to be filled
    const MAX_GET_BITS: usize = u32::BITWIDTH as usize;

    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            cache: 0,
            fill_level: 0,
            _phantom_data: PhantomData,
        }
    }
}

pub trait BitStreamCache {
    fn push(&mut self, bits: u64, count: usize);
    fn peek(&self, count: usize) -> u64;
    fn skip(&mut self, count: usize);
}

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

pub struct BitStreamFlowLowInHighOut;

impl BitStreamFlowTrait for BitStreamFlowLowInHighOut {}

pub type BitStreamCacheLowInHighOut =
    BitStreamCacheBase<BitStreamFlowLowInHighOut>;

impl BitStreamCache for BitStreamCacheLowInHighOut {
    fn push(&mut self, bits: u64, count: usize) {
        // NOTE: `count`` may be zero!
        assert!(count <= Self::SIZE);
        assert!(count + (self.fill_level as usize) <= Self::SIZE);
        // If the maximal size of the cache is BitStreamCacheBase::Size, and we
        // have fillLevel [high] bits set, how many empty [low] bits do we have?
        let vacant_bits = Self::SIZE - self.fill_level as usize;
        assert!(vacant_bits <= Self::SIZE);
        assert!(vacant_bits != 0);
        assert!(vacant_bits >= count);
        // If we just directly 'or' these low bits into the cache right now,
        // how many unfilled bits of a gap will there be
        // in the middle of a cache?
        let empty_bits_gap = vacant_bits - count;
        assert!(empty_bits_gap <= Self::SIZE);
        if count != 0 {
            assert!(empty_bits_gap < Self::SIZE);
            // So just shift the new bits so that there is no such gap.
            self.cache |= bits << empty_bits_gap;
        }
        assert!(count <= u32::MAX as usize);
        self.fill_level += count as u32;
    }
    fn peek(&self, count: usize) -> u64 {
        assert!(count <= Self::SIZE);
        assert!(count <= Self::MAX_GET_BITS);
        assert!(count != 0);
        assert!(count <= self.fill_level as usize);
        extract_high_bits(self.cache, count)
    }
    fn skip(&mut self, count: usize) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level as usize);
        assert!(count <= u32::MAX as usize);
        self.fill_level -= count as u32;
        self.cache <<= count;
    }
}

//------------------------------------------------------------------------------

pub struct BitStreamFlowHighInLowOut;

impl BitStreamFlowTrait for BitStreamFlowHighInLowOut {}

pub type BitStreamCacheHighInLowOut =
    BitStreamCacheBase<BitStreamFlowHighInLowOut>;

impl BitStreamCache for BitStreamCacheHighInLowOut {
    fn push(&mut self, bits: u64, count: usize) {
        // NOTE: `count`` may be zero!
        assert!(count <= Self::SIZE);
        assert!(count + (self.fill_level as usize) <= Self::SIZE);
        self.cache |= bits << self.fill_level;
        assert!(count <= u32::MAX as usize);
        self.fill_level += count as u32;
    }
    fn peek(&self, count: usize) -> u64 {
        assert!(count <= Self::SIZE);
        assert!(count <= Self::MAX_GET_BITS);
        assert!(count != 0);
        assert!(count <= self.fill_level as usize);
        extract_low_bits(self.cache, count)
    }
    fn skip(&mut self, count: usize) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level as usize);
        assert!(count <= u32::MAX as usize);
        self.fill_level -= count as u32;
        self.cache >>= count;
    }
}

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
                    assert_eq!(<$t>::BITWIDTH as usize, <$t>::BITWIDTH);
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
                    for num_bits in 0usize..<$t>::BITWIDTH as usize {
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
                        const NUM_BITS: usize = <$t>::BITWIDTH as usize;
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
                    for num_bits in 0usize..<$t>::BITWIDTH as usize {
                        const ALLONES: $t = <$t>::MAX;
                        let res = extract_high_bits(ALLONES, num_bits);
                        assert_eq!((res.trailing_ones() as usize), num_bits);
                        assert_eq!(
                            (res.leading_zeros() as usize),
                            ((<$t>::BITWIDTH as usize) - num_bits)
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
                    for num_bits in 0usize..<$t>::BITWIDTH as usize {
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
                        const NUM_BITS: usize = <$t>::BITWIDTH as usize;
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
                    for num_bits in 0usize..<$t>::BITWIDTH as usize {
                        const ALLONES: $t = <$t>::MAX;
                        let res = extract_low_bits(ALLONES, num_bits);
                        assert_eq!((res.trailing_ones() as usize), num_bits);
                        assert_eq!(
                            (res.leading_zeros() as usize),
                            ((<$t>::BITWIDTH as usize) - num_bits)
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
    #[cfg(test)]
    mod low_in_high_out {
        use super::*;

        #[test]
        fn bitstreamcache_constructable_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let _cache = T::new();
        }

        #[test]
        fn bitstreamcache_push_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            for num_bits in 0usize..T::SIZE {
                let mut cache = T::new();
                cache.push(0, num_bits);
            }
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count + (self.fill_level as usize) <= Self::SIZE"
        )]
        fn bitstreamcache_push_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, T::SIZE);
            cache.push(0, 1);
        }

        #[test]
        fn bitstreamcache_double_push_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            for first_bits in 0usize..T::SIZE {
                for second_bits in 0usize..T::SIZE {
                    if first_bits + second_bits <= T::SIZE {
                        let mut cache = T::new();
                        cache.push(0, first_bits);
                        cache.push(0, second_bits);
                    }
                }
            }
        }

        #[test]
        fn bitstreamcache_zero_skip_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.skip(0);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_non_zero_skip_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.skip(1);
        }

        #[test]
        fn bitstreamcache_non_zero_skip_of_nonempty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_skip_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.skip(1);
            cache.skip(1);
        }

        #[test]
        fn bitstreamcache_skip_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.skip(1);
            cache.push(0, 1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_skip_overflow_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.skip(1);
            cache.push(0, 1);
            cache.skip(1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: count != 0")]
        fn bitstreamcache_zero_peek_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let cache = T::new();
            cache.peek(0);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_non_zero_peek_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let cache = T::new();
            cache.peek(1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: count != 0")]
        fn bitstreamcache_zero_peek_of_non_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(0);
        }

        #[test]
        fn bitstreamcache_non_zero_peek_of_non_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_peek_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.peek(2);
        }

        #[test]
        fn bitstreamcache_peek_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(1);
            cache.push(0, 1);
            cache.peek(2);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_peek_overflow_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowLowInHighOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.peek(1);
            cache.push(0, 1);
            cache.peek(3);
        }

        #[test]
        fn bitstreamcache_test() {
            type T = u16;
            let mut cache =
                BitStreamCacheBase::<BitStreamFlowLowInHighOut>::new();
            for _repeats in 0..16 {
                for bits in T::MIN..T::MAX {
                    cache.push(bits as u64, T::BITWIDTH as usize);
                    assert_eq!(
                        bits as usize,
                        cache.peek(T::BITWIDTH as usize) as usize
                    );
                    let mut bits_reconstucted: T = 0;
                    for _ in 0..T::BITWIDTH {
                        bits_reconstucted <<= 1;
                        bits_reconstucted |= cache.peek(1) as T;
                        cache.skip(1);
                    }
                    assert_eq!(bits_reconstucted, bits);
                }
            }
        }
    }
    //--------------------------------------------------------------------------

    //--------------------------------------------------------------------------

    #[cfg(test)]
    mod high_in_low_out {
        use super::*;

        #[test]
        fn bitstreamcache_constructable_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let _cache = T::new();
        }

        #[test]
        fn bitstreamcache_push_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            for num_bits in 0usize..T::SIZE {
                let mut cache = T::new();
                cache.push(0, num_bits);
            }
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count + (self.fill_level as usize) <= Self::SIZE"
        )]
        fn bitstreamcache_push_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, T::SIZE);
            cache.push(0, 1);
        }

        #[test]
        fn bitstreamcache_double_push_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            for first_bits in 0usize..T::SIZE {
                for second_bits in 0usize..T::SIZE {
                    if first_bits + second_bits <= T::SIZE {
                        let mut cache = T::new();
                        cache.push(0, first_bits);
                        cache.push(0, second_bits);
                    }
                }
            }
        }

        #[test]
        fn bitstreamcache_zero_skip_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.skip(0);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_non_zero_skip_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.skip(1);
        }

        #[test]
        fn bitstreamcache_non_zero_skip_of_nonempty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_skip_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.skip(1);
            cache.skip(1);
        }

        #[test]
        fn bitstreamcache_skip_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.skip(1);
            cache.push(0, 1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_skip_overflow_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.skip(1);
            cache.push(0, 1);
            cache.skip(1);
            cache.skip(1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: count != 0")]
        fn bitstreamcache_zero_peek_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let cache = T::new();
            cache.peek(0);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_non_zero_peek_of_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let cache = T::new();
            cache.peek(1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: count != 0")]
        fn bitstreamcache_zero_peek_of_non_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(0);
        }

        #[test]
        fn bitstreamcache_non_zero_peek_of_non_empty_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(1);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_peek_overflow_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.peek(2);
        }

        #[test]
        fn bitstreamcache_peek_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            cache.push(0, 1);
            cache.peek(1);
            cache.push(0, 1);
            cache.peek(2);
        }

        #[test]
        #[should_panic(
            expected = "assertion failed: count <= self.fill_level as usize"
        )]
        fn bitstreamcache_peek_overflow_after_refill_test() {
            type T = BitStreamCacheBase<BitStreamFlowHighInLowOut>;
            let mut cache = T::new();
            assert_eq!(cache.fill_level, 0);
            cache.push(0, 1);
            cache.peek(1);
            cache.push(0, 1);
            cache.peek(3);
        }

        #[test]
        fn bitstreamcache_test() {
            type T = u16;
            let mut cache =
                BitStreamCacheBase::<BitStreamFlowHighInLowOut>::new();
            for _repeats in 0..16 {
                for bits in T::MIN..T::MAX {
                    cache.push(bits as u64, T::BITWIDTH as usize);
                    assert_eq!(
                        bits as usize,
                        cache.peek(T::BITWIDTH as usize) as usize
                    );
                    let mut bits_reconstucted: T = 0;
                    for i in 0..T::BITWIDTH {
                        bits_reconstucted |= (cache.peek(1) as T) << i;
                        cache.skip(1);
                    }
                    assert_eq!(bits_reconstucted, bits);
                }
            }
        }
    }

    //--------------------------------------------------------------------------
}
