use rawspeed_common_exact_ops::exact_ops::div::CheckedDivExact;
use rawspeed_std::coord_common::{
    Coord2D, RowCount, RowIndex, RowLength, RowPitch,
};

#[derive(Debug, Clone, Copy)]
pub struct Array2DRef<'a, T> {
    slice: &'a [T],
    pitch: RowPitch<core::num::NonZero<usize>>,
    row_length: RowLength<core::num::NonZero<usize>>,
}

impl<'a, T> Array2DRef<'a, T> {
    #[inline]
    pub const fn new(
        slice: &'a [T],
        row_length: RowLength<core::num::NonZero<usize>>,
        pitch: RowPitch<core::num::NonZero<usize>>,
    ) -> Self {
        assert!(!slice.is_empty());
        assert!(pitch.val().get() >= row_length.val().get());
        assert!(slice.len().is_multiple_of(pitch.val().get()));
        Self {
            slice,
            pitch,
            row_length,
        }
    }

    const fn pitch(&self) -> RowPitch<core::num::NonZero<usize>> {
        self.pitch
    }

    #[inline]
    #[must_use]
    pub const fn row_length(&self) -> RowLength<core::num::NonZero<usize>> {
        self.row_length
    }

    #[inline]
    #[must_use]
    pub fn num_rows(&self) -> RowCount<core::num::NonZero<usize>> {
        CheckedDivExact::checked_div_exact(
            self.slice.len(),
            self.pitch().val().get(),
        )
        .and_then(|v| v.try_into().ok())
        .map(RowCount::new)
        .unwrap()
    }

    #[inline]
    #[must_use]
    pub fn get_row(&self, row: RowIndex) -> Option<&'a [T]> {
        if *row >= self.num_rows().get() {
            return None;
        }
        Some(
            {
                let full_row =
                    self.slice.chunks_exact(self.pitch().get()).nth(*row)?;
                full_row.get(..self.row_length().get())
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&'a T> {
        let row = self.get_row(RowIndex::new(*index.row()))?;
        row.get(*index.col())
    }
}

impl<'a, T> core::ops::Index<RowIndex> for Array2DRef<'a, T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RowIndex) -> &'a Self::Output {
        self.get_row(index).unwrap()
    }
}

impl<'a, T> core::ops::Index<Coord2D> for Array2DRef<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord2D) -> &'a Self::Output {
        self.get_elt(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
