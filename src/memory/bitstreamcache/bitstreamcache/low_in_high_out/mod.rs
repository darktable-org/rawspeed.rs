use super::{
    BitStreamCache, BitStreamCacheBase, BitStreamFlowTrait, PhantomData,
};

use rawspeed_common::common::extract_high_bits;

#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
#[derive(Debug, Copy, Clone)]
pub struct BitStreamFlowLowInHighOut;

impl BitStreamFlowTrait for BitStreamFlowLowInHighOut {}

pub type BitStreamCacheLowInHighOut =
    BitStreamCacheBase<BitStreamFlowLowInHighOut>;

impl BitStreamCache for BitStreamCacheLowInHighOut {
    #[inline]
    fn new() -> Self {
        Self {
            cache: 0,
            fill_level: 0,
            _phantom_data: PhantomData,
        }
    }

    #[inline]
    fn fill_level(&self) -> usize {
        self.fill_level as usize
    }

    #[inline]
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
        u32::try_from(count).unwrap();
        self.fill_level += u32::try_from(count).unwrap();
    }

    #[inline]
    fn peek(&self, count: usize) -> u64 {
        assert!(count <= Self::SIZE);
        assert!(count <= Self::MAX_GET_BITS);
        assert!(count != 0);
        assert!(count <= self.fill_level as usize);
        extract_high_bits(self.cache, count)
    }

    #[inline]
    fn skip(&mut self, count: usize) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level as usize);
        u32::try_from(count).unwrap();
        self.fill_level -= u32::try_from(count).unwrap();
        self.cache <<= count;
    }
}

impl Default for BitStreamCacheLowInHighOut {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod test;
