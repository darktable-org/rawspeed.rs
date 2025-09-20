use super::super::height::Height;
use super::super::width::Width;
use crate::camerasxml_parser::colorrow::{self, ColorVariant};
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::{RowLength, RowPitch};
use rawspeed_std_ndslice::array2dref::Array2DRef;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Matrix {
    data: Vec<ColorVariant>,
    row_length: RowLength,
}

impl Matrix {
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

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Matrix {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut rows = Vec::<Vec<ColorVariant>>::new();
        while let Ok(row) = input.parse::<colorrow::ColorRow>() {
            if (**row.y).try_into() != Ok(rows.len()) {
                return Err(format!(
                    "unexpected row index, expected {} got {}",
                    rows.len(),
                    **row.y
                ));
            }
            if let Some(first_row) = rows.first()
                && let first_row_length = first_row.len()
                && let curr_row_length = row.value.values.len()
                && curr_row_length != first_row_length
            {
                return Err(format!(
                    "inconsistent row length, expected {first_row_length} got {curr_row_length}",
                ));
            }
            rows.push(row.value.values);
        }
        if rows.is_empty() {
            return Err(
                "unexpected end of input, expected `ColorRow`".to_owned()
            );
        }
        let matrix_elts = rows.iter().flat_map(|row| row.iter().copied());
        let data: Vec<ColorVariant> = matrix_elts.collect();
        Ok(Matrix::new(
            data,
            RowLength::new(rows.first().unwrap().len()),
        ))
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct CFA2 {
        width: Width,
        height: Height,
        body: Matrix,
    }
);

#[cfg(test)]
mod tests;
