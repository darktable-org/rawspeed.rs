use super::super::height::Height;
use super::super::width::Width;
use crate::camerasxml_parser::color::{self, ColorVariant};
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

#[derive(Debug, Clone, PartialEq)]
struct RowMajorMatrixBuilder<T> {
    rows: Vec<Vec<T>>,
}

impl<T> Default for RowMajorMatrixBuilder<T> {
    fn default() -> Self {
        Self {
            rows: Vec::default(),
        }
    }
}

impl<T> RowMajorMatrixBuilder<T> {
    fn append_row(&mut self, row: Vec<T>) -> Result<(), &'static str> {
        if let Some(width) = self.rows.first().map(Vec::len)
            && row.len() != width
        {
            return Err("Inconsistent row length");
        }
        self.rows.push(row);
        Ok(())
    }

    pub const fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn get_rows(self) -> Vec<Vec<T>> {
        self.rows
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Matrix {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let rows = {
            let mut mat = RowMajorMatrixBuilder::default();
            let mut row = Vec::new();
            while let Ok(elt) = input.parse::<color::Color>() {
                if (**elt.x) == 0 && !row.is_empty() {
                    mat.append_row(core::mem::take(&mut row))?;
                }
                if (**elt.y).try_into() != Ok(mat.row_count()) {
                    return Err(format!(
                        "unexpected row index, expected {} got {}",
                        mat.row_count(),
                        **elt.y
                    ));
                }
                if (**elt.x).try_into() != Ok(row.len()) {
                    return Err(format!(
                        "unexpected column index, expected {} got {}",
                        row.len(),
                        **elt.y
                    ));
                }
                row.push(elt.value);
            }
            if !row.is_empty() {
                mat.append_row(core::mem::take(&mut row))?;
            }
            mat.get_rows()
        };

        if rows.is_empty() {
            return Err("unexpected end of input, expected `Color`".to_owned());
        }
        let matrix_elts = rows.iter().flat_map(|row| row.iter().copied());
        let data = matrix_elts.collect();
        Ok(Matrix::new(
            data,
            RowLength::new(rows.first().unwrap().len()),
        ))
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct CFA {
        width: Width,
        height: Height,
        body: Matrix,
    }
);

#[cfg(test)]
mod tests;
