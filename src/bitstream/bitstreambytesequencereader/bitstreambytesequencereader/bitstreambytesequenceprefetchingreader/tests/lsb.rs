use rawspeed_bitstream_bitstreams::bitstreams::BitOrderLSB;

use crate::{
    bitstreambytesequencereader::{
        BitStreamByteSequenceDefaultReader,
        BitStreamByteSequencePrefetchingReader, BitStreamByteSequenceRead as _,
        BitStreamByteSequenceRewind as _,
    },
    test_lsb,
};

test_lsb!(
    BitStreamByteSequencePrefetchingReader::<
        BitStreamByteSequenceDefaultReader<'_, BitOrderLSB, _>,
    >
);
