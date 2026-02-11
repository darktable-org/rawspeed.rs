use super::{Str, xmlparser};

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Name<'a> {
        name: Str<'a>,
    }
);
