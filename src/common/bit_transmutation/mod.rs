pub trait ToBits {
    type Output;
    fn to_bits(self) -> Self::Output;
}

macro_rules! impl_to_bits {
    ($src:ty, $tgt:ty) => {
        impl ToBits for $src {
            type Output = $tgt;

            #[inline]
            fn to_bits(self) -> Self::Output {
                const {
                    assert!(
                        core::mem::size_of::<$src>()
                            == core::mem::size_of::<$tgt>()
                    );
                    assert!(<$tgt>::MIN == 0);
                }
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

    #[allow(clippy::wrong_self_convention)]
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
