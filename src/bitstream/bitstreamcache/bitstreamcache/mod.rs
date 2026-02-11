use core::marker::PhantomData;

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
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
    + core::ops::ShlAssign<u32>
    + core::ops::ShrAssign<u32>
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

    fn size(&self) -> u32;
    fn fill_level(&self) -> u32;

    fn push(&mut self, bits: Self::Storage, count: u32);
    fn peek(&self, count: u32) -> Self::Storage;
    fn skip(&mut self, count: u32);
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
    pub const SIZE: u32 = T::BITWIDTH;
}

mod high_in_low_out;
mod low_in_high_out;

pub use high_in_low_out::BitStreamCacheHighInLowOut;
pub use low_in_high_out::BitStreamCacheLowInHighOut;
use rawspeed_common_generic_num::generic_num::common::{Bitwidth, Integer};
