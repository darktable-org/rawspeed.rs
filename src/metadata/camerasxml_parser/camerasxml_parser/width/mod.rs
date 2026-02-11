use super::{Int, xmlparser};

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Width {
        width: Int,
    }
);
