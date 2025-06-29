use super::BitStreamCache;
use super::BitStreamCacheBase;
use super::BitStreamCacheData;
use super::BitStreamFlowTrait;
use super::PhantomData;

use rawspeed_common::common::extract_low_bits;

#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
#[derive(Debug, Copy, Clone)]
pub struct BitStreamFlowHighInLowOut;

impl BitStreamFlowTrait for BitStreamFlowHighInLowOut {}

pub type BitStreamCacheHighInLowOut<T = u64> =
    BitStreamCacheBase<BitStreamFlowHighInLowOut, T>;

impl<T: BitStreamCacheData> BitStreamCache for BitStreamCacheHighInLowOut<T> {
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
    fn fill_level(&self) -> usize {
        self.fill_level as usize
    }

    #[inline]
    fn push(&mut self, bits: Self::Storage, count: usize) {
        // NOTE: `count`` may be zero!
        assert!(count <= Self::SIZE);
        assert!(count + (self.fill_level as usize) <= Self::SIZE);
        self.cache |= bits << self.fill_level();
        self.fill_level += u32::try_from(count).unwrap();
    }

    #[inline]
    fn peek(&self, count: usize) -> Self::Storage {
        assert!(count <= Self::SIZE);
        assert!(count != 0);
        assert!(count <= self.fill_level as usize);
        extract_low_bits(self.cache, count)
    }

    #[inline]
    fn skip(&mut self, count: usize) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level as usize);
        self.fill_level -= u32::try_from(count).unwrap();
        self.cache >>= count;
    }
}

impl<T: BitStreamCacheData> Default for BitStreamCacheHighInLowOut<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod tests;
