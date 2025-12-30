use crate::bound_coord::BoundRowIndex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingRowIndex {
    value: BoundRowIndex,
}

impl WrappingRowIndex {
    #[inline]
    #[must_use]
    pub const fn new(value: BoundRowIndex) -> Self {
        Self { value }
    }
}

impl core::ops::Deref for WrappingRowIndex {
    type Target = BoundRowIndex;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
