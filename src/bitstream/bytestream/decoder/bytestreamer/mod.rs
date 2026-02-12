use rawspeed_common_generic_num::generic_num::bit_transmutation::{
    FromBits, FromNeBytes, ToBits, ToNeBytes,
};
use rawspeed_memory_endianness::endianness::{
    Endianness, SwapBytes, get_host_endianness,
};
use rawspeed_memory_fixed_length_load::fixed_length_load::{
    CopyFromSlice, LoadFromSlice,
};

pub struct ByteStreamer<'a> {
    slice: &'a [u8],
    endianness: Endianness,
}

#[cfg_attr(not(test), expect(dead_code))]
impl<'a> ByteStreamer<'a> {
    pub const fn new(slice: &'a [u8], endianness: Endianness) -> Self {
        Self { slice, endianness }
    }

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
#[expect(clippy::cast_sign_loss)]
#[expect(clippy::float_cmp)]
mod tests;
