use rawspeed_std::coord_common::{
    ColIndex, ColOffset, Coord2D, RowCount, RowIndex, RowLength, RowOffset,
};

use super::{height, width, x, xmlparser, y};

mod private {
    use super::{height, width, x, xmlparser, y};

    impl_elt_matcher!(
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Crop {
            x: x::X,
            y: y::Y,
            width: width::Width,
            height: height::Height,
        }
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct AbsoluteCropPosition {
    pos: Coord2D,
}

impl AbsoluteCropPosition {
    #[must_use]
    #[inline]
    pub const fn new(pos: Coord2D) -> Self {
        Self { pos }
    }
}

impl core::ops::Deref for AbsoluteCropPosition {
    type Target = Coord2D;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Width {
    Relative(ColOffset),
    Absolute(RowLength),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Height {
    Relative(RowOffset),
    Absolute(RowCount),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct CropSize {
    width: Width,
    height: Height,
}

impl CropSize {
    #[inline]
    #[must_use]
    pub const fn new(width: Width, height: Height) -> Self {
        Self { width, height }
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> Width {
        self.width
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> Height {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Crop {
    pub pos: AbsoluteCropPosition,
    pub dim: CropSize,
}

impl Crop {
    #[must_use]
    #[inline]
    pub const fn new(pos: AbsoluteCropPosition, dim: CropSize) -> Self {
        Self { pos, dim }
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Crop {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let crop: private::Crop = input.parse()?;
        if **crop.x < 0 || **crop.y < 0 {
            return Err("Crop x/y must be non-negative".to_owned());
        }
        let pos = AbsoluteCropPosition::new(Coord2D::new(
            RowIndex::new((**crop.y).try_into().unwrap()),
            ColIndex::new((**crop.x).try_into().unwrap()),
        ));
        let width = if **crop.width <= 0 {
            Width::Relative(ColOffset::new((**crop.width).try_into().unwrap()))
        } else {
            Width::Absolute(RowLength::new((**crop.width).try_into().unwrap()))
        };
        let height = if **crop.height <= 0 {
            Height::Relative(RowOffset::new(
                (**crop.height).try_into().unwrap(),
            ))
        } else {
            Height::Absolute(RowCount::new((**crop.height).try_into().unwrap()))
        };
        let dim = CropSize::new(width, height);
        Ok(Crop::new(pos, dim))
    }
}

#[cfg(test)]
mod tests;
