use super::camera;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct IndividualCameras<'a> {
    pub values: Vec<camera::Camera<'a>>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IndividualCameras<'a> {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        const EXPECTED_NAME: &str = "Camera";
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        if values.is_empty() {
            return Err(format!(
                "unexpected end of input, expected `{EXPECTED_NAME}`"
            ));
        }
        Ok(Self { values })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct Cameras<'a> {
        cameras: IndividualCameras<'a>,
    }
);

#[cfg(test)]
mod tests;
