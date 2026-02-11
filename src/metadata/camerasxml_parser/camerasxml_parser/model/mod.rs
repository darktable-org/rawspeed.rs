use super::{Str, xmlparser};

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Model<'a> {
        model: Str<'a>,
    }
);
