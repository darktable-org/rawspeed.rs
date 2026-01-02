use crate::{
    bound_numerics::{Bound, BoundUnsigned},
    wrapping_numerics::WrappingUnsigned,
};

macro_rules! wrap {
    ($wty:ident, $bty:ident, $ty:ident, $U:ident) => {
        use crate::{
            bound_coord::$bty,
            coord_common::{$U, $ty},
            wrapping_coord::$wty,
        };

        impl core::ops::Add<$U> for $wty
        where
            WrappingUnsigned<usize>:
                core::ops::Add<isize, Output = WrappingUnsigned<usize>>,
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $U) -> Self::Output {
                let lhs = WrappingUnsigned::new(
                    BoundUnsigned::new(
                        Bound::new(*self.domain()).unwrap(),
                        *self.value(),
                    )
                    .unwrap(),
                );
                let res = lhs + *rhs;
                $wty::new($bty::new(self.domain(), $ty::new(**res)).unwrap())
            }
        }
    };
}

wrap!(WrappingRowIndex, BoundRowIndex, RowIndex, RowOffset);
wrap!(WrappingColIndex, BoundColIndex, ColIndex, ColOffset);

#[cfg(test)]
mod tests;
