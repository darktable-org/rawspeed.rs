use rawspeed_metadata_colorfilterarray::colorfilterarray::ColorFilterArray;
use rawspeed_metadata_colorfilterarray::colorfilterarray::ColorVariant;
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;

use crate::camerasxml_parser::colorrow;

mod repr;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct CFA2 {
    data: ColorFilterArray,
}

impl CFA2 {
    pub fn take_cfa(self) -> ColorFilterArray {
        self.data
    }
}

impl core::ops::Deref for CFA2 {
    type Target = ColorFilterArray;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for CFA2 {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let cfa = input.parse::<repr::CFA2>()?;
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
                    colorrow::ColorVariant::R => ColorVariant::Red,
                    colorrow::ColorVariant::G => ColorVariant::Green,
                    colorrow::ColorVariant::B => ColorVariant::Blue,
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
