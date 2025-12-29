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

pub trait ConstZero {
    const ZERO: Self;
}

macro_rules! impl_constzero {
    ($($t:ty)+) => {
        $(
            impl ConstZero for $t {
                const ZERO: Self = 0;
            }
        )+
    };
}
impl_constzero!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[cfg(test)]
mod tests;
