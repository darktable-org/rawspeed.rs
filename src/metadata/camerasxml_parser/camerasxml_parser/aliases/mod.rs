use super::alias;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct IndividualAliases<'a> {
    pub values: Vec<alias::Alias<'a>>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IndividualAliases<'a> {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        const EXPECTED_NAME: &str = "Alias";
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        if values.is_empty() {
            return Err(format!(
                "unexpected end of input, expected `{EXPECTED_NAME}`"
            ));
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct Aliases<'a> {
        value: IndividualAliases<'a>,
    }
);

#[cfg(test)]
mod tests;
