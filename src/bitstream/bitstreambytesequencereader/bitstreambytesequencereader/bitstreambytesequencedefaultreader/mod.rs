use crate::bitstreambytesequencereader::BitStreamByteSequenceRead;
use rawspeed_bitstream_bitstreamslice::bitstreamslice::{
    BitStreamSlice, BitStreamSliceConstraints, BitStreamSliceError,
};
use rawspeed_memory_variable_length_load::variable_length_load::VariableLengthLoad as _;

#[derive(Debug)]
pub struct BitStreamByteSequenceDefaultReader<'a, T> {
    input: &'a [u8],
    pos: usize,
    _phantom_data: core::marker::PhantomData<T>,
}

impl<'a, T> BitStreamByteSequenceDefaultReader<'a, T>
where
    T: BitStreamSliceConstraints,
{
    #[inline]
    #[must_use]
    pub const fn new(input: BitStreamSlice<'a, T>) -> Self {
        Self {
            input: input.get_bytes(),
            pos: 0,
            _phantom_data: core::marker::PhantomData,
        }
    }
}

impl<'a, T> From<BitStreamSlice<'a, T>>
    for BitStreamByteSequenceDefaultReader<'a, T>
where
    T: BitStreamSliceConstraints,
{
    #[inline]
    fn from(input: BitStreamSlice<'a, T>) -> Self {
        Self::new(input)
    }
}

impl<'a, T> TryFrom<&'a [u8]> for BitStreamByteSequenceDefaultReader<'a, T>
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

impl<'a, T> BitStreamByteSequenceRead<T>
    for BitStreamByteSequenceDefaultReader<'a, T>
where
    T: BitStreamSliceConstraints,
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
    fn peek_input<ByteArray>(&self) -> Result<ByteArray, &'static str>
    where
        ByteArray: Default
            + core::ops::IndexMut<core::ops::RangeFull, Output = [u8]>
            + TryFrom<&'a [u8]>,
        <ByteArray as TryFrom<&'a [u8]>>::Error: core::fmt::Debug,
    {
        let byte_count = ByteArray::default()[..].len();

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

        let mut tmp: ByteArray = ByteArray::default();
        tmp[..].variable_length_load(self.input, self.pos);
        Ok(tmp)
    }
}

#[cfg(test)]
mod tests;
