use rawspeed_bitstream_bitstreamcache::bitstreamcache;
use rawspeed_memory_endianness::endianness::Endianness;

use crate::bitstreams::{BitOrder, BitOrderTrait, BitStreamTraits};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitOrderLSB;

impl BitOrderTrait for BitOrderLSB {}

impl BitStreamTraits for BitOrderLSB {
    const TAG: BitOrder = BitOrder::LSB;

    type StreamFlow = bitstreamcache::BitStreamCacheHighInLowOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 1];

    type ChunkByteArrayType = [u8; 4];

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;
}
