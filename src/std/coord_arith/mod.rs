use crate::coord_common::{
    ColIndex, ColOffset, Coord2D, CoordOffset2D, RowIndex, RowOffset,
};

impl core::ops::Neg for RowOffset {
    type Output = Option<Self>;

    #[inline]
    fn neg(self) -> Self::Output {
        Some(Self::new((*self).checked_neg()?))
    }
}

impl core::ops::Sub<Self> for RowIndex {
    type Output = Option<RowOffset>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs: isize = (*self).try_into().ok()?;
        let rhs = *rhs;
        Some(RowOffset::new(lhs.checked_sub_unsigned(rhs)?))
    }
}

impl core::ops::Sub<RowOffset> for RowIndex {
    type Output = Option<Self>;

    #[inline]
    fn sub(self, rhs: RowOffset) -> Self::Output {
        let lhs = *self;
        let rhs = (*rhs).try_into().ok()?;
        Some(Self::new(lhs.checked_sub(rhs)?))
    }
}

impl core::ops::Neg for ColOffset {
    type Output = Option<Self>;

    #[inline]
    fn neg(self) -> Self::Output {
        Some(Self::new((*self).checked_neg()?))
    }
}

impl core::ops::Sub<Self> for ColIndex {
    type Output = Option<ColOffset>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs: isize = (*self).try_into().ok()?;
        let rhs = *rhs;
        Some(ColOffset::new(lhs.checked_sub_unsigned(rhs)?))
    }
}

impl core::ops::Sub<ColOffset> for ColIndex {
    type Output = Option<Self>;

    #[inline]
    fn sub(self, rhs: ColOffset) -> Self::Output {
        let lhs = *self;
        let rhs = (*rhs).try_into().ok()?;
        Some(Self::new(lhs.checked_sub(rhs)?))
    }
}

impl core::ops::Neg for CoordOffset2D {
    type Output = Option<Self>;

    #[inline]
    fn neg(self) -> Self::Output {
        let row = self.row().neg();
        let col = self.col().neg();
        Some(Self::new(row?, col?))
    }
}

impl core::ops::Sub<Self> for Coord2D {
    type Output = Option<CoordOffset2D>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self;
        let row = lhs.row() - rhs.row();
        let col = lhs.col() - rhs.col();
        Some(CoordOffset2D::new(row?, col?))
    }
}

impl core::ops::Sub<CoordOffset2D> for Coord2D {
    type Output = Option<Self>;

    #[inline]
    fn sub(self, rhs: CoordOffset2D) -> Self::Output {
        let lhs = self;
        let row = lhs.row() - rhs.row();
        let col = lhs.col() - rhs.col();
        Some(Self::new(row?, col?))
    }
}

#[cfg(test)]
mod tests;
