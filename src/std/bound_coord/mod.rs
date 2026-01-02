macro_rules! wrap {
    ($ty:ident ($dty:ident) as $bty:ident) => {
        use crate::coord_common::{$dty, $ty};

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $bty {
            domain: $dty,
            value: $ty,
        }

        impl $bty {
            #[inline]
            #[must_use]
            pub fn new(domain: $dty, value: $ty) -> Option<Self> {
                if value < domain {
                    return Some(Self { domain, value });
                }
                None
            }

            #[inline]
            #[must_use]
            pub const fn value(&self) -> $ty {
                self.value
            }

            #[inline]
            #[must_use]
            pub const fn domain(&self) -> $dty {
                self.domain
            }
        }

        impl core::ops::Deref for $bty {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };
}

wrap!(RowIndex(RowCount) as BoundRowIndex);
wrap!(ColIndex(RowLength) as BoundColIndex);

#[cfg(test)]
mod tests;
