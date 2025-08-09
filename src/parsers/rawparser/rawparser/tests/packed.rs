use crate::rawparser::{RawParser, RawParserError};
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Cameras;
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::*;

#[test]
fn empty_buffer_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
            <Hints>
                <Hint name=\"filesize\" value=\"8\"/>
                <Hint name=\"full_width\" value=\"4\"/>
                <Hint name=\"full_height\" value=\"2\"/>
                <Hint name=\"order\" value=\"plain\"/>
            </Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![0; 24];
    let res = RawParser::get_decoder(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    );
    assert_eq!(
        res.err(),
        Some(RawParserError::DecoderError(
            "No known cameras match the given input size".to_owned()
        ))
    );
}

#[test]
fn unpack_8bytes_into_2_rows_of_4_elts_at_8bpc_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
            <Hints>
                <Hint name=\"filesize\" value=\"8\"/>
                <Hint name=\"full_width\" value=\"4\"/>
                <Hint name=\"full_height\" value=\"2\"/>
                <Hint name=\"order\" value=\"plain\"/>
            </Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![11, 12, 13, 14, 21, 22, 23, 24];
    let (res, out_buf_request) = RawParser::get_decoder(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_buf = out_buf_request.fulfill().unwrap();
    let mut output = output_buf.get_mut();
    res.decode(&mut output).unwrap();
    for row in 1..=2 {
        for col in 1..=4 {
            assert_eq!(
                output[Coord2D::new(
                    RowIndex::new(row - 1),
                    ColIndex::new(col - 1)
                )],
                (10 * row + col).try_into().unwrap()
            );
        }
    }
}
