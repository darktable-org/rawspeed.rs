#[non_exhaustive]
#[must_use]
#[derive(Debug)]
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
        let normalized_mid = match self.mid.try_into() {
            Ok(mid) => mid,
            Err(_) => range_len.get() - (self.mid.unsigned_abs() % range_len),
        };
        let normalized_mid = normalized_mid % range_len;
        let Some(mid_elt) = self.range.clone().nth(normalized_mid) else {
            debug_assert_eq!(self.range.len(), 0);
            return core::iter::Chain::default();
        };
        let front_part = mid_elt..self.range.end;
        let back_part = self.range.start..mid_elt;
        front_part.chain(back_part)
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
