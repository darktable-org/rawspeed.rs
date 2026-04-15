#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct RowLength<T = usize> {
    val: T,
}

impl<T> RowLength<T>
where
    T: Copy,
{
    #[inline]
    pub const fn new(len: T) -> Self {
        Self { val: len }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> T {
        self.val
    }
}

impl<T> core::ops::Deref for RowLength<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct RowCount<T = usize> {
    val: T,
}

impl<T> RowCount<T>
where
    T: Copy,
{
    #[inline]
    pub const fn new(len: T) -> Self {
        Self { val: len }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> T {
        self.val
    }
}

impl<T> core::ops::Deref for RowCount<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct Dimensions2D<T = usize> {
    row_len: RowLength<T>,
    row_count: RowCount<T>,
}

impl<T> Dimensions2D<T>
where
    T: Copy,
{
    #[inline]
    pub const fn new(row_len: RowLength<T>, row_count: RowCount<T>) -> Self {
        Self { row_len, row_count }
    }

    #[inline]
    pub const fn row_len(&self) -> RowLength<T> {
        self.row_len
    }

    #[inline]
    pub const fn row_count(&self) -> RowCount<T> {
        self.row_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct RowPitch<T = usize> {
    val: T,
}

impl<T> RowPitch<T>
where
    T: Copy,
{
    #[inline]
    pub const fn new(pitch: T) -> Self {
        Self { val: pitch }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> T {
        self.val
    }
}

impl<T> core::ops::Deref for RowPitch<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct RowIndex {
    row: usize,
}

impl RowIndex {
    #[inline]
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

#[expect(clippy::missing_trait_methods)]
impl PartialEq<RowCount> for RowIndex {
    #[inline]
    fn eq(&self, other: &RowCount) -> bool {
        (**self).eq(&**other)
    }
}

#[expect(clippy::missing_trait_methods)]
impl PartialOrd<RowCount> for RowIndex {
    #[inline]
    fn partial_cmp(&self, other: &RowCount) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct ColIndex {
    col: usize,
}

impl ColIndex {
    #[inline]
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
#[non_exhaustive]
#[must_use]
pub struct Coord2D {
    row: RowIndex,
    col: ColIndex,
}

impl Coord2D {
    #[inline]
    pub const fn new(row: RowIndex, col: ColIndex) -> Self {
        Self { row, col }
    }

    #[inline]
    pub const fn row(&self) -> RowIndex {
        self.row
    }

    #[inline]
    pub const fn col(&self) -> ColIndex {
        self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct RowOffset {
    row: isize,
}

impl RowOffset {
    #[inline]
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
#[non_exhaustive]
#[must_use]
pub struct ColOffset {
    col: isize,
}

impl ColOffset {
    #[inline]
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

#[expect(clippy::missing_trait_methods)]
impl PartialEq<RowLength> for ColIndex {
    #[inline]
    fn eq(&self, other: &RowLength) -> bool {
        (**self).eq(&**other)
    }
}

#[expect(clippy::missing_trait_methods)]
impl PartialOrd<RowLength> for ColIndex {
    #[inline]
    fn partial_cmp(&self, other: &RowLength) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct CoordOffset2D {
    row: RowOffset,
    col: ColOffset,
}

impl CoordOffset2D {
    #[inline]
    pub const fn new(row: RowOffset, col: ColOffset) -> Self {
        Self { row, col }
    }

    #[inline]
    pub const fn row(&self) -> RowOffset {
        self.row
    }

    #[inline]
    pub const fn col(&self) -> ColOffset {
        self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
#[must_use]
pub struct ByteMultiple {
    val: usize,
}

impl ByteMultiple {
    #[inline]
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
#[non_exhaustive]
#[must_use]
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
