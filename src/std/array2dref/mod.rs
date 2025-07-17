use crate::coord_common::Coord2D;
use crate::coord_common::RowIndex;
use crate::coord_common::RowLength;
use crate::coord_common::RowPitch;

#[derive(Debug, Clone, Copy)]
pub struct Array2DRef<'a, T> {
    slice: &'a [T],
    pitch: RowPitch,
    row_length: RowLength,
}

impl<'a, T> Array2DRef<'a, T> {
    #[inline]
    pub const fn new(
        slice: &'a [T],
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

    const fn pitch(&self) -> usize {
        self.pitch.val()
    }

    #[inline]
    #[must_use]
    pub const fn row_length(&self) -> usize {
        self.row_length.val()
    }

    #[inline]
    #[must_use]
    pub const fn num_rows(&self) -> usize {
        self.slice.len().checked_div(self.pitch()).unwrap()
    }

    #[expect(clippy::unwrap_in_result)]
    #[inline]
    #[must_use]
    pub fn get_row(&self, row: RowIndex) -> Option<&'a [T]> {
        if *row >= self.num_rows() {
            return None;
        }
        Some(
            {
                let full_row =
                    self.slice.chunks_exact(self.pitch()).nth(*row)?;
                full_row.get(..self.row_length())
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(RowIndex::new(index.row()))?;
        row.get(index.col())
    }
}

impl<'a, T> core::ops::Index<RowIndex> for Array2DRef<'a, T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RowIndex) -> &'a Self::Output {
        self.get_row(index).unwrap()
    }
}

impl<T> core::ops::Index<Coord2D> for Array2DRef<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord2D) -> &Self::Output {
        self.get_elt(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
