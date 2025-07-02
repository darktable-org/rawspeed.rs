use rawspeed_common::bit_transmutation::CopyFromSlice;
use rawspeed_common::bit_transmutation::FromBits;
use rawspeed_common::bit_transmutation::FromNeBytes;
use rawspeed_common::bit_transmutation::LoadFromSlice;
use rawspeed_common::bit_transmutation::ToBits;
use rawspeed_common::bit_transmutation::ToNeBytes;
use rawspeed_memory_endianness::endianness::Endianness;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;

pub struct ByteStreamer<'a> {
    slice: &'a [u8],
    endianness: Endianness,
}

impl<'a> ByteStreamer<'a> {
    #[allow(dead_code)]
    pub const fn new(slice: &'a [u8], endianness: Endianness) -> Self {
        Self { slice, endianness }
    }

    #[allow(dead_code)]
    pub fn read<T>(&mut self) -> T
    where
        T: ToBits + FromBits<<T as ToBits>::Output, Output = T>,
        <T as ToBits>::Output: ToNeBytes + SwapBytes,
        <<T as ToBits>::Output as ToNeBytes>::Output: Default
            + FromNeBytes<Output = <T as ToBits>::Output>
            + core::ops::Index<core::ops::RangeFull>
            + core::ops::IndexMut<core::ops::RangeFull>,
        <<<T as ToBits>::Output as ToNeBytes>::Output as core::ops::Index<
            core::ops::RangeFull,
        >>::Output: CopyFromSlice,
        <T as FromBits<<T as ToBits>::Output>>::BitsTy:
            From<<T as ToBits>::Output>,
    {
        let size: usize = size_of::<T>();
        let (slice, rest) = self.slice.split_at_checked(size).unwrap();
        self.slice = rest;

        let bytes =
            LoadFromSlice::<<<T as ToBits>::Output as ToNeBytes>::Output>::load_from_slice(slice);
        let val = bytes.from_ne_bytes();
        let val =
            val.get_byte_swapped(get_host_endianness() != self.endianness);
        T::from_bits(val.into())
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::float_cmp)]
mod tests;
