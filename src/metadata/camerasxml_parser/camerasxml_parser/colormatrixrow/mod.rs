use super::BodyStr;
use super::Int;
use super::plane;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct PlaneValues {
    pub values: Vec<Int>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for PlaneValues {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let raw_values = input.parse::<BodyStr<'a>>()?;
        let mut values = vec![];
        for val in raw_values.split_ascii_whitespace() {
            if let Ok(val) = val.parse() {
                values.push(Int { val });
            } else {
                return Err(format!("Unable to parse `{val:?}` as an integer"));
            }
        }
        Ok(Self { values })
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
