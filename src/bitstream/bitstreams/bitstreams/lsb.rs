use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[expect(missing_debug_implementations)]
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
