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
    fn iso_speed(&self) -> Option<u32>;
    fn blacklevel(&self) -> Option<()>;
    fn whitelevel(&self) -> Option<()>;
    fn blacklevel_separate(&self) -> Option<()>;
    fn wb_coeffs(&self) -> Option<()>;
    fn colormatrix(&self) -> Option<()>;
    fn is_cfa(&self) -> Option<()>;
    fn cfa(&self) -> Option<()>;
    fn filters(&self) -> Option<()>;
    fn bpp(&self) -> Option<()>;
    fn cpp(&self) -> usize;
    fn datatype(&self) -> Option<()>;
    fn dim_uncropped(&self) -> Option<()>;
    fn dim_cropped(&self) -> Option<()>;
    fn crop_offset(&self) -> Option<()>;
    fn black_areas(&self) -> Option<()>;
    fn fuji_rotation_pos(&self) -> Option<()>;
    fn pixel_aspect_ratio(&self) -> Option<()>;
    fn bad_pixel_positions(&self) -> Option<()>;

    fn decode(
        &self,
        output: &mut Array2DRefMut<'_, u16>,
    ) -> Result<(), RawDemuxerError>;
}
