use crate::coord_common::{RowCount, RowIndex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundRowIndex {
    domain: RowCount,
    value: RowIndex,
}

impl BoundRowIndex {
    #[inline]
    #[must_use]
    pub fn new(domain: RowCount, value: RowIndex) -> Option<Self> {
        if value < domain {
            return Some(Self { domain, value });
        }
        None
    }

    #[inline]
    #[must_use]
    pub const fn value(&self) -> RowIndex {
        self.value
    }

    #[inline]
    #[must_use]
    pub const fn domain(&self) -> RowCount {
        self.domain
    }
}

impl core::ops::Deref for BoundRowIndex {
    type Target = RowIndex;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests;
