use rawspeed_common::common::Bitwidth;
use rawspeed_memory_bitstream::bitstream;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_memory_bitstream::bitstream::BitOrderTrait;
use rawspeed_memory_bitstream::bitstream::BitStreamTraits;
use rawspeed_memory_bitstreamcache::bitstreamcache;
use rawspeed_memory_bitvacuumer::bitvacuumer::BitVacuumerBase;
use rawspeed_memory_bitvacuumer::bitvacuumer::BitVacuumerDrainImpl;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_std::array2dref::Array2DRef;
use rawspeed_std::coord_common::RowIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumBytes {
    val: usize,
}

impl NumBytes {
    #[inline]
    #[must_use]
    pub const fn new(val: usize) -> Self {
        Self { val }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for NumBytes {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinimalOutputRowPitch {
    val: NumBytes,
}

impl MinimalOutputRowPitch {
    #[inline]
    #[must_use]
    pub const fn new(val: NumBytes) -> Self {
        Self { val }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> NumBytes {
        self.val
    }
}

impl core::ops::Deref for MinimalOutputRowPitch {
    type Target = NumBytes;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExtraPadding {
    val: NumBytes,
}

impl ExtraPadding {
    #[inline]
    #[must_use]
    pub const fn new(val: NumBytes) -> Self {
        Self { val }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> NumBytes {
        self.val
    }
}

impl core::ops::Deref for ExtraPadding {
    type Target = NumBytes;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct OutputRowPitch {
    base: MinimalOutputRowPitch,
    padding: ExtraPadding,
}

impl OutputRowPitch {
    #[inline]
    #[must_use]
    pub const fn new_with_padding(
        base: MinimalOutputRowPitch,
        padding: ExtraPadding,
    ) -> Self {
        #![expect(unused_comparisons, clippy::absurd_extreme_comparisons)]
        assert!(padding.val().val() >= 0);
        Self { base, padding }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> NumBytes {
        NumBytes::new(
            self.base
                .val()
                .val()
                .checked_add(self.padding.val().val())
                .unwrap(),
        )
    }
}

#[derive(Debug)]
struct ByteCountingWriter<'a, W>
where
    W: std::io::Write,
{
    writer: &'a mut W,
    num_bytes: NumBytes,
}

impl<'a, W> ByteCountingWriter<'a, W>
where
    W: std::io::Write,
{
    #[inline]
    #[must_use]
    const fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            num_bytes: NumBytes::new(0),
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<W> std::io::Write for ByteCountingWriter<'_, W>
where
    W: std::io::Write,
{
    #[expect(clippy::unwrap_in_result)]
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res = self.writer.write(buf);
        if let Ok(res) = res {
            self.num_bytes =
                NumBytes::new((*self.num_bytes).checked_add(res).unwrap());
        }
        res
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[derive(Debug)]
pub struct Packer<'a, 'b, W, T>
where
    W: std::io::Write,
{
    writer: &'b mut W,
    bit_order: BitOrder,
    item_bitlen: usize,
    pitch: OutputRowPitch,
    input: Array2DRef<'a, T>,
}

impl<'a, 'b, W, T> Packer<'a, 'b, W, T>
where
    W: std::io::Write,
    T: Copy,
    u64: From<T>,
{
    #[inline]
    #[must_use]
    pub fn new<F>(
        writer: &'b mut W,
        bit_order: BitOrder,
        item_bitlen: usize,
        input: Array2DRef<'a, T>,
        pitch_adj_cb: F,
    ) -> Self
    where
        F: FnOnce(MinimalOutputRowPitch) -> ExtraPadding,
    {
        let min_pitch = MinimalOutputRowPitch::new(NumBytes::new(
            bit_order.predict_exact_bitstream_bytelen(
                input.row_length(),
                item_bitlen,
            ),
        ));
        let extra_padding = pitch_adj_cb(min_pitch);
        let pitch = OutputRowPitch::new_with_padding(min_pitch, extra_padding);
        Self {
            writer,
            bit_order,
            item_bitlen,
            pitch,
            input,
        }
    }

    #[expect(clippy::unwrap_in_result)]
    fn pack_row<BitOrder>(
        &mut self,
        row: RowIndex,
    ) -> std::io::Result<MinimalOutputRowPitch>
    where
        BitOrder: BitOrderTrait + BitStreamTraits,
        BitOrder::StreamFlow: bitstreamcache::BitStreamCache + Default,
        BitOrder::ChunkType: Bitwidth
            + TryFrom<<BitOrder::StreamFlow as bitstreamcache::BitStreamCache>::Storage>
            + SwapBytes,
        u32: From<BitOrder::ChunkType>,
        for<'c, 'd> BitVacuumerBase<'d, BitOrder, ByteCountingWriter<'c, W>>:
            BitVacuumerDrainImpl,
        <BitOrder::StreamFlow as bitstreamcache::BitStreamCache>::Storage: From<u64>,
        u64: From<T>,
    {
        let row = self.input.get_row(row).unwrap();
        let mut row_writer = ByteCountingWriter::new(&mut *self.writer);
        let mut vac: BitVacuumerBase<'_, BitOrder, _> =
            BitVacuumerBase::new(&mut row_writer);
        for item in row.iter().copied() {
            vac.put(item.into(), self.item_bitlen)?;
        }
        vac.flush()?;
        Ok(MinimalOutputRowPitch::new(row_writer.num_bytes))
    }

    fn pack_impl<BitOrder>(&mut self) -> std::io::Result<()>
    where
        BitOrder: BitOrderTrait + BitStreamTraits,
        BitOrder::StreamFlow: bitstreamcache::BitStreamCache + Default,
        BitOrder::ChunkType: Bitwidth
            + TryFrom<<BitOrder::StreamFlow as bitstreamcache::BitStreamCache>::Storage>
            + SwapBytes,
        u32: From<BitOrder::ChunkType>,
        for<'c, 'd> BitVacuumerBase<'d, BitOrder, ByteCountingWriter<'c, W>>:
            BitVacuumerDrainImpl,
        <BitOrder::StreamFlow as bitstreamcache::BitStreamCache>::Storage: From<u64>,
        u64: From<T>,
    {
        for row in 0..self.input.num_rows() {
            let real_pitch = self.pack_row::<BitOrder>(RowIndex::new(row))?;
            assert_eq!(real_pitch, self.pitch.base);

            let padding = core::iter::repeat_n(0_u8, **self.pitch.padding)
                .collect::<Vec<u8>>();
            self.writer.write_all(&padding)?;
        }
        Ok(())
    }

    #[inline]
    #[expect(clippy::unimplemented)]
    pub fn pack(mut self) -> std::io::Result<OutputRowPitch> {
        match self.bit_order {
            BitOrder::LSB => {
                self.pack_impl::<bitstream::BitOrderLSB>()?;
            }
            BitOrder::MSB => {
                self.pack_impl::<bitstream::BitOrderMSB>()?;
            }
            BitOrder::MSB32 => {
                self.pack_impl::<bitstream::BitOrderMSB32>()?;
            }
            BitOrder::JPEG => unreachable!(),
            BitOrder::MSB16 => {
                unimplemented!()
            }
            _ => unimplemented!(),
        }
        Ok(self.pitch)
    }
}

#[cfg(test)]
mod tests;
