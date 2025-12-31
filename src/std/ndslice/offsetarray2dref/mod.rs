use rawspeed_std::bound_coord::BoundColIndex;
use rawspeed_std::bound_coord::BoundRowIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::CoordOffset2D;
use rawspeed_std::coord_common::RowCount;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::wrapping_coord::WrappingColIndex;
use rawspeed_std::wrapping_coord::WrappingRowIndex;

use crate::array2dref::Array2DRef;

#[derive(Debug, Clone, Copy)]
pub struct OffsetArray2DRef<'a, T> {
    data: Array2DRef<'a, T>,
    origin: CoordOffset2D,
}

impl<'a, T> OffsetArray2DRef<'a, T> {
    #[inline]
    #[must_use]
    pub const fn new(data: Array2DRef<'a, T>, origin: CoordOffset2D) -> Self {
        Self { data, origin }
    }

    #[inline]
    #[must_use]
    pub const fn row_length(&self) -> usize {
        self.data.row_length()
    }

    #[inline]
    #[must_use]
    pub const fn num_rows(&self) -> usize {
        self.data.num_rows()
    }

    #[inline]
    #[must_use]
    fn get_row(&self, row: RowIndex) -> Option<&'a [T]> {
        let row = BoundRowIndex::new(RowCount::new(self.data.num_rows()), row)?;
        let row = WrappingRowIndex::from(row) + self.origin.row();
        self.data.get_row(**row)
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(index.row())?;

        let col = BoundColIndex::new(
            RowLength::new(self.data.row_length()),
            index.col(),
        )?;
        let col = WrappingColIndex::from(col) + self.origin.col();
        let col = ***col;
        row.get(col)
    }
}

impl<T> core::ops::Index<Coord2D> for OffsetArray2DRef<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord2D) -> &Self::Output {
        self.get_elt(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
