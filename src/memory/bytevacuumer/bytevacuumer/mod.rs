use rawspeed_memory_endianness::endianness::Endianness;
use rawspeed_memory_endianness::endianness::SwapBytes;
use rawspeed_memory_endianness::endianness::get_host_endianness;

pub trait ToBits {
    type Output;
    fn to_bits(self) -> Self::Output;
}

macro_rules! impl_to_bits {
    ($src:ty, $tgt:ty) => {
        impl ToBits for $src {
            type Output = $tgt;
            fn to_bits(self) -> Self::Output {
                Self::Output::from_ne_bytes(self.to_ne_bytes())
            }
        }
    };
}

impl_to_bits!(u8, u8);
impl_to_bits!(i8, u8);
impl_to_bits!(u16, u16);
impl_to_bits!(i16, u16);
impl_to_bits!(u32, u32);
impl_to_bits!(i32, u32);
impl_to_bits!(u64, u64);
impl_to_bits!(i64, u64);

impl_to_bits!(f32, u32);
impl_to_bits!(f64, u64);

pub trait ToNeBytes {
    type Output;
    fn to_ne_bytes(self) -> Self::Output;
}

impl ToNeBytes for u8 {
    type Output = [u8; 1];

    fn to_ne_bytes(self) -> Self::Output {
        self.to_ne_bytes()
    }
}

impl ToNeBytes for u16 {
    type Output = [u8; 2];

    fn to_ne_bytes(self) -> Self::Output {
        self.to_ne_bytes()
    }
}

impl ToNeBytes for u32 {
    type Output = [u8; 4];

    fn to_ne_bytes(self) -> Self::Output {
        self.to_ne_bytes()
    }
}

impl ToNeBytes for u64 {
    type Output = [u8; 8];

    fn to_ne_bytes(self) -> Self::Output {
        self.to_ne_bytes()
    }
}

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
