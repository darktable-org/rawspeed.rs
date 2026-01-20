use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[expect(missing_debug_implementations)]
#[non_exhaustive]
pub struct BitOrderMSB32;

impl BitOrderTrait for BitOrderMSB32 {}

impl BitStreamTraits for BitOrderMSB32 {
    const TAG: BitOrder = BitOrder::MSB32;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type MCUByteArrayType = [u8; 4];

    type ChunkByteArrayType = Self::MCUByteArrayType;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Little;
}

#[cfg(test)]
pub mod tests;
