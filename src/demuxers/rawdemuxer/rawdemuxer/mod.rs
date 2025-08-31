use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum RawDemuxerError {
    DecoderError(String),
}

impl core::fmt::Display for RawDemuxerError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RawDemuxerError::DecoderError(error) => {
                write!(f, "RawDemuxerError(DecoderError({error}))")
            }
        }
    }
}

pub trait RawDemuxer {
    fn make(&self) -> &str;
    fn model(&self) -> &str;
    fn mode(&self) -> Option<&str>;
    fn canonical_make(&self) -> &str;
    fn canonical_model(&self) -> &str;
    fn canonical_alias(&self) -> &str;
    fn canonical_id(&self) -> String;

    fn decode(
        &self,
        output: &mut Array2DRefMut<'_, u16>,
    ) -> Result<(), RawDemuxerError>;
}
