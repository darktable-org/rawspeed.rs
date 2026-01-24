use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[expect(missing_debug_implementations)]
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
