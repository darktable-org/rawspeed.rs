use rawspeed_common_bit_manip::bit_manip::extract_low_bits;

use super::BitStreamCache;
use super::BitStreamCacheBase;
use super::BitStreamCacheData;
use super::BitStreamFlowTrait;
use super::PhantomData;

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
        self.cache |= bits << self.fill_level();
        self.fill_level += count;
    }

    #[inline]
    fn peek(&self, count: u32) -> Self::Storage {
        assert!(count <= Self::SIZE);
        assert!(count != 0);
        assert!(count <= self.fill_level);
        extract_low_bits(self.cache, count)
    }

    #[inline]
    fn skip(&mut self, count: u32) {
        // `count` *could* be larger than `MaxGetBits`.
        // `count` could be zero.
        assert!(count <= Self::SIZE);
        assert!(count <= self.fill_level);
        self.fill_level -= count;
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
mod tests;
