use rawspeed_std::coord_common::{
    Coord2D, RowCount, RowIndex, RowLength, RowPitch,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[must_use]
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
    pub const fn row_length(&self) -> RowLength<core::num::NonZero<usize>> {
        self.row_length
    }

    #[inline]
    fn rows(&self) -> core::slice::ChunksExact<'a, T> {
        let rows = self.slice.chunks_exact(self.pitch().val().get());
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::hint::assert_unchecked(rows.remainder().is_empty());
        }
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::hint::assert_unchecked(rows.len() != 0);
        }
        rows
    }

    #[inline]
    pub fn num_rows(&self) -> RowCount<core::num::NonZero<usize>> {
        let num_rows = core::num::NonZero::new(self.rows().len());
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let num_rows = unsafe { num_rows.unwrap_unchecked() };
        RowCount::new(num_rows)
    }

    #[inline]
    #[must_use]
    pub fn get_row(&self, row: RowIndex) -> Option<&'a [T]> {
        if row.val() >= self.num_rows().val().get() {
            return None;
        }
        let full_row = self.rows().nth(*row);
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let full_row = unsafe { full_row.unwrap_unchecked() };
        let parts = full_row.split_at_checked(self.row_length.val().get());
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let (data, _padding) = unsafe { parts.unwrap_unchecked() };
        Some(data)
    }

    #[inline]
    #[must_use]
    pub fn get_elt(&self, index: Coord2D) -> Option<&'a T> {
        let row = self.get_row(RowIndex::new(*index.row()))?;
        if *index.col() >= self.row_length().val().get() {
            return None;
        }
        let elt = row.get(*index.col());
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        let elt = unsafe { elt.unwrap_unchecked() };
        Some(elt)
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
