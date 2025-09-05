use rawspeed_std::coord_common::{RowLength, RowPitch};
use rawspeed_std_ndslice::array2dref::Array2DRef;

use super::xmlparser;

pub const COLUMN_COUNT: usize = 3;

pub type T = i16;

mod repr {
    use super::super::colormatrixrow;
    use super::super::planes;
    use super::xmlparser;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ColorMatrixRows {
        pub elts: Vec<colormatrixrow::PlaneValues>,
    }

    impl core::ops::Deref for ColorMatrixRows {
        type Target = [colormatrixrow::PlaneValues];

        fn deref(&self) -> &Self::Target {
            &self.elts
        }
    }

    impl<'a, 'b> xmlparser::Parse<'a, 'b> for ColorMatrixRows {
        fn parse(
            input: &'b mut xmlparser::ParseStream<'a>,
        ) -> xmlparser::Result<Self> {
            let mut rows = Vec::new();
            while let Ok(row) = input.parse::<colormatrixrow::ColorMatrixRow>()
            {
                if rows.len() != (*row.plane).into() {
                    return Err(format!(
                        "unexpected plane, got {} expected {}",
                        *row.plane,
                        rows.len()
                    ));
                }
                assert_eq!(row.values.len(), super::COLUMN_COUNT);
                rows.push(row.values);
            }
            Ok(Self { elts: rows })
        }
    }

    impl_elt_with_body_matcher!(
        #[derive(Debug, Clone, PartialEq)]
        struct ColorMatrix {
            planes: planes::Planes,
            rows: ColorMatrixRows,
        }
    );
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ColorMatrix {
    data: Vec<T>,
}

impl ColorMatrix {
    pub const fn new(data: Vec<T>) -> Self {
        let ret = Self { data };
        let _ = ret.mat();
        ret
    }

    #[inline]
    #[must_use]
    pub const fn mat(&self) -> Array2DRef<'_, T> {
        Array2DRef::new(
            self.data.as_slice(),
            RowLength::new(COLUMN_COUNT),
            RowPitch::new(COLUMN_COUNT),
        )
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for ColorMatrix {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mat = input.parse::<repr::ColorMatrix>()?;
        if mat.rows.len() != mat.planes.into() {
            return Err(format!(
                "unexpected color matrix row count, got {} expected {}",
                mat.rows.len(),
                mat.planes.val()
            ));
        }
        let matrix_elts = (*mat.rows)
            .iter()
            .flat_map(|row| row.values.iter().copied());
        let data = matrix_elts.collect();
        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests;
