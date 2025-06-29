use core::marker::PhantomData;
use rawspeed_common::common::Bitwidth;
use rawspeed_common::common::Integer;

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum BitStreamFlow {
    LowInHighOut,
    HighInLowOut,
}

pub trait BitStreamFlowTrait {}

pub trait BitStreamCacheData:
    Sized
    + Copy
    + Bitwidth
    + Integer
    + From<u8>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shr<usize, Output = Self>
    + core::ops::ShlAssign<usize>
    + core::ops::ShrAssign<usize>
    + core::ops::BitOrAssign<Self>
{
}

impl BitStreamCacheData for u8 {}
impl BitStreamCacheData for u16 {}
impl BitStreamCacheData for u32 {}
impl BitStreamCacheData for u64 {}

pub trait BitStreamCache {
    type Storage;

    #[must_use]
    fn new() -> Self;

    fn fill_level(&self) -> usize;

    fn push(&mut self, bits: Self::Storage, count: usize);
    fn peek(&self, count: usize) -> Self::Storage;
    fn skip(&mut self, count: usize);
}

#[derive(Debug, Copy, Clone)]
pub struct BitStreamCacheBase<
    F: BitStreamFlowTrait,
    T: BitStreamCacheData = u64,
> {
    // The actual bits stored in the cache
    cache: T,

    // Bits left in cache
    fill_level: u32,

    _phantom_data: PhantomData<F>,
}

impl<F: BitStreamFlowTrait, T: BitStreamCacheData> BitStreamCacheBase<F, T> {
    // Width of cache, in bits
    pub const SIZE: usize = T::BITWIDTH;
}

mod high_in_low_out;
mod low_in_high_out;

pub use high_in_low_out::BitStreamCacheHighInLowOut;
pub use low_in_high_out::BitStreamCacheLowInHighOut;
