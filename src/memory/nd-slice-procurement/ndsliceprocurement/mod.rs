use rawspeed_memory_layoutfulbox::layoutfulbox::{
    LayoutfulBox, LayoutfulBoxError,
};
use rawspeed_std::coord_common::{
    Align, ByteMultiple, Dimensions2D, RowLength, RowPitch,
};
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

macro_rules! impl_strict_type {
    ($struct:ident of $type:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
        #[non_exhaustive]
        #[must_use]
        pub struct $struct {
            val: $type,
        }

        impl $struct {
            #[inline]
            pub const fn new(val: $type) -> Self {
                Self { val }
            }
        }

        impl core::ops::Deref for $struct {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.val
            }
        }
    };
}

impl_strict_type!(EltCount of usize);

#[derive(Debug, PartialEq)]
#[non_exhaustive]
#[must_use]
pub struct OwnedNDSlice<T> {
    storage: LayoutfulBox<T>,
    row_len: RowLength<core::num::NonZero<usize>>,
    pitch: RowPitch<core::num::NonZero<usize>>,
}

impl<T> OwnedNDSlice<T> {
    #[inline]
    const fn new(
        storage: LayoutfulBox<T>,
        row_len: RowLength<core::num::NonZero<usize>>,
        pitch: RowPitch<core::num::NonZero<usize>>,
    ) -> Self {
        Self {
            storage,
            row_len,
            pitch,
        }
    }

    #[inline]
    pub const fn get_mut(&mut self) -> Array2DRefMut<'_, T> {
        Array2DRefMut::new(
            self.storage.get_slice_mut(),
            self.row_len,
            self.pitch,
        )
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NDSliceProcurementRequestError {
    LayoutError(core::alloc::LayoutError),
    OutOfMemory,
}

impl core::fmt::Display for NDSliceProcurementRequestError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NDSliceProcurementRequestError::LayoutError(error) => {
                write!(f, "NDSliceProcurementRequestError({error})")
            }
            NDSliceProcurementRequestError::OutOfMemory => {
                write!(f, "DSliceProcurementRequestError(OutOfMemory)")
            }
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[must_use]
pub struct NDSliceProcurementRequest<T>
where
    T: Sized,
{
    dims: Dimensions2D<core::num::NonZero<usize>>,
    extra_row_padding: EltCount,
    row_alignment: Align,
    base_alignment: Align,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> NDSliceProcurementRequest<T> {
    #[inline]
    pub fn new(dims: Dimensions2D<core::num::NonZero<usize>>) -> Self {
        Self {
            dims,
            extra_row_padding: EltCount::new(0),
            row_alignment: Align::new(ByteMultiple::new(1)).unwrap(),
            base_alignment: Align::new(ByteMultiple::new(1)).unwrap(),
            _phantom: core::marker::PhantomData,
        }
    }

    #[inline]
    pub const fn dims(&self) -> Dimensions2D<core::num::NonZero<usize>> {
        self.dims
    }

    #[inline]
    pub const fn set_extra_row_padding(
        self,
        extra_row_padding: EltCount,
    ) -> Self {
        Self {
            extra_row_padding,
            ..self
        }
    }

    #[inline]
    pub const fn set_row_alignment(self, row_alignment: Align) -> Self {
        Self {
            row_alignment,
            ..self
        }
    }

    #[inline]
    pub const fn set_base_alignment(self, base_alignment: Align) -> Self {
        Self {
            base_alignment,
            ..self
        }
    }

    #[inline]
    pub fn get_layout(
        &self,
    ) -> Result<
        (core::alloc::Layout, RowPitch<core::num::NonZero<usize>>),
        core::alloc::LayoutError,
    > {
        let row_len = self
            .dims()
            .row_len()
            .checked_add(*self.extra_row_padding)
            .unwrap();
        let mut row_layout = core::alloc::Layout::array::<T>(row_len.get())?
            .align_to(**self.row_alignment)?;
        if self.dims().row_count().get() > 1 {
            row_layout = row_layout.pad_to_align();
        }

        let row_pitch = RowPitch::new(
            core::num::NonZero::new(row_layout.size() / size_of::<T>())
                .unwrap(),
        );

        let size = row_pitch
            .checked_mul(self.dims().row_count().val())
            .unwrap();
        let layout = core::alloc::Layout::array::<T>(size.get())?
            .align_to(row_layout.align())?
            .align_to(**self.base_alignment)?;

        Ok((layout, row_pitch))
    }

    #[inline]
    pub fn fulfill(
        self,
    ) -> Result<OwnedNDSlice<T>, NDSliceProcurementRequestError> {
        let (layout, row_pitch) = match self.get_layout() {
            Ok(v) => v,
            Err(e) => {
                return Err(NDSliceProcurementRequestError::LayoutError(e));
            }
        };

        let storage = match LayoutfulBox::new(layout) {
            Ok(storage) => storage,
            Err(LayoutfulBoxError::OutOfMemory) => {
                return Err(NDSliceProcurementRequestError::OutOfMemory);
            }
            _ => unreachable!(),
        };

        let mut res =
            OwnedNDSlice::new(storage, self.dims().row_len(), row_pitch);
        assert_eq!(res.get_mut().dims(), self.dims());
        Ok(res)
    }
}

#[cfg(test)]
mod tests;
