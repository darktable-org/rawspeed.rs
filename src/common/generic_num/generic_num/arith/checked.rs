macro_rules! impl_binary_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, other: Self) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = Option<Self>;
                #[inline]
                fn $method(self, other: Self) -> Self::Output {
                    <Self>::$method(self, other)
                }
            }
        )+
    };
}

impl_binary_op!(checked_add as CheckedAdd for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
impl_binary_op!(checked_sub as CheckedSub for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
impl_binary_op!(checked_mul as CheckedMul for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
impl_binary_op!(checked_rem as CheckedRem for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

macro_rules! impl_shift_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, amt: u32) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = Option<Self>;
                #[inline]
                fn $method(self, amt: u32) -> Self::Output {
                    <Self>::$method(self, amt)
                }
            }
        )+
    };
}
impl_shift_op!(checked_shl as CheckedShl for u8 u16 u32 u64 usize);
impl_shift_op!(checked_shr as CheckedShr for u8 u16 u32 u64 usize);
