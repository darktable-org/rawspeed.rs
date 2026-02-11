use super::{camera, xmlparser};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct IndividualCameras<'a> {
    pub values: Vec<camera::Camera<'a>>,
}

impl<'a> core::ops::Deref for IndividualCameras<'a> {
    type Target = [camera::Camera<'a>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.values.as_slice()
    }
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

impl<'a> core::ops::Deref for Cameras<'a> {
    type Target = [camera::Camera<'a>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cameras
    }
}

#[cfg(test)]
mod tests;
