use super::{name, value, xmlparser};

impl_elt_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Hint<'a> {
        name: name::Name<'a>,
        value: value::Value<'a>,
    }
);

#[cfg(test)]
mod tests;
