use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_codecs_packed_decoder::packed_decoder::Unpacker;
use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxer;
use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxerError;
use rawspeed_memory_nd_slice_procurement::ndsliceprocurement::NDSliceProcurementRequest;
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Camera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Cameras;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Hints;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Supported;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::blackareas::BlackArea;
use rawspeed_metadata_colorfilterarray::colorfilterarray::ColorVariant;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::Dimensions2D;
use rawspeed_std::coord_common::RowCount;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

fn parse_as_bitorder(s: &str) -> Option<BitOrder> {
    match s {
        "plain" => Some(BitOrder::LSB),
        "jpeg" => Some(BitOrder::MSB),
        "jpeg16" => Some(BitOrder::MSB16),
        "jpeg32" => Some(BitOrder::MSB32),
        _ => None,
    }
}

fn get_hint_with_name<'a>(hints: &Hints<'a>, name: &str) -> Option<&'a str> {
    hints
        .iter()
        .find(|hint| **(hint.name) == name)
        .map(|hint| **hint.value)
}

fn match_camera_by_filesize<'a>(
    camera: &'a Camera<'a>,
    input_len: usize,
) -> bool {
    let Some(hints) = camera.hints.as_ref() else {
        return false;
    };

    matches!(get_hint_with_name(hints, "filesize").map(str::parse), Some(Ok(filesize)) if input_len == filesize)
}

fn compute_pitch(
    total_bytecount: usize,
    row_count: u64,
) -> Result<usize, String> {
    assert!(total_bytecount > 0);
    assert!(row_count > 0);

    let total_bytecount: u64 = total_bytecount.try_into().unwrap();

    if !total_bytecount.is_multiple_of(row_count) {
        return Err("Input size is not multiple of the row count".to_owned());
    }

    let bytes_per_row = total_bytecount / row_count;
    assert!(bytes_per_row > 0);

    Ok(bytes_per_row.try_into().unwrap())
}

fn guess_bits(bytes_per_row: usize, num_cols: u64) -> Result<u64, String> {
    assert!(bytes_per_row > 0);
    assert!(num_cols > 0);

    let bytes_per_row: u64 = bytes_per_row.try_into().unwrap();

    let Some(bits_per_row) = bytes_per_row.checked_mul(8_u64) else {
        return Err("Overflow when computing per-row bit count".to_owned());
    };

    if !bits_per_row.is_multiple_of(num_cols) {
        return Err("Input size is not multiple of the column count".to_owned());
    }
    let bits_per_pixel = bits_per_row / num_cols;
    assert!(bits_per_pixel > 0);

    Ok(bits_per_pixel)
}

#[derive(Debug)]
pub struct NakedDemuxer<'a> {
    camera: &'a Camera<'a>,
    input: Array2DRef<'a, u8>,
    dims: Dimensions2D,
    order: BitOrder,
    bits: u64,
}

type T = u16;

impl<'a> NakedDemuxer<'a> {
    #![expect(clippy::unwrap_in_result)]
    #[inline(never)]
    pub fn new<F>(
        input: &'a [u8],
        cameras: &'a Cameras<'a>,
        check_camera_support_fn: F,
    ) -> Result<(Self, NDSliceProcurementRequest<T>), String>
    where
        F: FnOnce(Supported) -> Result<DecodeableCamera, String>,
    {
        if input.is_empty() {
            return Err("Input buffer must be non-empty".to_owned());
        }

        let Some(camera) = cameras
            .cameras
            .values
            .iter()
            .find(|camera| match_camera_by_filesize(camera, input.len()))
        else {
            return Err(
                "No known cameras match the given input size".to_owned()
            );
        };

        check_camera_support_fn(camera.supported)?;

        let hints = camera.hints.as_ref().unwrap();

        let input = match get_hint_with_name(hints, "offset")
            .map_or(Ok(0), str::parse)
            .map(|offset| input.get(offset..))
        {
            Ok(Some(input)) if !input.is_empty() => input,
            Err(err) => return Err(format!("{err:?}")),
            _ => return Err("The specified offset is invalid".to_owned()),
        };

        let (col_count, row_count) = match (
            get_hint_with_name(hints, "full_width").map(str::parse),
            get_hint_with_name(hints, "full_height").map(str::parse),
        ) {
            (Some(Ok(w)), Some(Ok(h))) if w > 0 && h > 0 => (w, h),
            (_, _) => return Err("The width/height is invalid".to_owned()),
        };

        let input_bytes_per_row = compute_pitch(input.len(), row_count)?;
        let src = Array2DRef::new(
            input,
            RowLength::new(input_bytes_per_row),
            RowPitch::new(input_bytes_per_row),
        );

        let Some(order) = get_hint_with_name(hints, "order")
            .map_or(Some(BitOrder::MSB16), parse_as_bitorder)
        else {
            return Err("The bit order is invalid".to_owned());
        };

        let bits = match get_hint_with_name(hints, "bits").map(str::parse) {
            Some(Ok(bits)) => bits,
            None => guess_bits(input_bytes_per_row, col_count)?,
            _ => return Err("The bitwidth is invalid".to_owned()),
        };

        if !(1_u64..=T::BITS.into()).contains(&bits) {
            return Err(format!(
                "The bitwidth is must be in [1..{:?}]",
                T::BITS
            ));
        }

        let dims = Dimensions2D::new(
            RowLength::new(col_count.try_into().unwrap()),
            RowCount::new(row_count.try_into().unwrap()),
        );
        Ok((
            Self {
                camera,
                input: src,
                dims,
                order,
                bits,
            },
            NDSliceProcurementRequest::new(dims),
        ))
    }
}

impl RawDemuxer for NakedDemuxer<'_> {
    #[inline]
    fn make(&self) -> &str {
        self.camera.make.as_ref()
    }

    #[inline]
    fn model(&self) -> &str {
        self.camera.model.as_ref()
    }

    #[inline]
    fn mode(&self) -> Option<&str> {
        self.camera.mode.map(|v| &***v)
    }

    #[inline]
    fn canonical_make(&self) -> &str {
        self.camera
            .id
            .map_or_else(|| self.make(), |id| id.make.as_ref())
    }

    #[inline]
    fn canonical_model(&self) -> &str {
        self.camera
            .id
            .map_or_else(|| self.model(), |id| id.model.as_ref())
    }

    #[inline]
    fn canonical_alias(&self) -> &str {
        self.model()
    }

    #[inline]
    fn canonical_id(&self) -> String {
        self.camera.id.map_or_else(
            || format!("{} {}", self.make(), self.model()),
            |id| id.value.to_string(),
        )
    }

    #[inline]
    fn iso_speed(&self) -> Option<u32> {
        None
    }

    #[inline]
    fn blacklevel(&self) -> Option<u16> {
        self.camera.sensors.get_for_iso(self.iso_speed()).map(|s| {
            let val = **s.black;
            val.try_into().unwrap()
        })
    }

    #[inline]
    fn whitelevel(&self) -> Option<u16> {
        self.camera.sensors.get_for_iso(self.iso_speed()).map(|s| {
            let val = **s.white;
            val.try_into().unwrap()
        })
    }

    #[inline]
    fn blacklevel_separate(&self) -> Option<()> {
        None
    }

    #[inline]
    fn wb_coeffs(&self) -> Option<()> {
        None
    }

    #[inline]
    fn colormatrix(&self) -> Option<Array2DRef<'_, i16>> {
        self.camera
            .colormatrices
            .as_ref()
            .map(|mat| mat.value.mat())
    }

    #[inline]
    fn is_cfa(&self) -> bool {
        self.camera.cfa.is_some()
    }

    #[inline]
    fn cfa(&self) -> Option<Array2DRef<'_, ColorVariant>> {
        self.camera.cfa.as_ref().map(|cfa| cfa.mat())
    }

    #[inline]
    fn filters(&self) -> Option<()> {
        None
    }

    #[inline]
    fn bpp(&self) -> usize {
        let bpc = match self.datatype() {
            rawspeed_demuxers_rawdemuxer::rawdemuxer::DataType::U16 => {
                size_of::<u16>()
            }
            _ => unreachable!(),
        };
        bpc.checked_mul(self.cpp()).unwrap()
    }

    #[inline]
    fn cpp(&self) -> usize {
        1
    }

    #[inline]
    fn datatype(&self) -> rawspeed_demuxers_rawdemuxer::rawdemuxer::DataType {
        rawspeed_demuxers_rawdemuxer::rawdemuxer::DataType::U16
    }

    #[inline]
    fn dim_uncropped(&self) -> Dimensions2D {
        self.dims
    }

    #[inline]
    fn dim_cropped(&self) -> Option<()> {
        None
    }

    #[inline]
    fn crop_offset(&self) -> Option<Coord2D> {
        Some(*self.camera.crop?.pos)
    }

    #[inline]
    fn black_areas(&self) -> Option<&[BlackArea]> {
        self.camera.blackareas.as_ref().map(|f| &*f.value.areas)
    }

    #[inline]
    fn fuji_rotation_pos(&self) -> Option<u32> {
        None
    }

    #[inline]
    fn pixel_aspect_ratio(&self) -> Option<f64> {
        None
    }

    #[inline]
    fn bad_pixel_positions(&self) -> Vec<Coord2D> {
        vec![]
    }

    #[inline(never)]
    fn decode(
        &self,
        output: &mut Array2DRefMut<'_, u16>,
    ) -> Result<(), RawDemuxerError> {
        if output.dims() != self.dims {
            return Err(RawDemuxerError::DecoderError(
                "Output buffer dimensions differ from expected".to_owned(),
            ));
        }

        Unpacker::new(
            self.input,
            self.order,
            self.bits.try_into().unwrap(),
            output,
        )
        .unpack();
        Ok(())
    }
}

#[cfg(test)]
mod tests;
