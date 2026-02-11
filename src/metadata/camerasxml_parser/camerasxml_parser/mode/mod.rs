use super::{Str, xmlparser};

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Mode<'a> {
        mode: Str<'a>,
    }
);
