use rawspeed_common::bit_transmutation::ToLeBytes;
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser;
use rawspeed_misc_md5::md5::MD5;
use rawspeed_parsers_rawparser::rawparser::RawParser;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use std::env;
use std::fs;

pub trait AsSlice {
    type Element;

    fn as_slice(&self) -> &[Self::Element];
}

impl<T, const N: usize> AsSlice for [T; N] {
    type Element = T;

    fn as_slice(&self) -> &[Self::Element] {
        self.as_slice()
    }
}

#[inline(never)]
fn img_data_hash<T>(img: Array2DRef<'_, T>) -> String
where
    T: Copy + ToLeBytes,
    <T as ToLeBytes>::Output: AsSlice<Element = u8>,
{
    let mut per_row_states = vec![];
    for row in 0..img.num_rows() {
        let row = &img[RowIndex::new(row)];
        let mut hasher = MD5::default();
        for e in row {
            hasher.extend(e.to_le_bytes().as_slice());
        }
        per_row_states.push(hasher.flush());
    }
    let mut hasher = MD5::default();
    for e in per_row_states.iter().flat_map(|e| e.iter()) {
        hasher.extend(e.to_le_bytes().as_slice());
    }
    hasher.flush().into()
}

#[expect(clippy::print_stdout)]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let camerasxml_path = "/home/lebedevri/rawspeed/data/cameras.xml";

    let camerasxml_contents = fs::read_to_string(camerasxml_path)
        .expect("Should have been able to read the `cameras.xml");

    let cameras = camerasxml_parser::parse_str(&camerasxml_contents)?;

    let file_path = args.get(1).expect("Usage: $ rstest <file>");

    let input = fs::read(file_path)
        .expect("Should have been able to read the raw file");

    let (res, out_buf_request) = RawParser::get_decoder(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .expect("No decoder");

    let mut output_buf = out_buf_request.fulfill().unwrap();
    let mut output = output_buf.get_mut();
    res.decode(&mut output).unwrap();

    let output: Array2DRef<'_, u16> = output.into();
    println!("Success!");
    println!("md5sum of per-line md5sums: {}", img_data_hash(output));

    Ok(())
}

#[cfg(test)]
mod tests;
