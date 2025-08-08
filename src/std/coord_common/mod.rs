#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowLength {
    val: usize,
}

impl RowLength {
    #[inline]
    #[must_use]
    pub const fn new(len: usize) -> Self {
        Self { val: len }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for RowLength {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowCount {
    val: usize,
}

impl RowCount {
    #[inline]
    #[must_use]
    pub const fn new(len: usize) -> Self {
        Self { val: len }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for RowCount {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions2D {
    row_len: RowLength,
    row_count: RowCount,
}

impl Dimensions2D {
    #[inline]
    #[must_use]
    pub const fn new(row_len: RowLength, row_count: RowCount) -> Self {
        Self { row_len, row_count }
    }

    #[inline]
    #[must_use]
    pub const fn row_len(&self) -> RowLength {
        self.row_len
    }

    #[inline]
    #[must_use]
    pub const fn row_count(&self) -> RowCount {
        self.row_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowPitch {
    val: usize,
}

impl RowPitch {
    #[inline]
    #[must_use]
    pub const fn new(pitch: usize) -> Self {
        Self { val: pitch }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for RowPitch {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RowIndex {
    row: usize,
}

impl RowIndex {
    #[inline]
    #[must_use]
    pub const fn new(row: usize) -> Self {
        Self { row }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.row
    }
}

impl core::ops::Deref for RowIndex {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColIndex {
    col: usize,
}

impl ColIndex {
    #[inline]
    #[must_use]
    pub const fn new(col: usize) -> Self {
        Self { col }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> usize {
        self.col
    }
}

impl core::ops::Deref for ColIndex {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.col
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coord2D {
    row: RowIndex,
    col: ColIndex,
}

impl Coord2D {
    #[inline]
    #[must_use]
    pub const fn new(row: RowIndex, col: ColIndex) -> Self {
        Self { row, col }
    }

    #[inline]
    #[must_use]
    pub const fn row(&self) -> usize {
        self.row.val()
    }

    #[inline]
    #[must_use]
    pub const fn col(&self) -> usize {
        self.col.val()
    }
}
