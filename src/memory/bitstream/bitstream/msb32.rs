use super::*;

use rawspeed_memory_bitstreamcache::bitstreamcache;

pub struct BitOrderMSB32;

impl BitOrderTrait for BitOrderMSB32 {}

impl BitStreamTraits for BitOrderMSB32 {
    const TAG: BitOrder = BitOrder::MSB32;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type ChunkType = u32;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 4;
}
