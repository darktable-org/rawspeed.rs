use super::*;

use rawspeed_common::common::extract_low_bits;

pub struct BitStreamFlowHighInLowOut;

impl BitStreamFlowTrait for BitStreamFlowHighInLowOut {}

pub type BitStreamCacheHighInLowOut =
    BitStreamCacheBase<BitStreamFlowHighInLowOut>;

impl BitStreamCache for BitStreamCacheHighInLowOut {
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
        self.cache |= bits << self.fill_level;
        assert!(u32::try_from(count).is_ok());
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
        assert!(u32::try_from(count).is_ok());
        self.fill_level -= count as u32;
        self.cache >>= count;
    }
}

impl Default for BitStreamCacheHighInLowOut {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test;
