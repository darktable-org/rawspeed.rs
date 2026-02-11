use super::*;
use rawspeed_common_generic_num::generic_num::common::Bitwidth as _;

#[test]
fn bitstreamcache_constructable_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let _cache = T::new();
}

#[test]
fn bitstreamcache_push_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    for num_bits in 0_u32..T::SIZE {
        let mut cache = T::new();
        assert_eq!(cache.fill_level(), 0);
        cache.push(0, num_bits);
        assert_eq!(cache.fill_level(), num_bits);
    }
}

#[test]
#[should_panic(
    expected = "assertion failed: count + self.fill_level <= Self::SIZE"
)]
fn bitstreamcache_push_overflow_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, T::SIZE);
    assert_eq!(cache.fill_level(), T::SIZE);
    cache.push(0, 1);
    unreachable!();
}

#[test]
fn bitstreamcache_double_push_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    for first_bits in 0_u32..T::SIZE {
        for second_bits in 0_u32..T::SIZE {
            if first_bits + second_bits <= T::SIZE {
                let mut cache = T::new();
                assert_eq!(cache.fill_level(), 0);
                cache.push(0, first_bits);
                assert_eq!(cache.fill_level(), first_bits);
                cache.push(0, second_bits);
                assert_eq!(cache.fill_level(), first_bits + second_bits);
            }
        }
    }
}

#[test]
fn bitstreamcache_zero_skip_of_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.skip(0);
    assert_eq!(cache.fill_level(), 0);
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_non_zero_skip_of_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.skip(1);
    unreachable!();
}

#[test]
fn bitstreamcache_non_zero_skip_of_nonempty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_skip_overflow_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
    cache.skip(1);
    unreachable!();
}

#[test]
fn bitstreamcache_skip_after_refill_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_skip_overflow_after_refill_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
    cache.skip(1);
    unreachable!()
}

#[test]
#[should_panic(expected = "assertion failed: count != 0")]
fn bitstreamcache_zero_peek_of_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.peek(0);
    unreachable!();
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_non_zero_peek_of_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.peek(1);
    unreachable!();
}

#[test]
#[should_panic(expected = "assertion failed: count != 0")]
fn bitstreamcache_zero_peek_of_non_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(0);
    unreachable!();
}

#[test]
fn bitstreamcache_non_zero_peek_of_non_empty_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(1);
    assert_eq!(cache.fill_level(), 1);
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_peek_overflow_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(2);
    unreachable!();
}

#[test]
fn bitstreamcache_peek_after_refill_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(1);
    assert_eq!(cache.fill_level(), 1);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 2);
    cache.peek(2);
    assert_eq!(cache.fill_level(), 2);
}

#[test]
#[should_panic(expected = "assertion failed: count <= self.fill_level")]
fn bitstreamcache_peek_overflow_after_refill_test() {
    type T = BitStreamCacheHighInLowOut<u8>;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(1);
    assert_eq!(cache.fill_level(), 1);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 2);
    cache.peek(3);
    unreachable!();
}

fn bitstreamcache_test<Storage>()
where
    Storage: BitStreamCacheData,
    u8: TryFrom<Storage>,
    <u8 as TryFrom<Storage>>::Error: core::fmt::Debug,
{
    type CacheType<Storage> = BitStreamCacheHighInLowOut<Storage>;
    type T = u8;
    let mut cache: BitStreamCacheHighInLowOut<Storage> =
        CacheType::<Storage>::new();
    for bits in T::MIN..T::MAX {
        assert_eq!(cache.fill_level(), 0);
        cache.push(bits.into(), T::BITWIDTH);
        assert_eq!(cache.fill_level(), T::BITWIDTH);
        assert_eq!(
            T::from(bits),
            T::try_from(cache.peek(T::BITWIDTH)).unwrap()
        );
        let mut bits_reconstucted: T = 0;
        for i in 0..T::BITWIDTH {
            assert_eq!(cache.fill_level(), T::BITWIDTH - i);
            bits_reconstucted |= T::try_from(cache.peek(1)).unwrap() << i;
            assert_eq!(cache.fill_level(), T::BITWIDTH - i);
            cache.skip(1);
            assert_eq!(cache.fill_level(), T::BITWIDTH - i - 1);
        }
        assert_eq!(bits_reconstucted, bits);
    }
}

#[test]
fn bitstreamcache_test_u8() {
    bitstreamcache_test::<u8>();
}

#[test]
fn bitstreamcache_test_u16() {
    bitstreamcache_test::<u16>();
}

#[test]
fn bitstreamcache_test_u32() {
    bitstreamcache_test::<u32>();
}

#[test]
fn bitstreamcache_test_u64() {
    bitstreamcache_test::<u64>();
}
