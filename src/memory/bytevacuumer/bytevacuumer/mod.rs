use rawspeed_common::bit_transmutation::ToBits;
use rawspeed_common::bit_transmutation::ToNeBytes;
use rawspeed_memory_endianness::endianness::Endianness;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;

pub struct ByteVacuumer<'a, W>
where
    W: std::io::Write,
{
    writer: &'a mut W,
    endianness: Endianness,
}

impl<'a, W> ByteVacuumer<'a, W>
where
    W: std::io::Write,
{
    #[allow(dead_code)]
    pub const fn new(writer: &'a mut W, endianness: Endianness) -> Self {
        Self { writer, endianness }
    }

    #[allow(dead_code)]
    fn write<T>(&mut self, val: T) -> std::io::Result<()>
    where
        T: ToBits,
        <T as ToBits>::Output: SwapBytes + ToNeBytes,
        <<T as ToBits>::Output as ToNeBytes>::Output:
            core::ops::Index<core::ops::RangeFull, Output = [u8]>,
    {
        let val = val.to_bits();
        let val =
            val.get_byte_swapped(get_host_endianness() != self.endianness);
        let bytes = val.to_ne_bytes();
        self.writer.write_all(&bytes[..])
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
#[allow(clippy::cast_sign_loss)]
mod tests;
