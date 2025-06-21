use super::{
    BitStreamCache, BitStreamCacheBase, BitStreamFlowTrait, PhantomData,
};

use rawspeed_common::common::extract_high_bits;

pub struct BitStreamFlowLowInHighOut;

impl BitStreamFlowTrait for BitStreamFlowLowInHighOut {}

pub type BitStreamCacheLowInHighOut =
    BitStreamCacheBase<BitStreamFlowLowInHighOut>;

impl BitStreamCache for BitStreamCacheLowInHighOut {
    fn new() -> Self {
        Self {
            cache: 0,
            fill_level: 0,
            _phantom_data: PhantomData,
        }
    }

    fn fill_level(&self) -> usize {
        self.fill_level as usize
    }

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
        assert!(u32::try_from(count).is_ok());
        self.fill_level += u32::try_from(count).expect("");
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
        assert!(u32::try_from(count).is_ok());
        self.fill_level -= u32::try_from(count).expect("");
        self.cache <<= count;
    }
}

impl Default for BitStreamCacheLowInHighOut {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod test;
