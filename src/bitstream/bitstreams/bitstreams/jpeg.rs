use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_bitstream_bitstreamcache::bitstreamcache;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BitOrderJPEG;

impl BitOrderTrait for BitOrderJPEG {}

impl BitStreamTraits for BitOrderJPEG {
    const TAG: BitOrder = BitOrder::JPEG;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = false; // Stuffing byte...

    type MCUByteArrayType = [u8; 1]; // FIXME

    type ChunkByteArrayType = [u8; 4];

    const CHUNK_ENDIANNESS: Endianness = Endianness::Big;
}
