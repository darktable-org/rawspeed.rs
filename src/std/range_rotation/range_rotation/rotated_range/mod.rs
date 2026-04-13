#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct RotatedRange<Idx> {
    range: core::ops::Range<Idx>,
    mid: isize,
}

#[non_exhaustive]
#[must_use]
#[derive(Debug, Clone)]
pub struct RotatedRangeIterator<Idx>
where
    core::ops::Range<Idx>: Iterator<Item = Idx>,
{
    bounds: core::ops::Range<Idx>,
    range: core::iter::Chain<
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
        <core::ops::Range<Idx> as IntoIterator>::IntoIter,
    >,
}

impl<Idx> RotatedRange<Idx> {
    #[inline]
    const fn new(range: core::ops::Range<Idx>, mid: isize) -> Self {
        Self { range, mid }
    }
}

impl<Idx> IntoIterator for RotatedRange<Idx>
where
    Idx: Copy,
    core::ops::Range<Idx>: Clone + Iterator<Item = Idx> + ExactSizeIterator,
    RotatedRangeIterator<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = RotatedRangeIterator<Idx>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let mid_elt = match core::num::NonZero::try_from(self.range.len()) {
            Ok(range_len) => {
                let normalized_mid = match self.mid.try_into() {
                    Ok(mid) => mid,
                    Err(_) => {
                        range_len.get() - (self.mid.unsigned_abs() % range_len)
                    }
                };
                let normalized_mid = normalized_mid % range_len;
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
        RotatedRangeIterator {
            bounds: self.range,
            range: front_part.chain(back_part),
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<Idx> Iterator for RotatedRangeIterator<Idx>
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
