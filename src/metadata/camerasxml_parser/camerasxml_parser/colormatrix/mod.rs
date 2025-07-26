use super::colormatrixrow;
use super::planes;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct ColorMatrixRows {
    pub values: Vec<colormatrixrow::ColorMatrixRow>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for ColorMatrixRows {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        if values.is_empty() {
            return Err(
                "unexpected end of input, expected `ColorMatrixRow`".to_owned()
            );
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct ColorMatrix {
        planes: planes::Planes,
        values: ColorMatrixRows,
    }
);

#[cfg(test)]
mod tests;
