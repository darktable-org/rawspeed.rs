use rawspeed_metadata_colorfilterarray::colorfilterarray::{
    ColorFilterArray, ColorVariant,
};
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::{ColIndex, Coord2D, RowIndex, RowLength};

use crate::camerasxml_parser::color;

mod repr;

#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::upper_case_acronyms)]
#[non_exhaustive]
pub struct CFA {
    data: ColorFilterArray,
}

impl CFA {
    pub const fn new(data: ColorFilterArray) -> Self {
        Self { data }
    }
}

impl core::ops::Deref for CFA {
    type Target = ColorFilterArray;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for CFA {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let cfa = input.parse::<repr::CFA>()?;
        let mat = cfa.body.mat();
        let real_height = mat.num_rows();
        let real_width = mat.row_length();
        if Ok(real_height) != (**cfa.height).try_into() {
            return Err(format!(
                "unexpected CFA matrix row count, got {} expected {}",
                real_height, **cfa.height
            ));
        }
        if Ok(real_width) != (**cfa.width).try_into() {
            return Err(format!(
                "unexpected CFA matrix row length, got {} expected {}",
                real_width, **cfa.width
            ));
        }
        let mut data =
            Vec::with_capacity(real_width.checked_mul(real_height).unwrap());
        for row in 0..real_height {
            for col in 0..real_width {
                let e =
                    mat[Coord2D::new(RowIndex::new(row), ColIndex::new(col))];
                let e = match e {
                    color::ColorVariant::Red => ColorVariant::Red,
                    color::ColorVariant::Green => ColorVariant::Green,
                    color::ColorVariant::Blue => ColorVariant::Blue,
                    color::ColorVariant::FujiGreen => ColorVariant::FujiGreen,
                    color::ColorVariant::Magenta => ColorVariant::Magenta,
                    color::ColorVariant::Yellow => ColorVariant::Yellow,
                    color::ColorVariant::Cyan => ColorVariant::Cyan,
                };
                data.push(e);
            }
        }
        Ok(Self {
            data: ColorFilterArray::new(data, RowLength::new(real_width)),
        })
    }
}

#[cfg(test)]
mod tests;
