use rawspeed_bitstream_bitstreams::bitstreams::BitOrderLSB;

use crate::{
    bitstreambytesequencereader::{
        BitStreamByteSequenceDefaultReader, BitStreamByteSequenceRead as _,
        BitStreamByteSequenceRewind as _,
    },
    test_lsb,
};

test_lsb!(BitStreamByteSequenceDefaultReader::<'_, BitOrderLSB, _>);
