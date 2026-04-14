use crate::range_rotation::rotated_range_iterator::RotatedRangeIterator;

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct LeftRotatedRange<Idx> {
    range: core::ops::Range<Idx>,
    mid: usize,
}

impl<Idx> LeftRotatedRange<Idx> {
    #[inline]
    const fn new(range: core::ops::Range<Idx>, mid: usize) -> Self {
        Self { range, mid }
    }
}

impl<Idx> IntoIterator for LeftRotatedRange<Idx>
where
    Idx: Copy + PartialOrd,
    core::ops::Range<Idx>: Clone + Iterator<Item = Idx> + ExactSizeIterator,
    RotatedRangeIterator<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = RotatedRangeIterator<Idx>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RotatedRangeIterator::new(self.range, |_| -> usize { self.mid })
    }
}

pub trait LeftRotatableRange {
    type Idx;

    fn rotate_left(self, mid: usize) -> LeftRotatedRange<Self::Idx>;
}

impl<Idx> LeftRotatableRange for core::ops::Range<Idx>
where
    LeftRotatedRange<Idx>: IntoIterator<Item = Idx>,
{
    type Idx = Idx;

    #[inline]
    fn rotate_left(self, mid: usize) -> LeftRotatedRange<Self::Idx> {
        LeftRotatedRange::new(self, mid)
    }
}

#[cfg(test)]
mod tests;
