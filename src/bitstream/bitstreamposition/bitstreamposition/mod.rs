use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSliceConstraints;

#[derive(Debug)]
#[non_exhaustive]
pub struct BitstreamPosition<T>
where
    T: BitStreamSliceConstraints,
{
    mcu_index: usize,
    bit_index: u32,
    _phantom_data: core::marker::PhantomData<T>,
}

impl<T> BitstreamPosition<T>
where
    T: BitStreamSliceConstraints,
{
    #[must_use]
    #[inline]
    pub const fn new(mcu_index: usize, bit_index: u32) -> Self {
        Self {
            mcu_index,
            bit_index,
            _phantom_data: core::marker::PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub const fn mcu_index(&self) -> usize {
        self.mcu_index
    }

    #[must_use]
    #[inline]
    pub const fn bit_index(&self) -> u32 {
        self.bit_index
    }
}

#[cfg(test)]
mod tests;
