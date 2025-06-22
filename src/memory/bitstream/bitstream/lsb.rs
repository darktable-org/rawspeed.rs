use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_memory_bitstreamcache::bitstreamcache;

#[allow(missing_copy_implementations)]
pub struct BitOrderLSB;

impl BitOrderTrait for BitOrderLSB {}

impl BitStreamTraits for BitOrderLSB {
    const TAG: BitOrder = BitOrder::LSB;

    type StreamFlow = bitstreamcache::BitStreamCacheHighInLowOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type ChunkType = u32;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1;
}
