use crate::{
    bound_coord::BoundRowIndex,
    bound_numerics::{Bound, BoundUnsigned},
    coord_common::{RowIndex, RowOffset},
    wrapping_coord::WrappingRowIndex,
    wrapping_numerics::WrappingUnsigned,
};

impl core::ops::Add<RowOffset> for WrappingRowIndex
where
    WrappingUnsigned<usize>:
        core::ops::Add<isize, Output = WrappingUnsigned<usize>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: RowOffset) -> Self::Output {
        let lhs = WrappingUnsigned::new(
            BoundUnsigned::new(
                Bound::new(*self.domain()).unwrap(),
                *self.value(),
            )
            .unwrap(),
        );
        let res = lhs + *rhs;
        WrappingRowIndex::new(
            BoundRowIndex::new(self.domain(), RowIndex::new(**res)).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests;
