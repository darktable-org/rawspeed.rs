#[derive(Debug, Copy, Clone, PartialEq)]
#[expect(clippy::exhaustive_enums)]
pub enum Endianness {
    Little,
    Big,
}

#[inline]
#[must_use]
pub const fn get_host_endianness() -> Endianness {
    #[cfg(target_endian = "little")]
    {
        Endianness::Little
    }
    #[cfg(target_endian = "big")]
    {
        Endianness::Big
    }
}

pub trait SwapBytes {
    #[must_use]
    fn swap_bytes(self) -> Self;

    #[must_use]
    fn get_byte_swapped(self, cond: bool) -> Self;
}

macro_rules! impl_swap_bytes {
    ($($t:ty)+) => {
        $(
            impl SwapBytes for $t {
                #[inline]
                fn swap_bytes(self) -> Self {
                    self.swap_bytes()
                }

                #[inline]
                fn get_byte_swapped(self, cond: bool) -> Self {
                    if !cond {
                        self
                    } else {
                        <$t>::swap_bytes(self)
                    }
                }
            }
        )+
    };
}

impl_swap_bytes!(u8 u16 u32 u64);

#[cfg(test)]
mod tests;
