use super::{BodyStr, plane, xmlparser};

type T = super::colormatrix::T;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneValues {
    pub values: [T; super::colormatrix::COLUMN_COUNT],
}

impl core::ops::Deref for PlaneValues {
    type Target = [T; super::colormatrix::COLUMN_COUNT];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for PlaneValues {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let raw_values = input.parse::<BodyStr<'a>>()?;
        let elts: Result<Vec<T>, _> = raw_values
            .split_ascii_whitespace()
            .map(str::parse::<T>)
            .collect();
        let Ok(elts) = elts else {
            return Err(format!(
                "Unable to parse plane components as integers: {}",
                elts.unwrap_err()
            ));
        };
        let Ok(elts) = (&*elts).try_into() else {
            return Err(format!(
                "Color matrix row must have {} components, got {}",
                super::colormatrix::COLUMN_COUNT,
                elts.len()
            ));
        };
        Ok(Self { values: elts })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct ColorMatrixRow {
        plane: plane::Plane,
        values: PlaneValues,
    }
);

#[cfg(test)]
mod tests;
