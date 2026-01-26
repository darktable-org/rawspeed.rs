pub trait CastUnsigned {
    type Output;
    #[must_use]
    fn cast_unsigned(self) -> Self::Output;
}

pub trait CastSigned {
    type Output;
    #[must_use]
    fn cast_signed(self) -> Self::Output;
}

macro_rules! impl_signcast {
    ($(($u:ty, $i:ty)$(,)?)+) => {
        $(
            impl CastUnsigned for $u {
                type Output = Self;
                #[inline]
                fn cast_unsigned(self) -> Self::Output {
                    self
                }
            }
            impl CastSigned for $i {
                type Output = Self;
                #[inline]
                fn cast_signed(self) -> Self::Output {
                    self
                }
            }
            impl CastUnsigned for $i {
                type Output = $u;
                #[inline]
                fn cast_unsigned(self) -> Self::Output {
                    self.cast_unsigned()
                }
            }
            impl CastSigned for $u {
                type Output = $i;
                #[inline]
                fn cast_signed(self) -> Self::Output {
                    self.cast_signed()
                }
            }
        )+
    };
}

impl_signcast!((u8, i8), (u16, i16), (u32, i32), (u64, i64), (usize, isize));

pub trait Integer: Sized + Bitwidth + ConstZero {}

macro_rules! impl_simple_trait {
    (impl $tr:ident for $($t:ty$(,)?)+) => {
        $(
            impl $tr for $t {}
        )+
    };
}

impl_simple_trait!(impl Integer for u8, u16, u32, u64, usize);

pub trait Bitwidth {
    const BITWIDTH: usize;
}

macro_rules! impl_bitwidth {
    ($($t:ty)+) => {
        $(
            impl Bitwidth for $t {
                const BITWIDTH: usize = <$t>::BITS as usize;
            }
        )+
    };
}

impl_bitwidth!(u8 u16 u32 u64 usize);

pub trait Max {
    const MAX: Self;
}

macro_rules! impl_max {
    ($($t:ty)+) => {
        $(
            impl Max for $t {
                const MAX: $t = <$t>::MAX;
            }
        )+
    };
}

impl_max!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

pub trait Min {
    const MIN: Self;
}

macro_rules! impl_min {
    ($($t:ty)+) => {
        $(
            impl Min for $t {
                const MIN: $t = <$t>::MIN;
            }
        )+
    };
}

impl_min!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

macro_rules! impl_literal_constant {
    (trait $trait:ident with $name:ident = $value:literal for $($t:ty)+) => {
        pub trait $trait {
            const $name: Self;
        }
        $(
            impl $trait for $t {
                const $name: Self = $value;
            }
        )+
    };
}

impl_literal_constant!(trait ConstZero with ZERO = 0 for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
impl_literal_constant!(trait ConstOne with ONE = 1 for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

pub trait IsMultipleOf {
    #[must_use]
    #[expect(clippy::wrong_self_convention)]
    fn is_multiple_of(self, other: Self) -> bool;
}

macro_rules! impl_is_multiple_of {
    ($($t:ty)+) => {
        $(
            impl IsMultipleOf for $t {
                #[inline]
                fn is_multiple_of(self, other: Self) -> bool {
                    self.is_multiple_of(other)
                }
            }
        )+
    };
}

impl_is_multiple_of!(u8 u16 u32 u64 usize);

pub trait TrailingZeros {
    #[must_use]
    fn trailing_zeros(self) -> u32;
}

macro_rules! impl_trailing_zeros {
    ($($t:ty)+) => {
        $(
            impl TrailingZeros for $t {
                #[inline]
                fn trailing_zeros(self) -> u32 {
                    self.trailing_zeros()
                }
            }
        )+
    };
}

impl_trailing_zeros!(u8 u16 u32 u64 usize);

#[cfg(test)]
mod tests;
