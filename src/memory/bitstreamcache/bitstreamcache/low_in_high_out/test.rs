use super::*;
use rawspeed_common::common::Bitwidth;

#[test]
fn bitstreamcache_constructable_test() {
    type T = BitStreamCacheLowInHighOut;
    let _cache = T::new();
}

#[test]
fn bitstreamcache_push_test() {
    type T = BitStreamCacheLowInHighOut;
    for num_bits in 0usize..T::SIZE {
        let mut cache = T::new();
        assert_eq!(cache.fill_level(), 0);
        cache.push(0, num_bits);
        assert_eq!(cache.fill_level(), num_bits);
    }
}

#[test]
#[should_panic(
    expected = "assertion failed: count + (self.fill_level as usize) <= Self::SIZE"
)]
fn bitstreamcache_push_overflow_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, T::SIZE);
    assert_eq!(cache.fill_level(), T::SIZE);
    cache.push(0, 1);
    unreachable!();
}

#[test]
fn bitstreamcache_double_push_test() {
    type T = BitStreamCacheLowInHighOut;
    for first_bits in 0usize..T::SIZE {
        for second_bits in 0usize..T::SIZE {
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
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.skip(0);
    assert_eq!(cache.fill_level(), 0);
}

#[test]
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_non_zero_skip_of_empty_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.skip(1);
    unreachable!();
}

#[test]
fn bitstreamcache_non_zero_skip_of_nonempty_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.skip(1);
    assert_eq!(cache.fill_level(), 0);
}

#[test]
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_skip_overflow_test() {
    type T = BitStreamCacheLowInHighOut;
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
    type T = BitStreamCacheLowInHighOut;
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
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_skip_overflow_after_refill_test() {
    type T = BitStreamCacheLowInHighOut;
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
    type T = BitStreamCacheLowInHighOut;
    let cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.peek(0);
    unreachable!();
}

#[test]
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_non_zero_peek_of_empty_test() {
    type T = BitStreamCacheLowInHighOut;
    let cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.peek(1);
    unreachable!();
}

#[test]
#[should_panic(expected = "assertion failed: count != 0")]
fn bitstreamcache_zero_peek_of_non_empty_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(0);
    unreachable!();
}

#[test]
fn bitstreamcache_non_zero_peek_of_non_empty_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(1);
    assert_eq!(cache.fill_level(), 1);
}

#[test]
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_peek_overflow_test() {
    type T = BitStreamCacheLowInHighOut;
    let mut cache = T::new();
    assert_eq!(cache.fill_level(), 0);
    cache.push(0, 1);
    assert_eq!(cache.fill_level(), 1);
    cache.peek(2);
    unreachable!();
}

#[test]
fn bitstreamcache_peek_after_refill_test() {
    type T = BitStreamCacheLowInHighOut;
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
#[should_panic(
    expected = "assertion failed: count <= self.fill_level as usize"
)]
fn bitstreamcache_peek_overflow_after_refill_test() {
    type T = BitStreamCacheLowInHighOut;
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

#[test]
fn bitstreamcache_test() {
    type T = u16;
    let mut cache = BitStreamCacheLowInHighOut::new();
    for _repeats in 0..16 {
        for bits in T::MIN..T::MAX {
            assert_eq!(cache.fill_level(), 0);
            cache.push(bits as u64, T::BITWIDTH);
            assert_eq!(cache.fill_level(), T::BITWIDTH);
            assert_eq!(bits as usize, cache.peek(T::BITWIDTH) as usize);
            let mut bits_reconstucted: T = 0;
            for i in 0..T::BITWIDTH {
                bits_reconstucted <<= 1;
                assert_eq!(cache.fill_level(), T::BITWIDTH - i);
                bits_reconstucted |= cache.peek(1) as T;
                assert_eq!(cache.fill_level(), T::BITWIDTH - i);
                cache.skip(1);
                assert_eq!(cache.fill_level(), T::BITWIDTH - i - 1);
            }
            assert_eq!(bits_reconstucted, bits);
        }
    }
}
