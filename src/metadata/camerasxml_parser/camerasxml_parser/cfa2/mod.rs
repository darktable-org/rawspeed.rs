use super::super::camerasxml_parser::colorrow;
use super::height;
use super::width;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct CFA2Colors {
    pub values: Vec<colorrow::ColorRow>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for CFA2Colors {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        if values.is_empty() {
            return Err(
                "unexpected end of input, expected `ColorRow`".to_owned()
            );
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct CFA2 {
        width: width::Width,
        height: height::Height,
        values: CFA2Colors,
    }
);

#[cfg(test)]
mod tests;
