#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum LayoutfulBoxError {
    OutOfMemory,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct LayoutfulBox<T> {
    layout: core::alloc::Layout,
    ptr: *mut u8,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> LayoutfulBox<T> {
    #[inline]
    pub fn new(layout: core::alloc::Layout) -> Result<Self, LayoutfulBoxError> {
        assert!(layout.size() > 0);
        assert!(layout.size().is_multiple_of(size_of::<T>()));
        assert!(layout.align().is_multiple_of(align_of::<T>()));

        let ptr;
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            ptr = std::alloc::alloc(layout);
        }
        if ptr.is_null() {
            return Err(LayoutfulBoxError::OutOfMemory);
        }
        Ok(Self {
            layout,
            ptr,
            _phantom: core::marker::PhantomData,
        })
    }

    #[inline]
    #[must_use]
    pub const fn get_slice_mut(&mut self) -> &mut [T] {
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            core::slice::from_raw_parts_mut(
                self.ptr.cast::<T>(),
                self.layout.size() / size_of::<T>(),
            )
        }
    }
}

impl<T> Drop for LayoutfulBox<T> {
    #[inline]
    fn drop(&mut self) {
        let ptr = core::mem::take(&mut self.ptr);
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            std::alloc::dealloc(ptr, self.layout);
        }
    }
}

#[cfg(test)]
mod tests;
