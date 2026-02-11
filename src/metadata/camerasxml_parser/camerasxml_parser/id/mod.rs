use super::{BodyStr, make, model, xmlparser};

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct ID<'a> {
        make: make::Make<'a>,
        model: model::Model<'a>,
        value: BodyStr<'a>,
    }
);

#[cfg(test)]
mod tests;
