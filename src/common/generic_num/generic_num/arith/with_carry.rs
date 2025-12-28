macro_rules! impl_ternary_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, rhs: Self, carry: bool) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = (Self, bool);
                #[inline]
                fn $method(self, rhs: Self, carry: bool) -> Self::Output {
                    <Self>::$method(self, rhs, carry)
                }
            }
        )+
    };
}

impl_ternary_op!(carrying_add as CarryingAdd for u8 u16 u32 u64);
impl_ternary_op!(borrowing_sub as BorrowingSub for u8 u16 u32 u64);
