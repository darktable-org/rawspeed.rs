use super::height;
use super::width;
use super::x;
use super::xmlparser;
use super::y;

impl_elt_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Crop {
        x: x::X,
        y: y::Y,
        width: width::Width,
        height: height::Height,
    }
);

#[cfg(test)]
mod tests;
