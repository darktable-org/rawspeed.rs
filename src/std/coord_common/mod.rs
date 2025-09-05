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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub const fn row(&self) -> RowIndex {
        self.row
    }

    #[inline]
    #[must_use]
    pub const fn col(&self) -> ColIndex {
        self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowOffset {
    row: isize,
}

impl RowOffset {
    #[inline]
    #[must_use]
    pub const fn new(row: isize) -> Self {
        Self { row }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> isize {
        self.row
    }
}

impl core::ops::Deref for RowOffset {
    type Target = isize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColOffset {
    col: isize,
}

impl ColOffset {
    #[inline]
    #[must_use]
    pub const fn new(col: isize) -> Self {
        Self { col }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> isize {
        self.col
    }
}

impl core::ops::Deref for ColOffset {
    type Target = isize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordOffset2D {
    row: RowOffset,
    col: ColOffset,
}

impl CoordOffset2D {
    #[inline]
    #[must_use]
    pub const fn new(row: RowOffset, col: ColOffset) -> Self {
        Self { row, col }
    }

    #[inline]
    #[must_use]
    pub const fn row(&self) -> RowOffset {
        self.row
    }

    #[inline]
    #[must_use]
    pub const fn col(&self) -> ColOffset {
        self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ByteMultiple {
    val: usize,
}

impl ByteMultiple {
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

impl core::ops::Deref for ByteMultiple {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Align {
    val: ByteMultiple,
}

impl Align {
    #[inline]
    pub fn new(val: ByteMultiple) -> Result<Self, String> {
        if val > ByteMultiple::new(0) && val.is_power_of_two() {
            return Ok(Self { val });
        }
        Err("Invalid alignment".to_owned())
    }
}

impl core::ops::Deref for Align {
    type Target = ByteMultiple;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
