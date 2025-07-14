#[derive(Debug, Clone, Copy)]
pub struct RowLength {
    val: usize,
}

#[cfg_attr(not(test), expect(dead_code))]
impl RowLength {
    pub const fn new(len: usize) -> Self {
        Self { val: len }
    }

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

#[cfg_attr(not(test), expect(dead_code))]
impl RowPitch {
    pub const fn new(pitch: usize) -> Self {
        Self { val: pitch }
    }

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

#[cfg_attr(not(test), expect(dead_code))]
impl RowIndex {
    pub const fn new(row: usize) -> Self {
        Self { row }
    }

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

#[cfg_attr(not(test), expect(dead_code))]
impl ColIndex {
    pub const fn new(col: usize) -> Self {
        Self { col }
    }

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

#[cfg_attr(not(test), expect(dead_code))]
impl Coord2D {
    pub const fn new(row: RowIndex, col: ColIndex) -> Self {
        Self { row, col }
    }

    pub const fn row(&self) -> usize {
        self.row.val()
    }

    pub const fn col(&self) -> usize {
        self.col.val()
    }
}
