use rawspeed_std::{
    bound_coord::{BoundColIndex, BoundRowIndex},
    coord_common::{Coord2D, CoordOffset2D, RowCount, RowIndex, RowLength},
    wrapping_coord::{WrappingColIndex, WrappingRowIndex},
};

use crate::array2dref::Array2DRef;

mod iteration;

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct OffsetArray2DRef<'a, T> {
    data: Array2DRef<'a, T>,
    origin: CoordOffset2D,
}

impl<'a, T> OffsetArray2DRef<'a, T> {
    #[inline]
    pub const fn new(data: Array2DRef<'a, T>, origin: CoordOffset2D) -> Self {
        Self { data, origin }
    }

    #[inline]
    #[must_use]
    pub const fn row_length(&self) -> RowLength<core::num::NonZero<usize>> {
        self.data.row_length()
    }

    #[inline]
    #[must_use]
    pub fn num_rows(&self) -> RowCount<core::num::NonZero<usize>> {
        self.data.num_rows()
    }

    #[inline]
    #[must_use]
    fn get_row(&self, row: RowIndex) -> Option<&'a [T]> {
        if row.val() >= self.num_rows().val().get() {
            return None;
        }
        let row =
            BoundRowIndex::new(RowCount::new(self.data.num_rows().get()), row);
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let row = unsafe { row.unwrap_unchecked() };
        let row = WrappingRowIndex::from(row) + self.origin.row();
        let row = self.data.get_row(**row);
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let row = unsafe { row.unwrap_unchecked() };
        Some(row)
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&'a T> {
        let row = self.get_row(RowIndex::new(*index.row()))?;
        if *index.col() >= self.row_length().val().get() {
            return None;
        }

        let col = BoundColIndex::new(
            RowLength::new(self.data.row_length().get()),
            index.col(),
        );
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let col = unsafe { col.unwrap_unchecked() };
        let col = WrappingColIndex::from(col) + self.origin.col();
        let col = ***col;
        let col = row.get(col);
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let col = unsafe { col.unwrap_unchecked() };
        Some(col)
    }

    #[inline]
    pub const fn rows<'b>(&'b self) -> iteration::Rows<'a, 'b, T> {
        iteration::Rows::new(self)
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
