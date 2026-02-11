use rawspeed_common_exact_ops::exact_ops::div::CheckedDivExact;
use rawspeed_std::coord_common::{
    Coord2D, Dimensions2D, RowCount, RowIndex, RowLength, RowPitch,
};

#[derive(Debug)]
pub struct Array2DRefMut<'a, T> {
    slice: &'a mut [T],
    pitch: RowPitch,
    row_length: RowLength,
}

impl<'a, T> Array2DRefMut<'a, T> {
    #[inline]
    pub const fn new(
        slice: &'a mut [T],
        row_length: RowLength,
        pitch: RowPitch,
    ) -> Self {
        assert!(!slice.is_empty());
        assert!(row_length.val() > 0);
        assert!(pitch.val() > 0);
        assert!(pitch.val() >= row_length.val());
        assert!(slice.len().is_multiple_of(pitch.val()));
        Self {
            slice,
            pitch,
            row_length,
        }
    }

    const fn pitch(&self) -> RowPitch {
        self.pitch
    }

    #[inline]
    #[must_use]
    pub const fn row_length(&self) -> RowLength {
        self.row_length
    }

    #[inline]
    #[must_use]
    pub fn num_rows(&self) -> RowCount {
        RowCount::new(
            CheckedDivExact::checked_div_exact(
                self.slice.len(),
                self.pitch().val(),
            )
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn dims(&self) -> Dimensions2D {
        Dimensions2D::new(self.row_length(), self.num_rows())
    }

    #[inline]
    #[must_use]
    pub fn get_row(&self, row: RowIndex) -> Option<&[T]> {
        if *row >= *self.num_rows() {
            return None;
        }
        Some(
            {
                let full_row =
                    self.slice.chunks_exact(*self.pitch()).nth(*row)?;
                full_row.get(..*self.row_length())
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn get_row_mut(&mut self, row: RowIndex) -> Option<&mut [T]> {
        if *row >= *self.num_rows() {
            return None;
        }
        Some(
            {
                let row_len = self.row_length();
                let full_row =
                    self.slice.chunks_exact_mut(*self.pitch()).nth(*row)?;
                full_row.get_mut(..*row_len)
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(RowIndex::new(*index.row()))?;
        row.get(*index.col())
    }

    #[inline]
    #[must_use]
    pub fn get_elt_mut(&mut self, index: Coord2D) -> Option<&mut T> {
        let row = self.get_row_mut(RowIndex::new(*index.row()))?;
        row.get_mut(*index.col())
    }
}

impl<'a, T> From<Array2DRefMut<'a, T>>
    for crate::array2dref::Array2DRef<'a, T>
{
    #[inline]
    fn from(val: Array2DRefMut<'a, T>) -> Self {
        Self::new(val.slice, val.row_length, val.pitch)
    }
}

impl<T> core::ops::Index<RowIndex> for Array2DRefMut<'_, T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RowIndex) -> &Self::Output {
        self.get_row(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<RowIndex> for Array2DRefMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: RowIndex) -> &mut Self::Output {
        self.get_row_mut(index).unwrap()
    }
}

impl<T> core::ops::Index<Coord2D> for Array2DRefMut<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord2D) -> &Self::Output {
        self.get_elt(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<Coord2D> for Array2DRefMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: Coord2D) -> &mut Self::Output {
        self.get_elt_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
