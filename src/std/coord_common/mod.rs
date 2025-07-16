#[derive(Debug, Clone, Copy)]
pub struct RowLength {
    val: usize,
}

impl RowLength {
    #[must_use]
    pub const fn new(len: usize) -> Self {
        Self { val: len }
    }

    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for RowLength {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RowPitch {
    val: usize,
}

impl RowPitch {
    #[must_use]
    pub const fn new(pitch: usize) -> Self {
        Self { val: pitch }
    }

    #[must_use]
    pub const fn val(&self) -> usize {
        self.val
    }
}

impl core::ops::Deref for RowPitch {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RowIndex {
    row: usize,
}

impl RowIndex {
    #[must_use]
    pub const fn new(row: usize) -> Self {
        Self { row }
    }

    #[must_use]
    pub const fn val(&self) -> usize {
        self.row
    }
}

impl core::ops::Deref for RowIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColIndex {
    col: usize,
}

impl ColIndex {
    #[must_use]
    pub const fn new(col: usize) -> Self {
        Self { col }
    }

    #[must_use]
    pub const fn val(&self) -> usize {
        self.col
    }
}

impl core::ops::Deref for ColIndex {
    type Target = usize;

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
    #[must_use]
    pub const fn new(row: RowIndex, col: ColIndex) -> Self {
        Self { row, col }
    }

    #[must_use]
    pub const fn row(&self) -> usize {
        self.row.val()
    }

    #[must_use]
    pub const fn col(&self) -> usize {
        self.col.val()
    }
}
