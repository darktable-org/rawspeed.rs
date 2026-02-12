use super::{width, x, xmlparser};

impl_elt_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Vertical {
        x: x::X,
        width: width::Width,
    }
);

#[cfg(test)]
mod tests;
