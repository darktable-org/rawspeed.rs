macro_rules! impl_binary_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, divisor: Self) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = Option<Self>;
                #[inline]
                fn $method(self, divisor: Self) -> Self::Output {
                    let quotent = self.checked_div(divisor)?;
                    let maybe_divident = quotent.checked_mul(divisor).unwrap();
                    if maybe_divident == self {
                        Some(quotent)
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

impl_binary_op!(checked_div_exact as CheckedDivExact for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[cfg(test)]
mod tests;
