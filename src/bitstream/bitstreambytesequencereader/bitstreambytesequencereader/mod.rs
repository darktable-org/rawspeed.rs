use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSliceConstraints;

pub trait BitStreamByteSequenceRead<T>
where
    T: BitStreamSliceConstraints,
{
    #[must_use]
    fn get_pos(&self) -> usize;

    #[must_use]
    fn get_remaining_size(&self) -> usize;

    fn mark_num_bytes_as_consumed(&mut self, num_bytes: usize);

    fn peek_input<ByteArray>(&self) -> Result<ByteArray, &'static str>
    where
        for<'a> ByteArray: Default
            + core::ops::IndexMut<core::ops::RangeFull, Output = [u8]>
            + TryFrom<&'a [u8]>,
        for<'a> <ByteArray as TryFrom<&'a [u8]>>::Error: core::fmt::Debug;
}

pub trait BitStreamByteSequenceRewind<T>
where
    T: BitStreamSliceConstraints,
{
    #[must_use]
    fn rewind(&self) -> Self;
}

mod bitstreambytesequencedefaultreader;
pub use bitstreambytesequencedefaultreader::BitStreamByteSequenceDefaultReader;
