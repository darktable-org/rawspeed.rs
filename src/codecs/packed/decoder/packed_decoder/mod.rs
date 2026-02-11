use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerBase;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerCacheFillImpl;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerReplenisher;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerReplenisherStorage;
use rawspeed_bitstream_bitstream_decoder::bitstreamer::BitStreamerTraits;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderTrait;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;
use rawspeed_common_generic_num::generic_num::common::Bitwidth;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

#[derive(Debug)]
pub struct Unpacker<'a, 'b, 'c, T>
where
    T: Bitwidth + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: core::fmt::Debug,
{
    input: Array2DRef<'a, u8>,
    bit_order: BitOrder,
    item_bitlen: u32,
    output: &'c mut Array2DRefMut<'b, T>,
}

impl<'a, 'b, 'c, T> Unpacker<'a, 'b, 'c, T>
where
    T: Bitwidth + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: core::fmt::Debug,
{
    #[inline]
    #[must_use]
    pub fn new(
        input: Array2DRef<'a, u8>,
        bit_order: BitOrder,
        item_bitlen: u32,
        output: &'c mut Array2DRefMut<'b, T>,
    ) -> Self {
        #[expect(clippy::unimplemented)]
        if bit_order == BitOrder::JPEG {
            unimplemented!("Bit order {:?} is not unpackable!", bit_order)
        }
        assert!(item_bitlen > 0);
        assert!(item_bitlen <= T::BITWIDTH);
        Self {
            input,
            bit_order,
            item_bitlen,
            output,
        }
    }

    #[inline]
    fn unpack_row<BitOrder>(&mut self, row: RowIndex)
    where
        BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
        for<'z> BitStreamerBase<'z, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
        for<'z> BitStreamerReplenisherStorage<'z, BitOrder>:
            BitStreamerReplenisher<'z, BitOrder>,
        <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
        u64: From<<<BitOrder as BitStreamTraits>::StreamFlow as  BitStreamCache>::Storage>,
    {
        let bytes = self.input.get_row(row).unwrap();
        let row = self.output.get_row_mut(row).unwrap();
        let mut bs =
            BitStreamerBase::<BitOrder>::new(bytes.try_into().unwrap());
        for item in row.iter_mut() {
            bs.fill(self.item_bitlen).unwrap();
            *item = bs.peek_bits_no_fill(self.item_bitlen).try_into().unwrap();
            bs.skip_bits_no_fill(self.item_bitlen);
        }
    }

    #[inline]
    fn unpack_impl<BitOrder>(&mut self)
    where
        BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits + BitStreamerTraits,
        for<'z> BitStreamerBase<'z, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
        for<'z> BitStreamerReplenisherStorage<'z, BitOrder>:
            BitStreamerReplenisher<'z, BitOrder>,
        <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
        u64: From<<<BitOrder as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage>,
    {
        assert_eq!(self.input.num_rows(), self.output.num_rows());
        for row in 0..*self.input.num_rows() {
            self.unpack_row::<BitOrder>(RowIndex::new(row));
        }
    }

    #[inline]
    pub fn unpack(mut self) {
        match self.bit_order {
            BitOrder::LSB => self.unpack_impl::<bitstreams::BitOrderLSB>(),
            BitOrder::MSB => self.unpack_impl::<bitstreams::BitOrderMSB>(),
            BitOrder::MSB16 => self.unpack_impl::<bitstreams::BitOrderMSB16>(),
            BitOrder::MSB32 => self.unpack_impl::<bitstreams::BitOrderMSB32>(),
            BitOrder::JPEG => unreachable!(),
            _ => unreachable!("TODO"),
        }
    }
}

mod tests;
