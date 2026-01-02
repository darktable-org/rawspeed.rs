macro_rules! wrap {
    ($bty:ident as $wty:ident) => {
        use crate::bound_coord::$bty;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $wty {
            value: $bty,
        }

        impl $wty {
            #[inline]
            #[must_use]
            pub const fn new(value: $bty) -> Self {
                Self { value }
            }
        }

        impl From<$bty> for $wty {
            #[inline]
            fn from(value: $bty) -> $wty {
                Self::new(value)
            }
        }

        impl core::ops::Deref for $wty {
            type Target = $bty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };
}

wrap!(BoundRowIndex as WrappingRowIndex);
wrap!(BoundColIndex as WrappingColIndex);
