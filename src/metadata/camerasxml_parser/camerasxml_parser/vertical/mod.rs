use super::width;
use super::x;
use super::xmlparser;

impl_elt_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Vertical {
        x: x::X,
        width: width::Width,
    }
);

#[cfg(test)]
mod tests;
