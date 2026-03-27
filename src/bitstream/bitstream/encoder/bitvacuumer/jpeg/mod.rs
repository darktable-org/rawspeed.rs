use crate::bitvacuumer::{
    AsSlice, BitVacuumer, BitVacuumerBase, BitVacuumerDefaultDrainImpl,
    BitVacuumerDrainImpl, Bitwidth, SwapBytes, get_host_endianness,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::{
    BitStreamCache as _, BitStreamCacheData,
};
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderJPEG, BitOrderTrait, BitStreamTraits,
};
use rawspeed_common_generic_num::generic_num::bit_transmutation::{
    ConcatBytesNe, ToNeBytes,
};

type T = BitOrderJPEG;

pub type BitVacuumerJPEG<'a, W> = BitVacuumerBase<'a, T, W>;

impl<W> BitVacuumer for BitVacuumerJPEG<'_, W> where W: std::io::Write {}

impl<W> BitVacuumerDrainImpl<T> for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    W: std::io::Write,
    u32: From<u8>
        + Bitwidth
        + From<
            <<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output,
        > + core::ops::Shl<usize>
        + core::ops::ShlAssign<usize>
        + core::ops::BitOrAssign,
    <<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output:
        Bitwidth + SwapBytes + TryFrom<u64>,
{
    #[inline]
    fn drain_impl<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output>
            + TryFrom<u64>
            + ToNeBytes + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType=u8>,
        <CacheStorage as TryFrom<u64>>::Error: core::fmt::Debug
        {
        assert!(self.cache.fill_level() >= CacheStorage::BITWIDTH);

        let chunk = CacheStorage::try_from(
            self.cache.peek(CacheStorage::BITWIDTH).zext(),
        )
        .unwrap();

        if chunk
            .to_ne_bytes()
            .as_slice()
            .iter()
            .all(|byte| *byte != 0xFF_u8)
        {
            return BitVacuumerDefaultDrainImpl::<T>::drain_impl::<CacheStorage>(
                self,
            );
        }

        self.cache.skip(CacheStorage::BITWIDTH);
        let chunk = chunk.get_byte_swapped(
            <T as BitStreamTraits>::CHUNK_ENDIANNESS != get_host_endianness(),
        );
        for byte in chunk.to_ne_bytes().as_slice().iter().copied() {
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
