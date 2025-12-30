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
impl_binary_op!(checked_rem as CheckedRem for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
