use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[expect(missing_debug_implementations)]
#[non_exhaustive]
pub struct BitOrderMSB16;

impl BitOrderTrait for BitOrderMSB16 {}

impl BitStreamTraits for BitOrderMSB16 {
    const TAG: BitOrder = BitOrder::MSB16;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 2];

    type ChunkByteArrayType = Self::MCUByteArrayType;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;
}
