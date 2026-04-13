#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct LeftRotatedRange<Idx> {
    range: core::ops::Range<Idx>,
    mid: usize,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct LeftRotatedRangeIterator<Idx>
where
    core::ops::Range<Idx>: Iterator<Item = Idx>,
{
    bounds: core::ops::Range<Idx>,
    range: core::iter::Chain<
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
    >,
}

impl<Idx> LeftRotatedRange<Idx> {
    #[inline]
    const fn new(range: core::ops::Range<Idx>, mid: usize) -> Self {
        Self { range, mid }
    }
}

impl<Idx> IntoIterator for LeftRotatedRange<Idx>
where
    Idx: Copy,
    core::ops::Range<Idx>: Clone + Iterator<Item = Idx> + ExactSizeIterator,
    LeftRotatedRangeIterator<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = LeftRotatedRangeIterator<Idx>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let mid_elt = match core::num::NonZero::try_from(self.range.len()) {
            Ok(range_len) => {
                let normalized_mid = self.mid % range_len;
                let elt = self.range.clone().nth(normalized_mid);
                #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
                unsafe {
                    elt.unwrap_unchecked()
                }
            }
            Err(_) => self.range.start,
        };

        let front_part = mid_elt..self.range.end;
        let back_part = self.range.start..mid_elt;
        LeftRotatedRangeIterator {
            bounds: self.range,
            range: front_part.chain(back_part),
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<Idx> Iterator for LeftRotatedRangeIterator<Idx>
where
    Idx: PartialOrd,
    core::ops::Range<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.range.next()?;
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::hint::assert_unchecked(self.bounds.contains(&item));
        }
        Some(item)
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
