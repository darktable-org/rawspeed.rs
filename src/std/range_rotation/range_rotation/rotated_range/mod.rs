use crate::range_rotation::rotated_range_iterator::RotatedRangeIterator;

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct RotatedRange<Idx> {
    range: core::ops::Range<Idx>,
    mid: isize,
}

impl<Idx> RotatedRange<Idx> {
    #[inline]
    const fn new(range: core::ops::Range<Idx>, mid: isize) -> Self {
        Self { range, mid }
    }
}

impl<Idx> IntoIterator for RotatedRange<Idx>
where
    Idx: Copy + PartialOrd,
    core::ops::Range<Idx>: Clone + Iterator<Item = Idx> + ExactSizeIterator,
    RotatedRangeIterator<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = RotatedRangeIterator<Idx>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RotatedRangeIterator::new(self.range, |range_len| -> usize {
            match self.mid.try_into() {
                Ok(mid) => mid,
                Err(_) => {
                    range_len.get() - (self.mid.unsigned_abs() % range_len)
                }
            }
        })
    }
}

pub trait RotatableRange {
    type Idx;

    fn rotate(self, mid: isize) -> RotatedRange<Self::Idx>;
}

impl<Idx> RotatableRange for core::ops::Range<Idx>
where
    RotatedRange<Idx>: IntoIterator<Item = Idx>,
{
    type Idx = Idx;

    #[inline]
    fn rotate(self, mid: isize) -> RotatedRange<Self::Idx> {
        RotatedRange::new(self, mid)
    }
}

#[cfg(test)]
mod tests;
