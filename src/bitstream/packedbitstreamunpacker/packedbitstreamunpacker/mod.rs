use rawspeed_bitstream_bitstream_decoder::bitstreamer::{
    BitStreamerBase, BitStreamerCacheFillImpl, BitStreamerReplenisher,
    BitStreamerReplenisherStorage, BitStreamerTraits,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_bitstream_packedbitstreamslice::packedbitstreamslice::{
    BitPackingLayout, PackedBitstreamSlice,
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
    BitOrder: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitPackingLayout<ITEM_PACKED_BITLEN>,
{
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
        let package_bytelen = <BitOrder as BitPackingLayout<
            ITEM_PACKED_BITLEN,
        >>::PACKED_MCU_BYTELEN;
        if slice.get_slice().get_bytes().len() != package_bytelen {
            return Err(PackedBitstreamUnpackerError::WrongSize(
                PackedBitstreamUnpackerWrongSizeError {
                    expected: package_bytelen,
                    actual: slice.get_slice().get_bytes().len(),
                },
            ));
        }
        let mut storage = core::array::from_fn(|_| 0_u16);
        let items = storage.get_mut(..Self::len()).unwrap();
        let mut bs = BitStreamerBase::<BitOrder>::new(slice.get_slice());
        for item in items.iter_mut() {
            let item_packed_bitlen_ = ITEM_PACKED_BITLEN.try_into().unwrap();
            bs.fill(item_packed_bitlen_).unwrap();
            let bits = bs.peek_bits_no_fill(item_packed_bitlen_).zext();
            bs.skip_bits_no_fill(item_packed_bitlen_);
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
        <BitOrder as BitPackingLayout<ITEM_PACKED_BITLEN>>::PACKED_ITEM_COUNT
    }

    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[u16] {
        self.storage.get(..Self::len()).unwrap()
    }
}

#[cfg(test)]
mod tests;
