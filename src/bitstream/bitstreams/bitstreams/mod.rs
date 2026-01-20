use rawspeed_common_generic_num::generic_num::common::Bitwidth as _;
use rawspeed_memory_endianness::endianness::Endianness;

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
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
    type ChunkByteArrayType;
    const CHUNK_ENDIANNESS: Endianness;
    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32;
}

#[inline]
const fn predict_bitstream_bytelen<BS>(
    num_items: usize,
    item_bitlen: usize,
) -> usize
where
    BS: BitStreamTraits,
{
    const {
        assert!(BS::FIXED_SIZE_CHUNKS);
    };
    let bitlen = item_bitlen.checked_mul(num_items).unwrap();
    let bitlen = bitlen.checked_next_multiple_of(u32::BITWIDTH).unwrap();
    assert!(bitlen.is_multiple_of(8));
    bitlen / 8
}

impl BitOrder {
    #[inline]
    #[must_use]
    pub const fn predict_exact_bitstream_bytelen(
        self,
        num_items: usize,
        item_bitlen: usize,
    ) -> usize {
        match self {
            BitOrder::LSB => {
                predict_bitstream_bytelen::<BitOrderLSB>(num_items, item_bitlen)
            }
            BitOrder::MSB => {
                predict_bitstream_bytelen::<BitOrderMSB>(num_items, item_bitlen)
            }
            BitOrder::MSB16 => predict_bitstream_bytelen::<BitOrderMSB16>(
                num_items,
                item_bitlen,
            ),
            BitOrder::MSB32 => predict_bitstream_bytelen::<BitOrderMSB32>(
                num_items,
                item_bitlen,
            ),
            #[expect(clippy::unimplemented)]
            BitOrder::JPEG => unimplemented!(),
        }
    }
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
