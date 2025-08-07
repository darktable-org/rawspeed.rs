use super::Str;
use super::xmlparser;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Supported {
    Supported,
    SupportedNoSamples,
    Unsupported,
    UnsupportedNoSamples,
    Unknown,
    UnknownNoSamples,
}

impl Supported {
    #[inline]
    #[must_use]
    pub const fn is_explicitly_supported(&self) -> bool {
        matches!(*self, Supported::Supported | Supported::SupportedNoSamples)
    }

    #[inline]
    #[must_use]
    pub const fn is_explicitly_unsupported(&self) -> bool {
        matches!(
            *self,
            Supported::Unsupported | Supported::UnsupportedNoSamples
        )
    }
}

impl Default for Supported {
    #[inline]
    fn default() -> Self {
        Self::Supported
    }
}

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SupportedStr<'a> {
        supported: Str<'a>,
    }
);

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Supported {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        Ok(
            match input.parse::<SupportedStr<'_>>().map_or("yes", |v| **v) {
                "yes" => Self::Supported,
                "no-samples" => Self::SupportedNoSamples,
                "no" => Self::Unsupported,
                "no-no-samples" => Self::UnsupportedNoSamples,
                "unknown" => Self::Unknown,
                "unknown-no-samples" => Self::UnknownNoSamples,
                s => return Err(format!("Invalid support enum: {s}")),
            },
        )
    }
}
