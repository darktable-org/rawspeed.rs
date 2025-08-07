use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Supported;

#[derive(Debug)]
#[non_exhaustive]
pub struct DecodeableCamera;

impl DecodeableCamera {
    #[inline]
    pub fn new_if_supported(supported: Supported) -> Result<Self, String> {
        if !supported.is_explicitly_supported() {
            return Err("This camera is not supported".to_owned());
        }

        Ok(Self {})
    }

    #[inline]
    pub fn new_unless_unsupported(
        supported: Supported,
    ) -> Result<Self, String> {
        if supported.is_explicitly_unsupported() {
            return Err("This camera is not supported (explicit)".to_owned());
        }

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests;
