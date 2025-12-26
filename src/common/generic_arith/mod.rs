macro_rules! impl_binary_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, other: Self) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = Self;
                #[inline]
                fn $method(self, other: Self) -> Self {
                    <Self>::$method(self, other)
                }
            }
        )+
    };
}

impl_binary_op!(wrapping_add as WrappingAdd for u8 u16 u32 u64);
impl_binary_op!(wrapping_rem as WrappingRem for u8 u16 u32 u64);
