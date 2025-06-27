use core::marker::PhantomData;
use rawspeed_common::common::Bitwidth as _;

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum BitStreamFlow {
    LowInHighOut,
    HighInLowOut,
}

pub trait BitStreamFlowTrait {}

pub trait BitStreamCache {
    #[must_use]
    fn new() -> Self;

    fn fill_level(&self) -> usize;

    fn push(&mut self, bits: u64, count: usize);
    fn peek(&self, count: usize) -> u64;
    fn skip(&mut self, count: usize);
}

#[derive(Debug, Copy, Clone)]
pub struct BitStreamCacheBase<T: BitStreamFlowTrait> {
    // The actual bits stored in the cache
    cache: u64,

    // Bits left in cache
    fill_level: u32,

    _phantom_data: PhantomData<T>,
}

impl<T: BitStreamFlowTrait> BitStreamCacheBase<T> {
    // Width of cache, in bits
    pub const SIZE: usize = u64::BITWIDTH;

    // How many bits could be requested to be filled
    const MAX_GET_BITS: usize = u32::BITWIDTH;
}

mod high_in_low_out;
mod low_in_high_out;

pub use high_in_low_out::BitStreamCacheHighInLowOut;
pub use low_in_high_out::BitStreamCacheLowInHighOut;
