use rawspeed_common_generic_num::generic_num::bit_transmutation::{
    ConcatBytesNe, FromBits, ToNeBytes,
};
use rawspeed_memory_endianness::endianness::{
    Endianness, SwapBytes, get_host_endianness,
};

#[non_exhaustive]
#[must_use]
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
        T: FromBits,
        <T as FromBits>::BitsTy: ToNeBytes + SwapBytes,
        for<'b> <<T as FromBits>::BitsTy as ToNeBytes>::Output:
            TryFrom<&'b [u8]> + ConcatBytesNe<Output = <T as FromBits>::BitsTy>,
        for<'b> <<<T as FromBits>::BitsTy as ToNeBytes>::Output as TryFrom<&'b [u8]>>::Error: core::fmt::Debug,
    {
        let size: usize = size_of::<T>();
        let (slice, rest) = self.slice.split_at_checked(size).unwrap();
        self.slice = rest;

        let bytes: <<T as FromBits>::BitsTy as ToNeBytes>::Output =
            slice.try_into().unwrap();
        let val = bytes.concat_bytes_ne();
        let val =
            val.get_byte_swapped(get_host_endianness() != self.endianness);
        T::from_bits(val)
    }
}

#[cfg(test)]
#[expect(clippy::cast_sign_loss)]
#[expect(clippy::float_cmp)]
mod tests;
