use super::*;

use rawspeed_memory_bitstreamcache::bitstreamcache;

pub struct BitOrderJPEG;

impl BitOrderTrait for BitOrderJPEG {}

impl BitStreamTraits for BitOrderJPEG {
    const TAG: BitOrder = BitOrder::JPEG;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = false; // Stuffing byte...

    type ChunkType = u32;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Big;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1; // FIXME
}
