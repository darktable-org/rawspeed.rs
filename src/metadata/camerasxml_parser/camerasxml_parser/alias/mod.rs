use super::{BodyStr, id_attr, xmlparser};

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Alias<'a> {
        id: Option<id_attr::Id<'a>>,
        value: BodyStr<'a>,
    }
);

#[cfg(test)]
mod tests;
