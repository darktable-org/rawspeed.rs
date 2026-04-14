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

impl<Idx> RotatedRangeIterator<Idx>
where
    Idx: Copy + PartialOrd,
    core::ops::Range<Idx>: Iterator<Item = Idx> + ExactSizeIterator,
{
    #[inline]
    pub(in crate::range_rotation) fn new<F>(
        range: core::ops::Range<Idx>,
        index: F,
    ) -> Self
    where
        F: FnOnce(core::num::NonZero<usize>) -> usize,
    {
        let mid_elt = match range.len().try_into() {
            Ok(range_len) => {
                let mid = index(range_len);
                let elt = range.clone().nth(mid % range_len);
                #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
                unsafe {
                    elt.unwrap_unchecked()
                }
            }
            Err(_) => range.start,
        };
        let front_part = mid_elt..range.end;
        let back_part = range.start..mid_elt;
        RotatedRangeIterator {
            bounds: range,
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
