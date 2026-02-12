use rawspeed_bitstream_bitstreamcache::bitstreamcache;
use rawspeed_memory_endianness::endianness::Endianness;

use crate::bitstreams::{BitOrder, BitOrderTrait, BitStreamTraits};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitOrderMSB;

impl BitOrderTrait for BitOrderMSB {}

impl BitStreamTraits for BitOrderMSB {
    const TAG: BitOrder = BitOrder::MSB;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 1];

    type ChunkByteArrayType = [u8; 4];

    const CHUNK_ENDIANNESS: Endianness = Endianness::Big;
}
