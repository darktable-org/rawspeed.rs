use rawspeed_common_exact_ops::exact_ops::div::CheckedDivExact;
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

pub trait BitStreamTraits
// where
//     <Self as BitStreamTraits>::StreamFlow: BitStreamFlowTrait,
{
    const TAG: BitOrder;
    type StreamFlow;
    const FIXED_SIZE_CHUNKS: bool;
    type MCUByteArrayType;
    const CHUNK_ENDIANNESS: Endianness;
}

#[inline]
#[must_use]
pub fn predict_bitstream_bytelen<BitOrder>(
    num_items: usize,
    item_bitlen: u32,
) -> u64
where
    BitOrder: BitStreamTraits,
{
    const {
        assert!(<BitOrder as BitStreamTraits>::FIXED_SIZE_CHUNKS);
    };
    let mcu_bytelen =
        size_of::<<BitOrder as BitStreamTraits>::MCUByteArrayType>();
    let mcu_bytelen = u64::try_from(mcu_bytelen).unwrap();
    let mcu_bitlen = mcu_bytelen.checked_mul(8).unwrap();
    let num_items = u64::try_from(num_items).unwrap();
    let item_bitlen = u64::from(item_bitlen);
    let bitlen = item_bitlen.checked_mul(num_items).unwrap();
    let bitlen = bitlen.checked_next_multiple_of(mcu_bitlen).unwrap();
    <_ as CheckedDivExact>::checked_div_exact(bitlen, 8).unwrap()
}

#[derive(Debug)]
#[non_exhaustive]
#[must_use]
pub struct MaximalPackedElementCount {
    pub bytelen: usize,
    pub item_count: u64,
}

impl MaximalPackedElementCount {
    #[inline]
    pub fn new<BitOrder>(bytelen: usize, item_packed_bitlen: u32) -> Self
    where
        BitOrder: BitOrderTrait + BitStreamTraits,
    {
        const {
            assert!(<BitOrder as BitStreamTraits>::FIXED_SIZE_CHUNKS);
        };
        assert_ne!(item_packed_bitlen, 0);
        let mcu_bytelen =
            size_of::<<BitOrder as BitStreamTraits>::MCUByteArrayType>();
        let mcu_bytelen = u64::try_from(mcu_bytelen).unwrap();
        let mcu_bitlen = mcu_bytelen.checked_mul(8).unwrap();
        let bytelen = u64::try_from(bytelen).unwrap();
        let num_mcus = bytelen.checked_div(mcu_bytelen).unwrap();
        let usable_bytelen = num_mcus.checked_mul(mcu_bytelen).unwrap();
        let num_bits = num_mcus.checked_mul(mcu_bitlen).unwrap();
        let item_count =
            num_bits.checked_div(item_packed_bitlen.into()).unwrap();
        Self {
            bytelen: usable_bytelen.try_into().unwrap(),
            item_count,
        }
    }
}

impl BitOrder {
    #[inline]
    #[must_use]
    pub fn predict_exact_bitstream_bytelen(
        self,
        num_items: usize,
        item_bitlen: u32,
    ) -> u64 {
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

#[cfg(test)]
mod tests;

pub use jpeg::BitOrderJPEG;
pub use lsb::BitOrderLSB;
pub use msb::BitOrderMSB;
pub use msb16::BitOrderMSB16;
pub use msb32::BitOrderMSB32;
