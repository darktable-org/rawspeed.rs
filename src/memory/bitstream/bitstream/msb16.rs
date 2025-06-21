use super::*;

use rawspeed_memory_bitstreamcache::bitstreamcache;

pub struct BitOrderMSB16;

impl BitOrderTrait for BitOrderMSB16 {}

impl BitStreamTraits for BitOrderMSB16 {
    const TAG: BitOrder = BitOrder::MSB16;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type ChunkType = u16;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 2;
}
