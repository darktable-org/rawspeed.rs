use rawspeed_memory_endianness::endianness::Endianness;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitOrder {
    LSB,
    MSB,
    MSB16,
    MSB32,
    JPEG,
}

pub trait BitOrderTrait {}

pub trait BitStreamTraits {
    const TAG: BitOrder;
    type StreamFlow;
    const FIXED_SIZE_CHUNKS: bool;
    type ChunkType;
    type ChunkByteArrayType;
    const CHUNK_ENDIANNESS: Endianness;
    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32;
}

mod jpeg;
mod lsb;
mod msb;
mod msb16;
mod msb32;

pub use jpeg::BitOrderJPEG;
pub use lsb::BitOrderLSB;
pub use msb::BitOrderMSB;
pub use msb16::BitOrderMSB16;
pub use msb32::BitOrderMSB32;
