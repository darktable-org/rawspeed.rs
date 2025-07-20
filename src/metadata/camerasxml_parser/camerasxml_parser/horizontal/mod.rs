use super::height;
use super::xmlparser;
use super::y;

impl_elt_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Horizontal {
        y: y::Y,
        height: height::Height,
    }
);

#[cfg(test)]
mod tests;
