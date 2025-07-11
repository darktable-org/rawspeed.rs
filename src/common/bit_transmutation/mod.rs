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

macro_rules! impl_to_ne_bytes {
    ($src:ty) => {
        impl ToNeBytes for $src {
            type Output = [u8; core::mem::size_of::<$src>()];

            #[inline]
            fn to_ne_bytes(self) -> Self::Output {
                self.to_ne_bytes()
            }
        }
    };
}

impl_to_ne_bytes!(u8);
impl_to_ne_bytes!(u16);
impl_to_ne_bytes!(u32);
impl_to_ne_bytes!(u64);

pub trait CopyFromSlice {
    fn copy_from_slice_(&mut self, src: &[u8]);
}

impl CopyFromSlice for [u8] {
    #[inline]
    fn copy_from_slice_(&mut self, src: &[u8]) {
        self.copy_from_slice(src);
    }
}

pub trait LoadFromSlice<T>
where
    T: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <T as core::ops::Index<core::ops::RangeFull>>::Output: CopyFromSlice,
{
    fn load_from_slice(&self) -> T;
}

impl<T> LoadFromSlice<T> for [u8]
where
    T: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <T as core::ops::Index<core::ops::RangeFull>>::Output: CopyFromSlice,
{
    #[inline]
    fn load_from_slice(&self) -> T {
        let mut out: T = Default::default();
        out[..].copy_from_slice_(self);
        out
    }
}

pub trait FromNeBytes {
    type Output;

    #[expect(clippy::wrong_self_convention)]
    fn from_ne_bytes(self) -> Self::Output;
}

macro_rules! impl_from_ne_bytes {
    ($tgt:ty) => {
        impl FromNeBytes for [u8; core::mem::size_of::<$tgt>()] {
            type Output = $tgt;

            #[inline]
            fn from_ne_bytes(self) -> Self::Output {
                Self::Output::from_ne_bytes(self)
            }
        }
    };
}

impl_from_ne_bytes!(u8);
impl_from_ne_bytes!(u16);
impl_from_ne_bytes!(u32);
impl_from_ne_bytes!(u64);
