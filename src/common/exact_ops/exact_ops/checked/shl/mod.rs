macro_rules! impl_binary_op {
    ($method:ident as $trait:ident for $($t:ty)+) => {
        pub trait $trait {
            type Output;
            #[must_use]
            fn $method(self, rhs: u32) -> Self::Output;
        }
        $(
            impl $trait for $t {
                type Output = Option<Self>;
                #[inline]
                fn $method(self, rhs: u32) -> Self::Output {
                    let res = self.checked_shl(rhs)?;
                    let maybe_shiftee = res.checked_shr(rhs).unwrap();
                    if maybe_shiftee == self {
                        Some(res)
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

impl_binary_op!(checked_shl_exact as CheckedShlExact for u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[cfg(test)]
mod tests;
