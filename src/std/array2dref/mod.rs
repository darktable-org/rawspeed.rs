use crate::coord_common::Coord2D;
use crate::coord_common::RowLength;
use crate::coord_common::RowPitch;

#[derive(Debug)]
pub struct Array2DRef<'a, T> {
    slice: &'a [T],
    pitch: RowPitch,
    row_length: RowLength,
}

#[cfg_attr(not(test), expect(dead_code))]
impl<'a, T> Array2DRef<'a, T> {
    const fn new(
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

    const fn row_length(&self) -> usize {
        self.row_length.val()
    }

    const fn num_rows(&self) -> usize {
        self.slice.len().checked_div(self.pitch()).unwrap()
    }

    #[expect(clippy::unwrap_in_result)]
    fn get_row(&self, row: usize) -> Option<&[T]> {
        if row >= self.num_rows() {
            return None;
        }
        Some(
            {
                let full_row =
                    self.slice.chunks_exact(self.pitch()).nth(row)?;
                full_row.get(..self.row_length())
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(index.row())?;
        row.get(index.col())
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
