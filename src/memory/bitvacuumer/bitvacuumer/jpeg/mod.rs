use super::{
    BitOrderTrait, BitStreamCache, BitStreamTraits, BitVacuumer,
    BitVacuumerBase, BitVacuumerDefaultDrainImpl, BitVacuumerDrainImpl,
    Bitwidth, SwapBytes, get_host_endianness,
};

use rawspeed_memory_bitstream::bitstream::BitOrderJPEG;

pub type BitVacuumerJPEG<'a, W> = BitVacuumerBase<'a, BitOrderJPEG, W>;

impl<W> BitVacuumer for BitVacuumerJPEG<'_, W> where W: std::io::Write {}

impl<W> BitVacuumerDrainImpl for BitVacuumerBase<'_, BitOrderJPEG, W>
where
    BitOrderJPEG: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<<BitOrderJPEG as BitStreamTraits>::ChunkType>
        + core::ops::Shl<usize>
        + core::ops::ShlAssign<usize>
        + core::ops::BitOrAssign,
    <BitOrderJPEG as BitStreamTraits>::StreamFlow: BitStreamCache,
    <BitOrderJPEG as BitStreamTraits>::ChunkType:
        Bitwidth + SwapBytes + TryFrom<u64>,
{
    #[inline]
    fn drain_impl(&mut self) -> std::io::Result<()> {
        type T = BitOrderJPEG;

        assert!(self.cache.fill_level() >= u32::BITWIDTH);

        let stream_chunk_bitwidth: usize =
            <T as BitStreamTraits>::ChunkType::BITWIDTH;

        assert!(u32::BITWIDTH == stream_chunk_bitwidth);

        let Ok(chunk) = <<T as BitStreamTraits>::ChunkType>::try_from(
            self.cache.peek(stream_chunk_bitwidth),
        ) else {
            panic!("lossless cast failed?")
        };

        if chunk.to_ne_bytes().iter().all(|byte| *byte != 0xFF_u8) {
            return BitVacuumerDefaultDrainImpl::drain_impl(self);
        }

        self.cache.skip(stream_chunk_bitwidth);
        let chunk = chunk.get_byte_swapped(
            <T as BitStreamTraits>::CHUNK_ENDIANNESS != get_host_endianness(),
        );
        let bytes = chunk.to_ne_bytes();
        for byte in bytes {
            self.writer.write_all(&[byte])?;
            if byte == 0xFF_u8 {
                const STUFFING_BYTE: u8 = 0x00_u8;
                self.writer.write_all(&[STUFFING_BYTE])?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
