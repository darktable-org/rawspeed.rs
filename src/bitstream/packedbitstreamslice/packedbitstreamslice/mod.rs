use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_bitstream_bitstreamslice::bitstreamslice::{
    BitStreamSlice, BitStreamSliceConstraints,
};
use rawspeed_common_lcm::lcm::lcm;

pub trait BitPackingLayout<const ITEM_PACKED_BITLEN: usize> {
    const PACKED_MCU_BYTELEN: usize;
    const PACKED_ITEM_COUNT: usize;
    const PACKED_ITEM_BITLEN: usize;
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize>
    BitPackingLayout<ITEM_PACKED_BITLEN> for BitOrder
where
    BitOrder: BitOrderTrait + BitStreamTraits,
{
    const PACKED_MCU_BYTELEN: usize =
        PackingDescriptionImpl::<BitOrder, ITEM_PACKED_BITLEN>::new()
            .packed_mcu_bytelen;
    const PACKED_ITEM_COUNT: usize =
        PackingDescriptionImpl::<BitOrder, ITEM_PACKED_BITLEN>::new()
            .packed_item_count;
    const PACKED_ITEM_BITLEN: usize =
        PackingDescriptionImpl::<BitOrder, ITEM_PACKED_BITLEN>::new()
            .packed_item_bitlen;
}

#[derive(Debug)]
// #[non_exhaustive]
pub struct PackingDescriptionImpl<BitOrder, const ITEM_PACKED_BITLEN: usize> {
    packed_mcu_bytelen: usize,
    packed_item_count: usize,
    packed_item_bitlen: usize,
    _phantom: core::marker::PhantomData<BitOrder>,
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize>
    PackingDescriptionImpl<BitOrder, ITEM_PACKED_BITLEN>
{
    #[must_use]
    #[inline]
    pub const fn new() -> Self
    where
        BitOrder: BitOrderTrait + BitStreamTraits,
    {
        const {
            assert!(ITEM_PACKED_BITLEN >= 1 && ITEM_PACKED_BITLEN <= 16);
        };
        let mcu_bytelen =
            size_of::<<BitOrder as BitStreamTraits>::MCUByteArrayType>();
        let mcu_bitlen = mcu_bytelen.checked_mul(8).unwrap();
        let packed_mcu_bitlen = lcm!(mcu_bitlen, ITEM_PACKED_BITLEN).unwrap();
        let packed_item_count = packed_mcu_bitlen / ITEM_PACKED_BITLEN;
        let packed_mcu_bytelen = packed_mcu_bitlen / 8;
        Self {
            packed_mcu_bytelen,
            packed_item_count,
            packed_item_bitlen: ITEM_PACKED_BITLEN,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize> Default
    for PackingDescriptionImpl<BitOrder, ITEM_PACKED_BITLEN>
where
    BitOrder: BitOrderTrait + BitStreamTraits,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PackedBitstreamSlice<'a, BitOrder, const ITEM_PACKED_BITLEN: usize>
where
    BitOrder: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitPackingLayout<ITEM_PACKED_BITLEN>
        + BitStreamSliceConstraints,
{
    slice: BitStreamSlice<'a, BitOrder>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct PackedBitstreamSliceWrongSizeError {
    expected_multiplicity: usize,
    actual_len: usize,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum PackedBitstreamSliceError {
    WrongSize(PackedBitstreamSliceWrongSizeError),
}

impl<'a, BitOrder, const ITEM_PACKED_BITLEN: usize>
    PackedBitstreamSlice<'a, BitOrder, ITEM_PACKED_BITLEN>
where
    BitOrder: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamSliceConstraints,
{
    #[inline]
    pub const fn new(
        slice: BitStreamSlice<'a, BitOrder>,
    ) -> Result<Self, PackedBitstreamSliceError> {
        const {
            assert!(ITEM_PACKED_BITLEN >= 1 && ITEM_PACKED_BITLEN <= 16);
        }
        let packed_mcu_bytelen = <BitOrder as BitPackingLayout<
            ITEM_PACKED_BITLEN,
        >>::PACKED_MCU_BYTELEN;
        if !slice.get_bytes().len().is_multiple_of(packed_mcu_bytelen) {
            return Err(PackedBitstreamSliceError::WrongSize(
                PackedBitstreamSliceWrongSizeError {
                    expected_multiplicity: packed_mcu_bytelen,
                    actual_len: slice.get_bytes().len(),
                },
            ));
        }
        Ok(Self { slice })
    }

    #[must_use]
    #[inline]
    pub const fn get_slice(&self) -> BitStreamSlice<'a, BitOrder> {
        self.slice
    }
}

#[cfg(test)]
mod tests;
