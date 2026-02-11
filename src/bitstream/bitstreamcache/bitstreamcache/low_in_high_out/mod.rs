use rawspeed_common_bit_manip::bit_manip::extract_high_bits;

use super::BitStreamCache;
use super::BitStreamCacheBase;
use super::BitStreamCacheData;
use super::BitStreamFlowTrait;
use super::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct BitStreamFlowLowInHighOut;

impl BitStreamFlowTrait for BitStreamFlowLowInHighOut {}

pub type BitStreamCacheLowInHighOut<T = u64> =
    BitStreamCacheBase<BitStreamFlowLowInHighOut, T>;

impl<T: BitStreamCacheData> BitStreamCache for BitStreamCacheLowInHighOut<T> {
    type Storage = T;

    #[inline]
    fn new() -> Self {
        Self {
            cache: 0.into(),
            fill_level: 0,
            _phantom_data: PhantomData,
        }
    }

    #[inline]
    fn size(&self) -> u32 {
        Self::SIZE
    }

    #[inline]
    fn fill_level(&self) -> u32 {
        self.fill_level
    }

    #[inline]
    fn push(&mut self, bits: Self::Storage, count: u32) {
        // NOTE: `count`` may be zero!
        assert!(count <= Self::SIZE);
        assert!(count + self.fill_level <= Self::SIZE);
        // If the maximal size of the cache is BitStreamCacheBase::Size, and we
        // have fillLevel [high] bits set, how many empty [low] bits do we have?
        let vacant_bits = Self::SIZE - self.fill_level;
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
        self.fill_level += count;
    }

    #[inline]
    fn peek(&self, count: u32) -> Self::Storage {
        assert!(count <= Self::SIZE);
        assert!(count != 0);
        assert!(count <= self.fill_level);
        extract_high_bits(self.cache, count)
    }

    #[inline]
    fn skip(&mut self, count: u32) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level);
        self.fill_level -= count;
        self.cache <<= count;
    }
}

impl<T: BitStreamCacheData> Default for BitStreamCacheLowInHighOut<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
