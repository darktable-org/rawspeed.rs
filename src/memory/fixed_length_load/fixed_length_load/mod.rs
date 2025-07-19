pub trait CopyFromSlice {
    fn copy_from_slice_(&mut self, src: &[u8]);
}

impl CopyFromSlice for [u8] {
    #[inline]
    fn copy_from_slice_(&mut self, src: &[u8]) {
        self.copy_from_slice(src);
    }
}

pub trait LoadFromSlice<T>
where
    T: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <T as core::ops::Index<core::ops::RangeFull>>::Output: CopyFromSlice,
{
    fn load_from_slice(&self) -> T;
}

impl<T> LoadFromSlice<T> for [u8]
where
    T: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <T as core::ops::Index<core::ops::RangeFull>>::Output: CopyFromSlice,
{
    #[inline]
    fn load_from_slice(&self) -> T {
        let mut out: T = Default::default();
        out[..].copy_from_slice_(self);
        out
    }
}

#[cfg(test)]
mod tests;
