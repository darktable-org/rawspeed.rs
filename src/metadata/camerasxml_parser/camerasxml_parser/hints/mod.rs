use super::{hint, xmlparser};

#[derive(Debug, Clone, PartialEq)]
pub struct IndividualHints<'a> {
    pub values: Vec<hint::Hint<'a>>,
}

impl<'a> core::ops::Deref for IndividualHints<'a> {
    type Target = [hint::Hint<'a>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IndividualHints<'a> {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut values = Vec::new();
        while let Ok(hint) = input.parse() {
            values.push(hint);
        }
        if values.is_empty() {
            return Err("unexpected end of input, expected `Hint`".to_owned());
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct Hints<'a> {
        value: IndividualHints<'a>,
    }
);

impl<'a> core::ops::Deref for Hints<'a> {
    type Target = [hint::Hint<'a>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests;
