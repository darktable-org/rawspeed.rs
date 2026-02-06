use rawspeed_bitstream_bitstream_decoder::bitstreamer::{
    BitStreamerBase, BitStreamerCacheFillImpl, BitStreamerReplenisher,
    BitStreamerReplenisherStorage, BitStreamerTraits,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_bitstream_packedbitstreamslice::packedbitstreamslice::{
    PackedBitstreamSlice, PackingDescription,
};

#[derive(Debug)]
pub struct PackedBitstreamUnpacker<BitOrder, const ITEM_PACKED_BITLEN: usize>
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    storage: [u16; 32],
    _phantom: core::marker::PhantomData<BitOrder>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct PackedBitstreamUnpackerWrongSizeError {
    expected: usize,
    actual: usize,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum PackedBitstreamUnpackerError {
    WrongSize(PackedBitstreamUnpackerWrongSizeError),
}

impl<BitOrder, const ITEM_PACKED_BITLEN: usize>
    PackedBitstreamUnpacker<BitOrder, ITEM_PACKED_BITLEN>
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits,
{
    const DSC: PackingDescription<BitOrder, ITEM_PACKED_BITLEN> =
        PackingDescription::new();

    #[expect(clippy::needless_pass_by_value)]
    #[inline]
    pub fn new<'a>(
        slice: PackedBitstreamSlice<'a, BitOrder, ITEM_PACKED_BITLEN>,
    ) -> Result<Self, PackedBitstreamUnpackerError>
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
        if slice.get_slice().get_bytes().len() != Self::DSC.packed_mcu_bytelen()
        {
            return Err(PackedBitstreamUnpackerError::WrongSize(
                PackedBitstreamUnpackerWrongSizeError {
                    expected: Self::DSC.packed_mcu_bytelen(),
                    actual: slice.get_slice().get_bytes().len(),
                },
            ));
        }
        let mut storage = core::array::from_fn(|_| 0_u16);
        let items = storage.get_mut(..Self::len()).unwrap();
        let mut bs = BitStreamerBase::<BitOrder>::new(slice.get_slice());
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
        Self::DSC.packed_item_count()
    }

    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[u16] {
        self.storage.get(..Self::len()).unwrap()
    }
}

#[cfg(test)]
mod tests;
