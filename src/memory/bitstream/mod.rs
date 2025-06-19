pub mod bitstreamflow;

pub mod bitstream {
    use super::bitstreamflow::Bitwidth;
    use crate::bitstreamflow::BitStreamCache;
    use rawspeed_memory_endianness::endianness::Endianness;
    use rawspeed_memory_endianness::endianness::SwapBytes;
    use rawspeed_memory_endianness::endianness::get_byte_swapped;
    use std::marker::PhantomData;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum BitOrder {
        LSB,
        MSB,
        MSB32,
    }

    pub trait BitOrderTrait {}

    pub trait BitStreamTraits<T: BitOrderTrait> {
        const TAG: BitOrder;
        type StreamFlow;
        const FIXED_SIZE_CHUNKS: bool;
        type ChunkType;
        const CHUNK_ENDIANNESS: Endianness;
        const MIN_LOAD_STEP_BYTE_MULTIPLE: u32;
    }

    //--------------------------------------------------------------------------

    pub struct BitOrderLSB;

    impl BitOrderTrait for BitOrderLSB {}

    impl BitStreamTraits<BitOrderLSB> for BitOrderLSB {
        const TAG: BitOrder = BitOrder::LSB;

        type StreamFlow = super::bitstreamflow::BitStreamCacheHighInLowOut;

        const FIXED_SIZE_CHUNKS: bool = true;

        type ChunkType = u32;

        const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

        const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1;
    }

    pub type BitVacuumerLSB<'a, W> = BitVacuumerBase<'a, BitOrderLSB, W>;

    //--------------------------------------------------------------------------

    pub struct BitOrderMSB;

    impl BitOrderTrait for BitOrderMSB {}

    impl BitStreamTraits<BitOrderMSB> for BitOrderMSB {
        const TAG: BitOrder = BitOrder::MSB;

        type StreamFlow = super::bitstreamflow::BitStreamCacheLowInHighOut;

        const FIXED_SIZE_CHUNKS: bool = true;

        type ChunkType = u32;

        const CHUNK_ENDIANNESS: Endianness = Endianness::Big;

        const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1;
    }

    pub type BitVacuumerMSB<'a, W> = BitVacuumerBase<'a, BitOrderMSB, W>;

    //--------------------------------------------------------------------------

    pub struct BitOrderMSB32;

    impl BitOrderTrait for BitOrderMSB32 {}

    impl BitStreamTraits<BitOrderMSB32> for BitOrderMSB32 {
        const TAG: BitOrder = BitOrder::MSB32;

        type StreamFlow = super::bitstreamflow::BitStreamCacheLowInHighOut;

        const FIXED_SIZE_CHUNKS: bool = true;

        type ChunkType = u32;

        const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

        const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 4;
    }

    pub type BitVacuumerMSB32<'a, W> = BitVacuumerBase<'a, BitOrderMSB32, W>;

    //--------------------------------------------------------------------------

    pub struct BitVacuumerBase<'a, T, W>
    where
        T: BitOrderTrait + BitStreamTraits<T>,
        W: std::io::Write,
        T::StreamFlow: super::bitstreamflow::BitStreamCache,
        T::ChunkType: Bitwidth + SwapBytes<T::ChunkType> + TryFrom<u64>,
        u32: std::ops::BitOrAssign<T::ChunkType>,
    {
        cache: T::StreamFlow,
        writer: &'a mut W,
        _phantom_data: PhantomData<T>,
    }

    pub trait BitVacuumerTraits<T: BitOrderTrait> {
        type WritebackCache;

        fn drain_impl(&mut self) -> std::io::Result<()>;
    }

    impl<'a, T, W> BitVacuumerTraits<T> for BitVacuumerBase<'a, T, W>
    where
        T: BitOrderTrait + BitStreamTraits<T>,
        W: std::io::Write,
        T::StreamFlow: super::bitstreamflow::BitStreamCache,
        T::ChunkType: Bitwidth + SwapBytes<T::ChunkType> + TryFrom<u64>,
        u32: std::ops::BitOrAssign<T::ChunkType>,
    {
        type WritebackCache = u32;

        fn drain_impl(&mut self) -> std::io::Result<()> {
            assert!(self.cache.fill_level() >= Self::WritebackCache::BITWIDTH);

            let stream_chunk_bitwidth: usize = T::ChunkType::BITWIDTH;

            assert!(Self::WritebackCache::BITWIDTH >= stream_chunk_bitwidth);
            assert!(
                Self::WritebackCache::BITWIDTH % stream_chunk_bitwidth == 0
            );
            let num_chunks_needed: usize =
                Self::WritebackCache::BITWIDTH / stream_chunk_bitwidth;
            assert!(num_chunks_needed >= 1);

            let mut cache: Self::WritebackCache = 0;
            for _i in 0..num_chunks_needed {
                let chunk: T::ChunkType = match <T::ChunkType>::try_from(
                    self.cache.peek(stream_chunk_bitwidth),
                ) {
                    Ok(t) => t,
                    Err(_) => panic!("lossless cast failed?"),
                };
                self.cache.skip(stream_chunk_bitwidth);
                let chunk: T::ChunkType = get_byte_swapped(
                    chunk,
                    T::CHUNK_ENDIANNESS != Endianness::Little,
                );
                if num_chunks_needed != 1 {
                    cache <<= stream_chunk_bitwidth;
                }
                cache |= chunk;
            }
            let bytes = cache.to_le_bytes();
            self.writer.write_all(&bytes)
        }
    }

    impl<'a, T, W> BitVacuumerBase<'a, T, W>
    where
        T: BitOrderTrait + BitStreamTraits<T>,
        W: std::io::Write,
        T::StreamFlow: super::bitstreamflow::BitStreamCache,
        T::ChunkType: Bitwidth + SwapBytes<T::ChunkType> + TryFrom<u64>,
        u32: std::ops::BitOrAssign<T::ChunkType>,
    {
        #[allow(dead_code)]
        pub fn new(writer: &'a mut W) -> Self
        where
            T::StreamFlow: Default,
        {
            Self {
                cache: Default::default(),
                writer,
                _phantom_data: PhantomData,
            }
        }

        pub fn flush(mut self) -> std::io::Result<()> {
            self.drain()?;

            if self.cache.fill_level() == 0 {
                return Ok(());
            }

            // Pad with zero bits, so we can drain the partial chunk.
            self.put(
                /*bits=*/ 0,
                <Self as BitVacuumerTraits<T>>::WritebackCache::BITWIDTH
                    - self.cache.fill_level(),
            )?;
            assert!(
                self.cache.fill_level()
                    == <Self as BitVacuumerTraits<T>>::WritebackCache::BITWIDTH
            );

            self.drain()?;
            assert!(self.cache.fill_level() == 0);

            Ok(())
        }

        pub fn drain(&mut self) -> std::io::Result<()> {
            if self.cache.fill_level()
                < <Self as BitVacuumerTraits<T>>::WritebackCache::BITWIDTH
            {
                return Ok(()); // NOTE: does not mean the cache is empty!
            }

            self.drain_impl()?;
            assert!(
                self.cache.fill_level()
                    < <Self as BitVacuumerTraits<T>>::WritebackCache::BITWIDTH
            );
            Ok(())
        }

        pub fn put(&mut self, bits: u64, count: usize) -> std::io::Result<()> {
            // NOTE: count may be zero!
            self.drain()?;
            self.cache.push(bits, count);
            Ok(())
        }
    }

    impl<'a, T, W> Drop for BitVacuumerBase<'a, T, W>
    where
        T: BitOrderTrait + BitStreamTraits<T>,
        W: std::io::Write,
        T::StreamFlow: super::bitstreamflow::BitStreamCache,
        T::ChunkType: Bitwidth + SwapBytes<T::ChunkType> + TryFrom<u64>,
        u32: std::ops::BitOrAssign<T::ChunkType>,
    {
        fn drop(&mut self) {
            let err: &'static str = "Unrecoverable Error: trying to drop \
            non-empty BitVacuumer. Did you forget to call `flush()`?";
            if self.cache.fill_level() != 0 {
                panic!("{}", err)
            }
        }
    }

    //--------------------------------------------------------------------------
}

#[cfg(test)]
mod tests_lsb {
    use core::result::Result::Ok;
    use std::io::Write;

    use super::bitstream::*;

    #[test]
    fn vec_ctor_test() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let _vac = BitVacuumerLSB::new(&mut buf);
    }

    #[test]
    fn arr_ctor_test() {
        use std::io::Cursor;
        let mut buf = [0u8; 1024];
        let mut buf = Cursor::new(buf.as_mut());
        let _vac = BitVacuumerLSB::new(&mut buf);
    }

    #[test]
    fn drop_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerLSB::new(&mut buf);
        drop(vac);
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    fn flush_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerLSB::new(&mut buf);
        vac.flush()?;
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "Unrecoverable Error: trying to drop non-empty BitVacuumer. Did you forget to call `flush()`?"
    )]
    fn dropping_unflushed_vac_byte() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        match vac.put(0, 1) {
            Ok(_) => (),
            Err(_) => panic!("unexpected panic"),
        }
        drop(vac);
    }

    #[test]
    fn flush_arr_overflow_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = [0u8; 0];
        let mut buf = Cursor::new(buf.as_mut());
        let mut vac = BitVacuumerLSB::new(&mut buf);
        vac.put(0, 1)?;
        assert!(vac.flush().is_err());
        Ok(())
    }

    #[test]
    fn byte_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_bytes in 0..(8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerLSB::new(&mut buf);
            for i in 0..num_bytes {
                vac.put(1 + i, 8)?;
            }
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![],
            vec![1, 0, 0, 0],
            vec![1, 2, 0, 0],
            vec![1, 2, 3, 0],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5, 0, 0, 0],
            vec![1, 2, 3, 4, 5, 6, 0, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn bit_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..32 {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerLSB::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(1, 1)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![1, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![16, 0, 0, 0],
            vec![32, 0, 0, 0],
            vec![64, 0, 0, 0],
            vec![128, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 2, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 8, 0, 0],
            vec![0, 16, 0, 0],
            vec![0, 32, 0, 0],
            vec![0, 64, 0, 0],
            vec![0, 128, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 4, 0],
            vec![0, 0, 8, 0],
            vec![0, 0, 16, 0],
            vec![0, 0, 32, 0],
            vec![0, 0, 64, 0],
            vec![0, 0, 128, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 16],
            vec![0, 0, 0, 32],
            vec![0, 0, 0, 64],
            vec![0, 0, 0, 128],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn sliding_0xff_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..(32 - 8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerLSB::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(0xFF, 8)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![255, 0, 0, 0],
            vec![254, 1, 0, 0],
            vec![252, 3, 0, 0],
            vec![248, 7, 0, 0],
            vec![240, 15, 0, 0],
            vec![224, 31, 0, 0],
            vec![192, 63, 0, 0],
            vec![128, 127, 0, 0],
            vec![0, 255, 0, 0],
            vec![0, 254, 1, 0],
            vec![0, 252, 3, 0],
            vec![0, 248, 7, 0],
            vec![0, 240, 15, 0],
            vec![0, 224, 31, 0],
            vec![0, 192, 63, 0],
            vec![0, 128, 127, 0],
            vec![0, 0, 255, 0],
            vec![0, 0, 254, 1],
            vec![0, 0, 252, 3],
            vec![0, 0, 248, 7],
            vec![0, 0, 240, 15],
            vec![0, 0, 224, 31],
            vec![0, 0, 192, 63],
            vec![0, 0, 128, 127],
            vec![0, 0, 0, 255],
        ];
        assert_eq!(res, expected);
        Ok(())
    }
}

#[cfg(test)]
mod tests_msb {
    use core::result::Result::Ok;
    use std::io::Write;

    use super::bitstream::*;

    #[test]
    fn vec_ctor_test() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let _vac = BitVacuumerMSB::new(&mut buf);
    }

    #[test]
    fn arr_ctor_test() {
        use std::io::Cursor;
        let mut buf = [0u8; 1024];
        let mut buf = Cursor::new(buf.as_mut());
        let _vac = BitVacuumerMSB::new(&mut buf);
    }

    #[test]
    fn drop_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerMSB::new(&mut buf);
        drop(vac);
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    fn flush_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerMSB::new(&mut buf);
        vac.flush()?;
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "Unrecoverable Error: trying to drop non-empty BitVacuumer. Did you forget to call `flush()`?"
    )]
    fn dropping_unflushed_vac_byte() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerMSB::new(&mut buf);
        match vac.put(0, 1) {
            Ok(_) => (),
            Err(_) => panic!("unexpected panic"),
        }
        drop(vac);
    }

    #[test]
    fn flush_arr_overflow_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = [0u8; 0];
        let mut buf = Cursor::new(buf.as_mut());
        let mut vac = BitVacuumerMSB::new(&mut buf);
        vac.put(0, 1)?;
        assert!(vac.flush().is_err());
        Ok(())
    }

    #[test]
    fn byte_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_bytes in 0..(8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB::new(&mut buf);
            for i in 0..num_bytes {
                vac.put(1 + i, 8)?;
            }
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![],
            vec![1, 0, 0, 0],
            vec![1, 2, 0, 0],
            vec![1, 2, 3, 0],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5, 0, 0, 0],
            vec![1, 2, 3, 4, 5, 6, 0, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn bit_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..32 {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(1, 1)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![128, 0, 0, 0],
            vec![64, 0, 0, 0],
            vec![32, 0, 0, 0],
            vec![16, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 128, 0, 0],
            vec![0, 64, 0, 0],
            vec![0, 32, 0, 0],
            vec![0, 16, 0, 0],
            vec![0, 8, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 2, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 128, 0],
            vec![0, 0, 64, 0],
            vec![0, 0, 32, 0],
            vec![0, 0, 16, 0],
            vec![0, 0, 8, 0],
            vec![0, 0, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 128],
            vec![0, 0, 0, 64],
            vec![0, 0, 0, 32],
            vec![0, 0, 0, 16],
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 1],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn sliding_0xff_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..(32 - 8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(0xFF, 8)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![255, 0, 0, 0],
            vec![127, 128, 0, 0],
            vec![63, 192, 0, 0],
            vec![31, 224, 0, 0],
            vec![15, 240, 0, 0],
            vec![7, 248, 0, 0],
            vec![3, 252, 0, 0],
            vec![1, 254, 0, 0],
            vec![0, 255, 0, 0],
            vec![0, 127, 128, 0],
            vec![0, 63, 192, 0],
            vec![0, 31, 224, 0],
            vec![0, 15, 240, 0],
            vec![0, 7, 248, 0],
            vec![0, 3, 252, 0],
            vec![0, 1, 254, 0],
            vec![0, 0, 255, 0],
            vec![0, 0, 127, 128],
            vec![0, 0, 63, 192],
            vec![0, 0, 31, 224],
            vec![0, 0, 15, 240],
            vec![0, 0, 7, 248],
            vec![0, 0, 3, 252],
            vec![0, 0, 1, 254],
            vec![0, 0, 0, 255],
        ];
        assert_eq!(res, expected);
        Ok(())
    }
}

#[cfg(test)]
mod tests_msb32 {
    use core::result::Result::Ok;
    use std::io::Write;

    use super::bitstream::*;

    #[test]
    fn vec_ctor_test() {
        use std::io::Cursor;
        let mut buf: Cursor<_> = Cursor::new(vec![]);
        let _vac = BitVacuumerMSB32::new(&mut buf);
    }

    #[test]
    fn arr_ctor_test() {
        use std::io::Cursor;
        let mut buf = [0u8; 1024];
        let mut buf = Cursor::new(buf.as_mut());
        let _vac = BitVacuumerMSB32::new(&mut buf);
    }

    #[test]
    fn drop_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerMSB32::new(&mut buf);
        drop(vac);
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    fn flush_empty_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let vac = BitVacuumerMSB32::new(&mut buf);
        vac.flush()?;
        buf.flush()?;
        assert!(&buf.get_ref().is_empty());
        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "Unrecoverable Error: trying to drop non-empty BitVacuumer. Did you forget to call `flush()`?"
    )]
    fn dropping_unflushed_vac_byte() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerMSB32::new(&mut buf);
        match vac.put(0, 1) {
            Ok(_) => (),
            Err(_) => panic!("unexpected panic"),
        }
        drop(vac);
    }

    #[test]
    fn flush_arr_overflow_test() -> std::io::Result<()> {
        use std::io::Cursor;
        let mut buf = [0u8; 0];
        let mut buf = Cursor::new(buf.as_mut());
        let mut vac = BitVacuumerMSB32::new(&mut buf);
        vac.put(0, 1)?;
        assert!(vac.flush().is_err());
        Ok(())
    }

    #[test]
    fn byte_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_bytes in 0..(8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB32::new(&mut buf);
            for i in 0..num_bytes {
                vac.put(1 + i, 8)?;
            }
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![],
            vec![0, 0, 0, 1],
            vec![0, 0, 2, 1],
            vec![0, 3, 2, 1],
            vec![4, 3, 2, 1],
            vec![4, 3, 2, 1, 0, 0, 0, 5],
            vec![4, 3, 2, 1, 0, 0, 6, 5],
            vec![4, 3, 2, 1, 0, 7, 6, 5],
            vec![4, 3, 2, 1, 8, 7, 6, 5],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn bit_enumeration_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..32 {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB32::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(1, 1)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 128],
            vec![0, 0, 0, 64],
            vec![0, 0, 0, 32],
            vec![0, 0, 0, 16],
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 1],
            vec![0, 0, 128, 0],
            vec![0, 0, 64, 0],
            vec![0, 0, 32, 0],
            vec![0, 0, 16, 0],
            vec![0, 0, 8, 0],
            vec![0, 0, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 1, 0],
            vec![0, 128, 0, 0],
            vec![0, 64, 0, 0],
            vec![0, 32, 0, 0],
            vec![0, 16, 0, 0],
            vec![0, 8, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 2, 0, 0],
            vec![0, 1, 0, 0],
            vec![128, 0, 0, 0],
            vec![64, 0, 0, 0],
            vec![32, 0, 0, 0],
            vec![16, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn sliding_0xff_test() -> std::io::Result<()> {
        let mut res: Vec<Vec<u8>> = vec![];
        for num_leading_zeros in 0..(32 - 8 + 1) {
            use std::io::Cursor;
            let mut buf = Cursor::new(vec![]);
            let mut vac = BitVacuumerMSB32::new(&mut buf);
            for _i in 0..num_leading_zeros {
                vac.put(0, 1)?;
            }
            vac.put(0xFF, 8)?;
            vac.flush()?;
            buf.flush()?;
            res.push(buf.get_ref().clone());
        }
        let expected: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 255],
            vec![0, 0, 128, 127],
            vec![0, 0, 192, 63],
            vec![0, 0, 224, 31],
            vec![0, 0, 240, 15],
            vec![0, 0, 248, 7],
            vec![0, 0, 252, 3],
            vec![0, 0, 254, 1],
            vec![0, 0, 255, 0],
            vec![0, 128, 127, 0],
            vec![0, 192, 63, 0],
            vec![0, 224, 31, 0],
            vec![0, 240, 15, 0],
            vec![0, 248, 7, 0],
            vec![0, 252, 3, 0],
            vec![0, 254, 1, 0],
            vec![0, 255, 0, 0],
            vec![128, 127, 0, 0],
            vec![192, 63, 0, 0],
            vec![224, 31, 0, 0],
            vec![240, 15, 0, 0],
            vec![248, 7, 0, 0],
            vec![252, 3, 0, 0],
            vec![254, 1, 0, 0],
            vec![255, 0, 0, 0],
        ];
        assert_eq!(res, expected);
        Ok(())
    }
}
