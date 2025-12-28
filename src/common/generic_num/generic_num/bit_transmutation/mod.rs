pub trait ToBits {
    type Output;
    fn to_bits(self) -> Self::Output;
}

macro_rules! impl_to_bits {
    ($true_ty:ty, $bits_ty:ty) => {
        impl ToBits for $true_ty {
            type Output = $bits_ty;

            #[inline]
            fn to_bits(self) -> Self::Output {
                const {
                    assert!(
                        core::mem::size_of::<$true_ty>()
                            == core::mem::size_of::<$bits_ty>()
                    );
                    assert!(<$bits_ty>::MIN == 0);
                }
                Self::Output::from_ne_bytes(self.to_ne_bytes())
            }
        }
    };
}

pub trait FromBits<T> {
    type BitsTy;
    type Output;
    fn from_bits(bits: Self::BitsTy) -> Self::Output;
}

macro_rules! impl_from_bits {
    ($true_ty:ty, $bits_ty:ty) => {
        impl FromBits<$bits_ty> for $true_ty {
            type BitsTy = $bits_ty;
            type Output = Self;

            #[inline]
            fn from_bits(val: $bits_ty) -> Self::Output {
                const {
                    assert!(
                        core::mem::size_of::<$true_ty>()
                            == core::mem::size_of::<$bits_ty>()
                    );
                    assert!(<$bits_ty>::MIN == 0);
                }
                Self::Output::from_ne_bytes(val.to_ne_bytes())
            }
        }
    };
}

macro_rules! impl_bit_cast {
    ($true_ty:ty, $bits_ty:ty) => {
        impl_to_bits!($bits_ty, $bits_ty);
        impl_to_bits!($true_ty, $bits_ty);

        impl_from_bits!($bits_ty, $bits_ty);
        impl_from_bits!($true_ty, $bits_ty);
    };
}

impl_bit_cast!(i8, u8);
impl_bit_cast!(i16, u16);
impl_bit_cast!(i32, u32);
impl_bit_cast!(i64, u64);

impl_to_bits!(f32, u32);
impl_to_bits!(f64, u64);

impl_from_bits!(f32, u32);
impl_from_bits!(f64, u64);

pub trait ToNeBytes {
    type Output;
    fn to_ne_bytes(self) -> Self::Output;
}

pub trait ToLeBytes {
    type Output;
    fn to_le_bytes(self) -> Self::Output;
}

macro_rules! impl_to_X_bytes {
    ($trait:ident, $method:ident, $src:ty) => {
        impl $trait for $src {
            type Output = [u8; core::mem::size_of::<$src>()];

            #[inline]
            fn $method(self) -> Self::Output {
                self.$method()
            }
        }
    };
}

macro_rules! impl_to_bytes {
    ($src:ty) => {
        impl_to_X_bytes!(ToNeBytes, to_ne_bytes, $src);
        impl_to_X_bytes!(ToLeBytes, to_le_bytes, $src);
    };
}

impl_to_bytes!(u8);
impl_to_bytes!(u16);
impl_to_bytes!(u32);
impl_to_bytes!(u64);

pub trait FromNeBytes {
    type Output;

    #[expect(clippy::wrong_self_convention)]
    fn from_ne_bytes(self) -> Self::Output;
}

macro_rules! impl_from_X_bytes {
    ($trait:ident, $method:ident, $tgt:ty) => {
        impl $trait for [u8; core::mem::size_of::<$tgt>()] {
            type Output = $tgt;

            #[inline]
            fn $method(self) -> Self::Output {
                Self::Output::$method(self)
            }
        }
    };
}

macro_rules! impl_from_bytes {
    ($src:ty) => {
        impl_from_X_bytes!(FromNeBytes, from_ne_bytes, $src);
    };
}

impl_from_bytes!(u8);
impl_from_bytes!(u16);
impl_from_bytes!(u32);
impl_from_bytes!(u64);
