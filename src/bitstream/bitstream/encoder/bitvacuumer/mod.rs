#[cfg(target_endian = "little")]
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCacheHighInLowOut;
#[cfg(not(target_endian = "little"))]
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCacheLowInHighOut;
use rawspeed_bitstream_bitstreamcache::bitstreamcache::{
    BitStreamCache, BitStreamCacheBase, BitStreamCacheData, BitStreamFlowTrait,
};
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderTrait, BitStreamTraits,
};
use rawspeed_common_bitseq::bitseq::{BitLen, BitSeq};
use rawspeed_common_exact_ops::exact_ops::div::CheckedDivExact;
use rawspeed_common_generic_num::generic_num::{
    bit_transmutation::{ConcatBytesNe, ToNeBytes},
    common::Bitwidth,
};
use rawspeed_memory_endianness::endianness::{SwapBytes, get_host_endianness};

pub trait AsSlice {
    type EltType;
    #[must_use]
    fn as_slice(&self) -> &[Self::EltType];
}

impl<T, const N: usize> AsSlice for [T; N] {
    type EltType = T;

    #[inline]
    fn as_slice(&self) -> &[Self::EltType] {
        self.as_slice()
    }
}

pub trait BitVacuumer {}

pub trait BitVacuumerDefaultDrainImpl<T>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::MCUByteArrayType: ConcatBytesNe,
{
    fn drain_impl<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<T::MCUByteArrayType as ConcatBytesNe>::Output>
            + TryFrom<u64>
            + ToNeBytes
            + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType = u8>,
        <CacheStorage as TryFrom<u64>>::Error: core::fmt::Debug;
}

pub trait BitVacuumerDrainImpl<T>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::MCUByteArrayType: ConcatBytesNe,
{
    fn drain_impl<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<T::MCUByteArrayType as ConcatBytesNe>::Output>
            + TryFrom<u64>
            + ToNeBytes
            + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType = u8>,
        <CacheStorage as TryFrom<u64>>::Error: core::fmt::Debug;
}

pub trait BitVacuumerUseDefaultDrainImpl {}

#[derive(Debug)]
#[non_exhaustive]
#[must_use]
pub struct BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    W: std::io::Write,
{
    cache:
        <<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache,
    writer: &'a mut W,
    _phantom_data: core::marker::PhantomData<T>,
}

impl<T, W> BitVacuumerDefaultDrainImpl<T> for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    W: std::io::Write,
    T::MCUByteArrayType: ConcatBytesNe,
    <T::MCUByteArrayType as ConcatBytesNe>::Output: Bitwidth
        + TryFrom<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>
        + SwapBytes,
    <<T::MCUByteArrayType as ConcatBytesNe>::Output as TryFrom<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>>::Error: core::fmt::Debug,
{
    #[inline]
    fn drain_impl<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<T::MCUByteArrayType as ConcatBytesNe>::Output>+ TryFrom<u64>
            + ToNeBytes + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType=u8>,
    {
        let mut cache: BitStreamCacheBase<_, CacheStorage> = {
            #[cfg(target_endian = "little")]
            {
                BitStreamCacheHighInLowOut::<CacheStorage>::new()
            }
            #[cfg(not(target_endian = "little"))]
            {
                BitStreamCacheLowInHighOut::<CacheStorage>::new()
            }
        };

        assert!(self.cache.fill_level() >= cache.size());

        let mcu_chunk_bitwidth =
            <T::MCUByteArrayType as ConcatBytesNe>::Output::BITWIDTH;

        assert!(cache.size() >= mcu_chunk_bitwidth);
        assert!(cache.size().is_multiple_of(mcu_chunk_bitwidth));
        let num_chunks_needed = cache.size() / mcu_chunk_bitwidth;
        assert!(num_chunks_needed >= 1);

        for _i in 0..num_chunks_needed {
            let chunk =
                <<T::MCUByteArrayType as ConcatBytesNe>::Output>::try_from(
                    self.cache.peek(mcu_chunk_bitwidth).zext(),
                )
                .unwrap();
            self.cache.skip(mcu_chunk_bitwidth);
            let chunk = chunk
                .get_byte_swapped(T::CHUNK_ENDIANNESS != get_host_endianness());
            cache.push(
                BitSeq::new(BitLen::new(mcu_chunk_bitwidth), chunk.into())
                    .unwrap(),
            );
        }
        let bytes = cache.peek(cache.size()).zext().to_ne_bytes();
        self.writer.write_all(bytes.as_slice())
    }
}

impl<T, W> BitVacuumerDrainImpl<T> for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits + BitVacuumerUseDefaultDrainImpl,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    W: std::io::Write,
    T::MCUByteArrayType: ConcatBytesNe,
    <T::MCUByteArrayType as ConcatBytesNe>::Output: Bitwidth
        + TryFrom<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>
        + SwapBytes,
    <<T::MCUByteArrayType as ConcatBytesNe>::Output as TryFrom<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>>::Error: core::fmt::Debug,
{
    #[inline]
    fn drain_impl<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<T::MCUByteArrayType as ConcatBytesNe>::Output>+ TryFrom<u64>
            + ToNeBytes + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType=u8>,
        <CacheStorage as TryFrom<u64>>::Error: core::fmt::Debug,
    {
        BitVacuumerDefaultDrainImpl::<T>::drain_impl::<CacheStorage>(self)
    }
}

impl<'a, T, W> BitVacuumerBase<'a, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    Self: BitVacuumerDrainImpl<T>,
    W: std::io::Write,
    T::MCUByteArrayType: ConcatBytesNe,
    <T::MCUByteArrayType as ConcatBytesNe>::Output: Bitwidth
        + TryFrom<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>
        + SwapBytes,
    u32: From<<T::MCUByteArrayType as ConcatBytesNe>::Output>,
    BitSeq<<<<T as BitStreamTraits>::StreamFlow as BitStreamFlowTrait<u64>>::Cache as BitStreamCache>::Storage>: From<BitSeq<u64>>,
{
    #[inline]
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            cache: Default::default(),
            writer,
            _phantom_data: core::marker::PhantomData,
        }
    }

    #[inline]
    fn flush_impl<MCUCacheStorage>(mut self) -> std::io::Result<()>
    where
        <T as BitStreamTraits>::MCUByteArrayType:
            ConcatBytesNe<Output = MCUCacheStorage>,
        MCUCacheStorage: BitStreamCacheData
            + ToNeBytes<Output = <T as BitStreamTraits>::MCUByteArrayType>
            + TryFrom<u64>
            + SwapBytes,
        <MCUCacheStorage as ToNeBytes>::Output: AsSlice<EltType = u8>,
        <MCUCacheStorage as TryFrom<u64>>::Error: core::fmt::Debug,
    {
        self.drain_priv::<u32>()?;

        if self.cache.fill_level() == 0 {
            return Ok(());
        }

        // Pad with zero bits, so we can drain the partial chunk.
        let desired_fill_level = self
            .cache
            .fill_level()
            .next_multiple_of(MCUCacheStorage::BITWIDTH);
        let padding_bitlen = desired_fill_level
            .checked_sub(self.cache.fill_level())
            .unwrap();

        let bits = BitSeq::new(BitLen::new(padding_bitlen), 0).unwrap();
        self.put(bits)?;

        let num_chunks = <_ as CheckedDivExact>::checked_div_exact(
            self.cache.fill_level(),
            MCUCacheStorage::BITWIDTH,
        )
        .unwrap();

        for _ in 0..num_chunks {
            self.drain_priv::<MCUCacheStorage>()?;
        }

        self.writer.flush()
    }

    #[inline]
    pub fn flush(self) -> std::io::Result<()>
    where
        <T as BitStreamTraits>::MCUByteArrayType: ConcatBytesNe,
        <<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output: BitStreamCacheData
            + ToNeBytes<Output = <T as BitStreamTraits>::MCUByteArrayType>
            + TryFrom<u64>
            + SwapBytes,
        <<<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output as ToNeBytes>::Output:
            AsSlice<EltType = u8>,
        <<<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output as TryFrom<u64>>::Error: core::fmt::Debug
    {
        self.flush_impl::<<<T as BitStreamTraits>::MCUByteArrayType as ConcatBytesNe>::Output>()
    }

    #[inline]
    pub fn drain_priv<CacheStorage>(&mut self) -> std::io::Result<()>
    where
        CacheStorage: BitStreamCacheData
            + From<<T::MCUByteArrayType as ConcatBytesNe>::Output>
            + TryFrom<u64>
            + ToNeBytes
            + SwapBytes,
        <CacheStorage as ToNeBytes>::Output: AsSlice<EltType = u8>,
        <CacheStorage as TryFrom<u64>>::Error: core::fmt::Debug,
    {
        if self.cache.fill_level() < CacheStorage::BITWIDTH {
            return Ok(()); // NOTE: does not mean the cache is empty!
        }

        BitVacuumerDrainImpl::<T>::drain_impl::<CacheStorage>(self)?;
        Ok(())
    }

    #[inline]
    pub fn drain(&mut self) -> std::io::Result<()> {
        self.drain_priv::<u32>()
    }

    #[inline]
    pub fn put(&mut self, bits: BitSeq<u64>) -> std::io::Result<()> {
        // NOTE: count may be zero!
        self.drain()?;
        self.cache.push(bits.into());
        Ok(())
    }
}

impl<T, W> Drop for BitVacuumerBase<'_, T, W>
where
    T: BitOrderTrait + BitStreamTraits,
    <T as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    W: std::io::Write,
{
    #[inline]
    fn drop(&mut self) {
        const ERR: &str = "Unrecoverable Error: trying to drop \
            non-empty BitVacuumer. Did you forget to call `flush()`?";
        assert!(self.cache.fill_level() == 0, "{}", ERR);
    }
}

pub mod jpeg;
pub mod lsb;
pub mod msb;
pub mod msb16;
pub mod msb32;
