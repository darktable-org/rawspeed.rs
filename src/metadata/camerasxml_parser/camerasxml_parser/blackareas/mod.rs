use super::{horizontal, vertical, xmlparser};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum BlackArea {
    Vertical(vertical::Vertical),
    Horizontal(horizontal::Horizontal),
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct IndividualBlackAreas {
    pub areas: Vec<BlackArea>,
}

impl core::ops::Deref for IndividualBlackAreas {
    type Target = [BlackArea];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.areas
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IndividualBlackAreas {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut areas = vec![];
        let mut made_changes = true;
        while made_changes {
            made_changes = false;
            while let Ok(row) = input.parse() {
                areas.push(BlackArea::Vertical(row));
                made_changes = true;
            }
            while let Ok(row) = input.parse() {
                areas.push(BlackArea::Horizontal(row));
                made_changes = true;
            }
        }
        if areas.is_empty() {
            return Err(
                "unexpected end of input, expected black areas".to_owned()
            );
        }
        Ok(IndividualBlackAreas { areas })
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
