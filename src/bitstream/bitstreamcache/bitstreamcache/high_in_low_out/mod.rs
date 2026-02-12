use rawspeed_common_bit_manip::bit_manip::extract_low_bits;
use rawspeed_common_bitseq::bitseq::{BitLen, BitSeq, BitSeqConstraints};

use crate::bitstreamcache::{
    BitStreamCache, BitStreamCacheBase, BitStreamCacheData, BitStreamFlowTrait,
};

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
            _phantom_data: core::marker::PhantomData,
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
    fn push(&mut self, bits: BitSeq<Self::Storage>)
    where
        <Self as BitStreamCache>::Storage: BitSeqConstraints,
    {
        // NOTE: `count`` may be zero!
        assert!(*bits.len() + self.fill_level <= Self::SIZE);
        self.cache |= bits.zext() << self.fill_level();
        self.fill_level += *bits.len();
    }

    #[inline]
    fn peek(&self, count: u32) -> BitSeq<Self::Storage> {
        assert!(count <= Self::SIZE);
        assert!(count != 0);
        assert!(count <= self.fill_level);
        let bits = extract_low_bits(self.cache, count);
        BitSeq::new(BitLen::new(count), bits).unwrap()
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
