use rawspeed_bitstream_bitstreamslice::bitstreamslice::{
    BitStreamSlice, BitStreamSliceConstraints, BitStreamSliceError,
};
use rawspeed_memory_variable_length_load::variable_length_load::VariableLengthLoad as _;

use crate::bitstreambytesequencereader::{
    BitStreamByteSequenceRead, BitStreamByteSequenceReadResult,
    BitStreamByteSequenceRewind,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[must_use]
pub struct BitStreamByteSequenceDefaultReader<'a, T, ByteArray> {
    input: &'a [u8],
    pos: usize,
    _phantom_data: core::marker::PhantomData<T>,
    _phantom_peeked_data: core::marker::PhantomData<ByteArray>,
}

impl<'a, T, ByteArray> BitStreamByteSequenceDefaultReader<'a, T, ByteArray>
where
    T: BitStreamSliceConstraints,
{
    #[inline]
    pub const fn new(input: BitStreamSlice<'a, T>) -> Self {
        Self {
            input: input.get_bytes(),
            pos: 0,
            _phantom_data: core::marker::PhantomData,
            _phantom_peeked_data: core::marker::PhantomData,
        }
    }
}

impl<'a, T, ByteArray> From<BitStreamSlice<'a, T>>
    for BitStreamByteSequenceDefaultReader<'a, T, ByteArray>
where
    T: BitStreamSliceConstraints,
{
    #[inline]
    fn from(input: BitStreamSlice<'a, T>) -> Self {
        Self::new(input)
    }
}

impl<'a, T, ByteArray> TryFrom<&'a [u8]>
    for BitStreamByteSequenceDefaultReader<'a, T, ByteArray>
where
    T: BitStreamSliceConstraints,
    Self: From<BitStreamSlice<'a, T>>,
{
    type Error = BitStreamSliceError;

    #[inline]
    fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
        let input = input.try_into()?;
        Ok(Self::from(input))
    }
}

impl<T, ByteArray> BitStreamByteSequenceReadResult
    for BitStreamByteSequenceDefaultReader<'_, T, ByteArray>
{
    type ByteArray = ByteArray;
    type Error = &'static str;
}

impl<T, ByteArray> BitStreamByteSequenceRead
    for BitStreamByteSequenceDefaultReader<'_, T, ByteArray>
where
    T: BitStreamSliceConstraints,
    for<'b> ByteArray: Default
        + core::ops::IndexMut<core::ops::RangeFull, Output = [u8]>
        + TryFrom<&'b [u8]>,
    for<'b> <ByteArray as TryFrom<&'b [u8]>>::Error: core::fmt::Debug,
{
    #[inline]
    fn get_pos(&self) -> usize {
        self.pos
    }

    #[inline]
    fn get_remaining_size(&self) -> usize {
        self.input.len() - self.get_pos()
    }

    #[inline]
    fn mark_num_bytes_as_consumed(&mut self, num_bytes: usize) {
        self.pos += num_bytes;
    }

    #[inline]
    fn peek_input(&self) -> Result<Self::ByteArray, Self::Error> {
        let byte_count = Self::ByteArray::default()[..].len();

        // Do we have N or more bytes left in
        // the input buffer? If so, then we can just read from said buffer.
        if let Some(chunk) =
            self.input.get(self.pos..).and_then(|s| s.get(..byte_count))
        {
            return Ok(chunk.try_into().unwrap());
        }

        // We have to use intermediate buffer,
        // either because the input is running out of bytes,
        // or because we want to  enforce bounds checking.

        // Note that in order to keep all fill-level invariants
        // we must allow to over-read past-the-end a bit.
        if self.get_pos() > self.input.len() + 2 * byte_count {
            const ERR: &str = "Buffer overflow read in BitStreamer";
            return Err(ERR);
        }

        let mut tmp: Self::ByteArray = Self::ByteArray::default();
        tmp[..].variable_length_load(self.input, self.pos);
        Ok(tmp)
    }
}

impl<T, ByteArray> BitStreamByteSequenceRewind
    for BitStreamByteSequenceDefaultReader<'_, T, ByteArray>
where
    T: BitStreamSliceConstraints,
{
    #[inline]
    fn rewind(&self) -> Self {
        Self::try_from(self.input).unwrap()
    }
}

#[cfg(test)]
mod tests;
