use rawspeed_bitstream_bitstreamcache::bitstreamcache;
use rawspeed_memory_endianness::endianness::Endianness;

use crate::bitstreams::{BitOrder, BitOrderTrait, BitStreamTraits};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitOrderMSB32;

impl BitOrderTrait for BitOrderMSB32 {}

impl BitStreamTraits for BitOrderMSB32 {
    const TAG: BitOrder = BitOrder::MSB32;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 4];

    type ChunkByteArrayType = Self::MCUByteArrayType;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;
}
