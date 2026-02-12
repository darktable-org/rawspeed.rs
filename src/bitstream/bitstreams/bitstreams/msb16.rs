use rawspeed_bitstream_bitstreamcache::bitstreamcache;
use rawspeed_memory_endianness::endianness::Endianness;

use crate::bitstreams::{BitOrder, BitOrderTrait, BitStreamTraits};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitOrderMSB16;

impl BitOrderTrait for BitOrderMSB16 {}

impl BitStreamTraits for BitOrderMSB16 {
    const TAG: BitOrder = BitOrder::MSB16;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 2];

    type ChunkByteArrayType = Self::MCUByteArrayType;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;
}
