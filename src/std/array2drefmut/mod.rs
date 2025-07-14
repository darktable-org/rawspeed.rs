#[derive(Debug, Clone, Copy)]
struct RowLength {
    val: usize,
}

#[cfg_attr(not(test), expect(dead_code))]
impl RowLength {
    const fn new(len: usize) -> Self {
        Self { val: len }
    }
}

impl core::ops::Deref for RowLength {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
struct RowPitch {
    val: usize,
}

#[cfg_attr(not(test), expect(dead_code))]
impl RowPitch {
    const fn new(pitch: usize) -> Self {
        Self { val: pitch }
    }
}

impl core::ops::Deref for RowPitch {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
struct RowIndex {
    row: usize,
}

#[cfg_attr(not(test), expect(dead_code))]
impl RowIndex {
    const fn new(row: usize) -> Self {
        Self { row }
    }
}

impl core::ops::Deref for RowIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

#[derive(Debug, Clone, Copy)]
struct ColIndex {
    col: usize,
}

#[cfg_attr(not(test), expect(dead_code))]
impl ColIndex {
    const fn new(col: usize) -> Self {
        Self { col }
    }
}

impl core::ops::Deref for ColIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.col
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord2D {
    row: RowIndex,
    col: ColIndex,
}

#[cfg_attr(not(test), expect(dead_code))]
impl Coord2D {
    const fn new(row: RowIndex, col: ColIndex) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Array2DRefMut<'a, T> {
    slice: &'a mut [T],
    pitch: RowPitch,
    row_length: RowLength,
}

#[cfg_attr(not(test), expect(dead_code))]
impl<'a, T> Array2DRefMut<'a, T> {
    const fn new(
        slice: &'a mut [T],
        row_length: RowLength,
        pitch: RowPitch,
    ) -> Self {
        assert!(!slice.is_empty());
        assert!(row_length.val > 0);
        assert!(pitch.val > 0);
        assert!(pitch.val >= row_length.val);
        assert!(slice.len().is_multiple_of(pitch.val));
        Self {
            slice,
            pitch,
            row_length,
        }
    }

    const fn pitch(&self) -> usize {
        self.pitch.val
    }

    const fn row_length(&self) -> usize {
        self.row_length.val
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

    #[expect(clippy::unwrap_in_result)]
    fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.num_rows() {
            return None;
        }
        Some(
            {
                let row_len = self.row_length();
                let full_row =
                    self.slice.chunks_exact_mut(self.pitch()).nth(row)?;
                full_row.get_mut(..row_len)
            }
            .unwrap(),
        )
    }

    pub fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(*index.row)?;
        row.get(*index.col)
    }

    pub fn get_elt_mut(&mut self, index: Coord2D) -> Option<&mut T> {
        let row = self.get_row_mut(*index.row)?;
        row.get_mut(*index.col)
    }
}

impl<T> core::ops::Index<Coord2D> for Array2DRefMut<'_, T> {
    type Output = T;

    fn index(&self, index: Coord2D) -> &Self::Output {
        self.get_elt(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<Coord2D> for Array2DRefMut<'_, T> {
    fn index_mut(&mut self, index: Coord2D) -> &mut Self::Output {
        self.get_elt_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
