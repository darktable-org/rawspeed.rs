#[derive(Debug)]
pub struct SVec<T, const N: usize>
where
    T: Copy + From<u8>,
{
    buf: [T; N],
    size: usize,
}

impl<T, const N: usize> SVec<T, N>
where
    T: Copy + From<u8>,
{
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        assert!(self.size <= N);
        self.size
    }

    #[inline]
    #[must_use]
    pub const fn remaining_capacity(&self) -> usize {
        N - self.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.remaining_capacity() == N
    }

    #[inline]
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.remaining_capacity() == 0
    }

    #[inline]
    pub fn extend(&mut self, elts: &[T]) {
        let curr_len = self.len();
        let dest = self
            .buf
            .get_mut(curr_len..)
            .unwrap()
            .get_mut(..elts.len())
            .expect("Buffer overflow");
        dest.copy_from_slice(elts);
        self.size += elts.len();
    }
}

impl<T, const N: usize> Default for SVec<T, N>
where
    T: Copy + From<u8>,
{
    #[inline]
    fn default() -> Self {
        Self {
            buf: [0_u8.into(); N],
            size: 0,
        }
    }
}

impl<T, const N: usize> core::ops::Index<core::ops::RangeFull> for SVec<T, N>
where
    T: Copy + From<u8>,
{
    type Output = [T];

    #[inline]
    fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
        assert_eq!(self.len(), N, "Buffer is not full yet!");
        self.buf.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
