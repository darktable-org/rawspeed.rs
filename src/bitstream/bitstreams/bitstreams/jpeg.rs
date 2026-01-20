use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[expect(missing_debug_implementations)]
#[non_exhaustive]
pub struct BitOrderJPEG;

impl BitOrderTrait for BitOrderJPEG {}

impl BitStreamTraits for BitOrderJPEG {
    const TAG: BitOrder = BitOrder::JPEG;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = false; // Stuffing byte...

    type ChunkByteArrayType = [u8; 4];

    const CHUNK_ENDIANNESS: Endianness = Endianness::Big;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1; // FIXME
}
