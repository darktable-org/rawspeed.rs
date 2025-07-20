use super::BodyStr;
use super::id_attr;
use super::xmlparser;

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Alias<'a> {
        id: Option<id_attr::Id<'a>>,
        value: BodyStr<'a>,
    }
);

#[cfg(test)]
mod tests;
