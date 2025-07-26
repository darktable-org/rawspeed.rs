use super::Str;
use super::xmlparser;

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Supported<'a> {
        supported: Str<'a>,
    }
);
