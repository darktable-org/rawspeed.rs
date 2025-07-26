use super::Str;
use super::xmlparser;

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Value<'a> {
        value: Str<'a>,
    }
);
