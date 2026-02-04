use rawspeed_bitstream_bitstream_decoder::bitstreamer::{
    BitStreamerBase, BitStreamerCacheFillImpl, BitStreamerReplenisher,
    BitStreamerReplenisherStorage, BitStreamerTraits,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSlice;
use rawspeed_common_lcm::lcm::lcm;

#[derive(Debug)]
pub struct PackingDescription<BitOrder, const ITEM_PACKED_BITLEN: usize> {
    packed_mcu_bytelen: usize,
    packed_item_count: usize,
    #[expect(dead_code)]
    packed_item_bitlen: usize,
    _phantom: core::marker::PhantomData<BitOrder>,
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize>
    PackingDescription<BitOrder, ITEM_PACKED_BITLEN>
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
    for PackingDescription<BitOrder, ITEM_PACKED_BITLEN>
where
    BitOrder: BitOrderTrait + BitStreamTraits,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PackedBitstreamSlice<BitOrder, const ITEM_PACKED_BITLEN: usize>
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    storage: [u16; 32],
    _phantom: core::marker::PhantomData<BitOrder>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct PackedBitstreamSliceWrongSizeError {
    expected: usize,
    actual: usize,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum PackedBitstreamSliceError {
    WrongSize(PackedBitstreamSliceWrongSizeError),
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize>
    PackedBitstreamSlice<BitOrder, ITEM_PACKED_BITLEN>
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    const DSC: PackingDescription<BitOrder, ITEM_PACKED_BITLEN> =
        PackingDescription::new();

    #[inline]
    pub fn new<'a>(
        slice: BitStreamSlice<'a, BitOrder>,
    ) -> Result<Self, PackedBitstreamSliceError>
        where
            BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
            BitStreamerBase<'a, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
            BitStreamerReplenisherStorage<'a, BitOrder>: BitStreamerReplenisher<'a, BitOrder>,
            <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
            u64: From<<<BitOrder as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage>,
            u16: TryFrom<u64>
      {
        const {
            assert!(ITEM_PACKED_BITLEN >= 1 && ITEM_PACKED_BITLEN <= 16);
        }
        if slice.get_bytes().len() != Self::DSC.packed_mcu_bytelen {
            return Err(PackedBitstreamSliceError::WrongSize(
                PackedBitstreamSliceWrongSizeError {
                    expected: Self::DSC.packed_mcu_bytelen,
                    actual: slice.get_bytes().len(),
                },
            ));
        }
        let mut storage = core::array::from_fn(|_| 0_u16);
        let items = storage.get_mut(..Self::len()).unwrap();
        let mut bs = BitStreamerBase::<BitOrder>::new(slice);
        for item in items.iter_mut() {
            bs.fill(ITEM_PACKED_BITLEN).unwrap();
            let bits = bs.peek_bits_no_fill(ITEM_PACKED_BITLEN);
            bs.skip_bits_no_fill(ITEM_PACKED_BITLEN);
            *item = bits.try_into().unwrap();
        }
        Ok(Self {
            storage,
            _phantom: core::marker::PhantomData,
        })
    }

    #[must_use]
    #[inline]
    pub const fn len() -> usize {
        Self::DSC.packed_item_count
    }

    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[u16] {
        self.storage.get(..Self::len()).unwrap()
    }
}

#[cfg(test)]
mod tests;
