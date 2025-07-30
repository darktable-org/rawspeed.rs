use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_codecs_packed_decoder::packed_decoder::Unpacker;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Camera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Cameras;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Hints;
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
struct NakedDemuxer<'a> {
    input: Array2DRef<'a, u8>,
    col_count: u64,
    row_count: u64,
    order: BitOrder,
    bits: u64,
}

type T = u16;

impl<'a, 'b> NakedDemuxer<'a> {
    #![expect(clippy::unwrap_in_result)]
    pub fn new(
        input: &'a [u8],
        cameras: &'b Cameras<'a>,
    ) -> Result<Self, String> {
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

        let Some(order) =
            get_hint_with_name(hints, "order").and_then(parse_as_bitorder)
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

        Ok(Self {
            input: src,
            col_count,
            row_count,
            order,
            bits,
        })
    }

    pub fn decode(
        &self,
        output: &mut Array2DRefMut<'a, T>,
    ) -> Result<(), String> {
        if (output.row_length(), output.num_rows())
            != (
                self.col_count.try_into().unwrap(),
                self.row_count.try_into().unwrap(),
            )
        {
            return Err(
                "Output buffer dimensions differ from expected".to_owned()
            );
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
