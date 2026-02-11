use super::{Int, xmlparser};

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Black {
        black: Int,
    }
);
