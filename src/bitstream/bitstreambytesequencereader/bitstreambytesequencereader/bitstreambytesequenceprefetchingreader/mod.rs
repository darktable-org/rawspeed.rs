use rawspeed_bitstream_bitstreamslice::bitstreamslice::{
    BitStreamSlice, BitStreamSliceConstraints, BitStreamSliceError,
};

use crate::bitstreambytesequencereader::{
    BitStreamByteSequenceRead, BitStreamByteSequenceReadResult,
    BitStreamByteSequenceRewind,
};

pub trait BitStreamByteSequencePrefetch {
    fn prefetch(&mut self);
}

impl<R> BitStreamByteSequencePrefetch
    for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead,
{
    fn prefetch(&mut self) {
        let cache = self.reader.peek_input();
        self.cache = cache;
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[must_use]
pub struct BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceReadResult,
{
    reader: R,
    cache: Result<R::ByteArray, R::Error>,
}

impl<R> BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead<Error = &'static str>,
    Self: BitStreamByteSequenceRead,
{
    #[inline]
    pub fn new_from_reader(reader: R) -> Self {
        let mut reader = Self {
            reader,
            cache: Err(""),
        };
        reader.mark_num_bytes_as_consumed(0);
        reader
    }

    #[inline]
    pub fn new<'a, T>(input: BitStreamSlice<'a, T>) -> Self
    where
        T: BitStreamSliceConstraints,
        R: From<BitStreamSlice<'a, T>>,
    {
        let reader = R::from(input);
        Self::new_from_reader(reader)
    }
}

impl<R> From<R> for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead<Error = &'static str>,
    Self: BitStreamByteSequenceRead,
{
    #[inline]
    fn from(reader: R) -> Self {
        Self::new_from_reader(reader)
    }
}

impl<'a, T, R> From<BitStreamSlice<'a, T>>
    for BitStreamByteSequencePrefetchingReader<R>
where
    T: BitStreamSliceConstraints,
    R: BitStreamByteSequenceRead<Error = &'static str>
        + From<BitStreamSlice<'a, T>>,
    Self: From<R>,
{
    #[inline]
    fn from(input: BitStreamSlice<'a, T>) -> Self {
        let reader = input.into();
        Self::from(reader)
    }
}

impl<'a, R> TryFrom<&'a [u8]> for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead<Error = &'static str>
        + TryFrom<&'a [u8], Error = BitStreamSliceError>,
    Self: From<R>,
{
    type Error = BitStreamSliceError;

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let reader = R::try_from(value)?;
        Ok(Self::from(reader))
    }
}

impl<R> BitStreamByteSequenceReadResult
    for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceReadResult,
{
    type ByteArray = R::ByteArray;
    type Error = R::Error;
}

impl<R> BitStreamByteSequenceRead for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead,
    Result<R::ByteArray, R::Error>: Copy,
    Self: BitStreamByteSequencePrefetch,
{
    #[inline]
    fn get_pos(&self) -> usize {
        self.reader.get_pos()
    }

    #[inline]
    fn get_remaining_size(&self) -> usize {
        self.reader.get_remaining_size()
    }

    #[inline]
    fn mark_num_bytes_as_consumed(&mut self, num_bytes: usize) {
        self.reader.mark_num_bytes_as_consumed(num_bytes);
        self.prefetch();
    }

    #[inline]
    fn peek_input(&self) -> Result<Self::ByteArray, Self::Error> {
        self.cache
    }
}

impl<R> BitStreamByteSequenceRewind
    for BitStreamByteSequencePrefetchingReader<R>
where
    R: BitStreamByteSequenceRead<Error = &'static str>
        + BitStreamByteSequenceRewind,
    Self: BitStreamByteSequenceRead,
{
    #[inline]
    fn rewind(&self) -> Self {
        let reader = self.reader.rewind();
        Self::new_from_reader(reader)
    }
}

#[cfg(test)]
mod tests;
