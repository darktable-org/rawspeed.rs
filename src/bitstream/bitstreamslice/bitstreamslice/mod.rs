use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};

#[derive(Debug, Clone, Copy)]
struct ByteIndex {
    index: usize,
}

impl ByteIndex {
    #[must_use]
    #[inline]
    const fn new(index: usize) -> Self {
        Self { index }
    }
}

impl core::ops::Deref for ByteIndex {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MCUIndex<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    index: usize,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> MCUIndex<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    #[must_use]
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            _phantom: core::marker::PhantomData,
        }
    }

    #[must_use]
    #[inline]
    pub const fn val(&self) -> usize {
        self.index
    }
}

impl<T> From<usize> for MCUIndex<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    #[inline]
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct MCUIndexByteOverflow;

impl<T> TryFrom<MCUIndex<T>> for ByteIndex
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Error = MCUIndexByteOverflow;

    #[inline]
    fn try_from(value: MCUIndex<T>) -> Result<Self, Self::Error> {
        let mcu_size = size_of::<<T as BitStreamTraits>::MCUByteArrayType>();
        value
            .val()
            .checked_mul(mcu_size)
            .ok_or(MCUIndexByteOverflow)
            .map(Self::new)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MCURange<T, R>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
    R: core::ops::RangeBounds<MCUIndex<T>>,
{
    range: R,
    _phantom: core::marker::PhantomData<T>,
}

impl<T, R> MCURange<T, R>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
    R: core::ops::RangeBounds<MCUIndex<T>>,
{
    #[must_use]
    #[inline]
    const fn new(range: R) -> Self {
        Self {
            range,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T, R> From<R> for MCURange<T, R>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
    R: core::ops::RangeBounds<MCUIndex<T>>,
{
    #[inline]
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

pub trait ConvertibleIntoByteRange<T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Output;
}

impl<T> ConvertibleIntoByteRange<T> for core::ops::Range<MCUIndex<T>>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Output = core::ops::Range<usize>;
}

impl<T> TryFrom<MCURange<T, core::ops::Range<MCUIndex<T>>>>
    for core::ops::Range<usize>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Error = MCUIndexByteOverflow;

    #[inline]
    fn try_from(
        range: MCURange<T, core::ops::Range<MCUIndex<T>>>,
    ) -> Result<Self, Self::Error> {
        let range = range.range;
        let start = *ByteIndex::try_from(range.start)?;
        let end = *ByteIndex::try_from(range.end)?;
        Ok(core::ops::Range { start, end })
    }
}

impl<T> ConvertibleIntoByteRange<T> for core::ops::RangeInclusive<MCUIndex<T>>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Output = core::ops::Range<usize>;
}

impl<T> TryFrom<MCURange<T, core::ops::RangeInclusive<MCUIndex<T>>>>
    for core::ops::Range<usize>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
    core::ops::Range<usize>: TryFrom<
            MCURange<T, core::ops::Range<MCUIndex<T>>>,
            Error = MCUIndexByteOverflow,
        >,
{
    type Error = MCUIndexByteOverflow;

    #[inline]
    fn try_from(
        range: MCURange<T, core::ops::RangeInclusive<MCUIndex<T>>>,
    ) -> Result<Self, Self::Error> {
        let range = range.range;
        let start = *range.start();
        let end = range
            .end()
            .val()
            .checked_add(1)
            .ok_or(MCUIndexByteOverflow)?;
        MCURange::from(core::ops::Range {
            start,
            end: end.into(),
        })
        .try_into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum BitStreamSliceError {
    InputIsEmpty,
    InputIsTruncated,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitStreamSlice<'a, T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    input: &'a [u8],
    _phantom: core::marker::PhantomData<T>,
}

impl<'a, T> BitStreamSlice<'a, T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    const MCU_SIZE: usize =
        size_of::<<T as BitStreamTraits>::MCUByteArrayType>();

    #[inline]
    pub const fn new(input: &'a [u8]) -> Result<Self, BitStreamSliceError> {
        if input.is_empty() {
            return Err(BitStreamSliceError::InputIsEmpty);
        }
        if !input.len().is_multiple_of(Self::MCU_SIZE) {
            return Err(BitStreamSliceError::InputIsTruncated);
        }
        Ok(Self {
            input,
            _phantom: core::marker::PhantomData,
        })
    }

    #[must_use]
    #[inline]
    pub const fn mcu_count(&self) -> usize {
        self.input.len().checked_div(Self::MCU_SIZE).unwrap()
    }

    #[must_use]
    #[inline]
    pub const fn get_bytes(&self) -> &'a [u8] {
        self.input
    }

    #[must_use]
    #[inline]
    pub fn get<R, I>(&self, index: R) -> Option<Self>
    where
        R: core::ops::RangeBounds<MCUIndex<T>>
            + ConvertibleIntoByteRange<T, Output = I>
            + Into<MCURange<T, R>>,
        MCURange<T, R>: TryInto<I>,
        I: core::ops::RangeBounds<usize>
            + core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        let index = index.into();
        let index = index.try_into().ok()?;
        let bytes = self.input.get(index)?;
        Some(Self::new(bytes).unwrap())
    }
}

impl<'a, T> TryFrom<&'a [u8]> for BitStreamSlice<'a, T>
where
    T: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    type Error = BitStreamSliceError;

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests;
