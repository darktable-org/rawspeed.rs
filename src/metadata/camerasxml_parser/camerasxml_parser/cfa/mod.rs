use super::color;
use super::height;
use super::width;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct CFAColors {
    pub values: Vec<color::Color>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for CFAColors {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        if values.is_empty() {
            return Err("unexpected end of input, expected `Color`".to_owned());
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct CFA {
        width: width::Width,
        height: height::Height,
        values: CFAColors,
    }
);

#[cfg(test)]
mod tests;
