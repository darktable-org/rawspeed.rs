use super::horizontal;
use super::vertical;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct IndividualBlackAreas {
    pub verticals: Vec<vertical::Vertical>,
    pub horizontals: Vec<horizontal::Horizontal>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IndividualBlackAreas {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut res = IndividualBlackAreas {
            verticals: vec![],
            horizontals: vec![],
        };
        let mut made_changes = true;
        while made_changes {
            made_changes = false;
            while let Ok(row) = input.parse() {
                res.verticals.push(row);
                made_changes = true;
            }
            while let Ok(row) = input.parse() {
                res.horizontals.push(row);
                made_changes = true;
            }
        }
        if res.verticals.is_empty() && res.horizontals.is_empty() {
            return Err(
                "unexpected end of input, expected black areas".to_owned()
            );
        }
        Ok(res)
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct BlackAreas {
        value: IndividualBlackAreas,
    }
);

#[cfg(test)]
mod tests;
