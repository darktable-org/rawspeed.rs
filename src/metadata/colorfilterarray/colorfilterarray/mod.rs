use rawspeed_std::coord_common::{RowLength, RowPitch};
use rawspeed_std_ndslice::array2dref::Array2DRef;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum ColorVariant {
    Red,
    Green,
    Blue,
    FujiGreen,
    Magenta,
    Yellow,
    Cyan,
}

impl ColorVariant {
    #[inline]
    #[must_use]
    const fn card() -> usize {
        7
    }
}

pub mod dcraw_filter;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ColorFilterArray {
    data: Vec<ColorVariant>,
    row_length: RowLength,
}

impl ColorFilterArray {
    #[inline]
    #[must_use]
    pub const fn new(data: Vec<ColorVariant>, row_length: RowLength) -> Self {
        let ret = Self { data, row_length };
        let _ = ret.mat();
        ret
    }

    #[inline]
    #[must_use]
    pub const fn mat(&self) -> Array2DRef<'_, ColorVariant> {
        Array2DRef::new(
            self.data.as_slice(),
            self.row_length,
            RowPitch::new(self.row_length.val()),
        )
    }
}
