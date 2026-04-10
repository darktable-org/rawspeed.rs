#[non_exhaustive]
#[must_use]
#[derive(Debug)]
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
    Idx: Copy + Default,
    core::ops::Range<Idx>: Clone + Iterator<Item = Idx> + ExactSizeIterator,
{
    type Item = Idx;

    type IntoIter = core::iter::Chain<
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
    >;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Ok(range_len) = core::num::NonZero::try_from(self.range.len())
        else {
            return core::iter::Chain::default();
        };
        let normalized_mid = self.mid % range_len;
        let Some(mid_elt) = self.range.clone().nth(normalized_mid) else {
            debug_assert_eq!(self.range.len(), 0);
            return core::iter::Chain::default();
        };
        let front_part = mid_elt..self.range.end;
        let back_part = self.range.start..mid_elt;
        front_part.chain(back_part)
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
