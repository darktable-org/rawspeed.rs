use rawspeed_std::coord_common::{ColIndex, Coord2D, RowIndex};
use rawspeed_std_range_rotation::range_rotation::rotated_range::{
    RotatableRange as _, RotatedRange,
};

use crate::{array2dref::Array2DRef, offsetarray2dref::OffsetArray2DRef};

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Rows<'a, 'b, T> {
    underlying: &'b OffsetArray2DRef<'a, T>,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct RowIterator<'a, 'b, T> {
    underlying: &'b OffsetArray2DRef<'a, T>,
    rotated_rows: <RotatedRange<usize> as IntoIterator>::IntoIter,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Row<'a, 'b, T> {
    underlying: &'b OffsetArray2DRef<'a, T>,
    row: RowIndex,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Columns<'a, 'b, 'c, T> {
    underlying: &'c Row<'a, 'b, T>,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct ColumnIterator<'a, 'b, 'c, T> {
    underlying: &'c Row<'a, 'b, T>,
    rotated_cols: <RotatedRange<usize> as IntoIterator>::IntoIter,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Element<'a, 'b, T> {
    underlying: &'b Array2DRef<'a, T>,
    coord: Coord2D,
}

impl<'a, 'b, T> Rows<'a, 'b, T> {
    #[inline]
    pub(in crate::offsetarray2dref) const fn new(
        underlying: &'b OffsetArray2DRef<'a, T>,
    ) -> Self {
        Self { underlying }
    }
}

impl<'a, 'b, T> IntoIterator for Rows<'a, 'b, T> {
    type Item = Row<'a, 'b, T>;

    type IntoIter = RowIterator<'a, 'b, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIterator::new(self.underlying)
    }
}

impl<'a, 'b, T> RowIterator<'a, 'b, T> {
    #[inline]
    fn new(underlying: &'b OffsetArray2DRef<'a, T>) -> Self {
        let rows = 0..*underlying.num_rows();
        let rotated_rows = rows.rotate(*underlying.origin.row());
        let rotated_rows = rotated_rows.into_iter();
        Self {
            underlying,
            rotated_rows,
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<'a, 'b, T> Iterator for RowIterator<'a, 'b, T> {
    type Item = Row<'a, 'b, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rotated_rows.next()?;
        Some(Row::new(self.underlying, RowIndex::new(row)))
    }
}

impl<'a, 'b, T> Row<'a, 'b, T> {
    #[inline]
    const fn new(
        underlying: &'b OffsetArray2DRef<'a, T>,
        row: RowIndex,
    ) -> Self {
        Self { underlying, row }
    }

    #[inline]
    pub const fn cols<'c>(&'c self) -> Columns<'a, 'b, 'c, T> {
        Columns::new(self)
    }
}

impl<'a, 'b, 'c, T> Columns<'a, 'b, 'c, T> {
    #[inline]
    const fn new(underlying: &'c Row<'a, 'b, T>) -> Self {
        Self { underlying }
    }
}

impl<'a, 'b, 'c, T> IntoIterator for Columns<'a, 'b, 'c, T> {
    type Item = Element<'a, 'b, T>;

    type IntoIter = ColumnIterator<'a, 'b, 'c, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColumnIterator::new(self.underlying)
    }
}

impl<'a, 'b, 'c, T> ColumnIterator<'a, 'b, 'c, T> {
    #[inline]
    fn new(underlying: &'c Row<'a, 'b, T>) -> Self {
        let cols = 0..*underlying.underlying.row_length();
        let rotated_cols = cols.rotate(*underlying.underlying.origin.col());
        let rotated_cols = rotated_cols.into_iter();
        Self {
            underlying,
            rotated_cols,
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<'a, 'b, T> Iterator for ColumnIterator<'a, 'b, '_, T> {
    type Item = Element<'a, 'b, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let col = self.rotated_cols.next()?;
        let coord = Coord2D::new(self.underlying.row, ColIndex::new(col));
        Some(Element::new(&self.underlying.underlying.data, coord))
    }
}

impl<'a, 'b, T> Element<'a, 'b, T> {
    #[inline]
    const fn new(underlying: &'b Array2DRef<'a, T>, coord: Coord2D) -> Self {
        Self { underlying, coord }
    }
}

impl<'a, T> core::ops::Deref for Element<'a, '_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &'a Self::Target {
        self.underlying.get_elt(self.coord).unwrap()
    }
}
