pub trait BitStreamByteSequenceReadResult {
    type ByteArray;
    type Error;
}

pub trait BitStreamByteSequenceRead: BitStreamByteSequenceReadResult {
    #[must_use]
    fn get_pos(&self) -> usize;

    #[must_use]
    fn get_remaining_size(&self) -> usize;

    fn mark_num_bytes_as_consumed(&mut self, num_bytes: usize);

    fn peek_input(&self) -> Result<Self::ByteArray, Self::Error>;
}

pub trait BitStreamByteSequenceRewind {
    #[must_use]
    fn rewind(&self) -> Self;
}

mod bitstreambytesequencedefaultreader;
pub use bitstreambytesequencedefaultreader::BitStreamByteSequenceDefaultReader;

mod bitstreambytesequenceprefetchingreader;
pub use bitstreambytesequenceprefetchingreader::BitStreamByteSequencePrefetchingReader;

#[cfg(test)]
mod tests;
