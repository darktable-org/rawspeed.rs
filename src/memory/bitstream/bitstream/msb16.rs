use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_memory_bitstreamcache::bitstreamcache;

#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct BitOrderMSB16;

impl BitOrderTrait for BitOrderMSB16 {}

impl BitStreamTraits for BitOrderMSB16 {
    const TAG: BitOrder = BitOrder::MSB16;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type ChunkType = u16;
    type ChunkByteArrayType = [u8; 2];

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 2;
}
