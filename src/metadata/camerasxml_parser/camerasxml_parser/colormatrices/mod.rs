use super::{colormatrix, xmlparser};

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct ColorMatrices {
        value: colormatrix::ColorMatrix,
    }
);

#[cfg(test)]
mod tests;
