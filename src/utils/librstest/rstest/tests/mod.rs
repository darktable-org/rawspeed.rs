use super::{AsSlice, Hash, img_data_hash, img_hash};
use crate::rstest::camerasxml_parser::Cameras;
use rawspeed_common_generic_num::generic_num::{
    bit_transmutation::ToLeBytes, common::Bitwidth,
};
use rawspeed_memory_nd_slice_procurement::ndsliceprocurement::NDSliceProcurementRequest;
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_parsers_rawparser::rawparser::RawParser;
use rawspeed_std::coord_common::{
    ColIndex, Coord2D, Dimensions2D, RowCount, RowIndex, RowLength,
};

use test_file_system::TestFileSystem;
mod test_file_system;

use test_logger::TestLogger;
mod test_logger;

fn from_halves<T>(hi: T, lo: T) -> T
where
    T: Bitwidth + core::ops::Shl<usize>,
    <T as core::ops::Shl<usize>>::Output: core::ops::BitOr<T, Output = T>,
{
    let bitwidth = T::BITWIDTH;
    assert!(bitwidth.is_multiple_of(2));
    (hi << (bitwidth / 2)) | lo
}

fn image_hash_test<T>(dims: Dimensions2D, str: &'static str)
where
    T: Bitwidth + core::ops::Shl<usize> + Copy + TryFrom<usize> + ToLeBytes,
    <T as core::ops::Shl<usize>>::Output: core::ops::BitOr<T, Output = T>,
    <T as ToLeBytes>::Output: AsSlice<Element = u8>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let mut out = NDSliceProcurementRequest::<T>::new(dims).fulfill().unwrap();
    let mut img = out.get_mut();
    for row in 1..=*dims.row_count() {
        for col in 1..=*dims.row_len() {
            img[Coord2D::new(RowIndex::new(row - 1), ColIndex::new(col - 1))] =
                from_halves::<T>(
                    row.try_into().unwrap(),
                    col.try_into().unwrap(),
                );
        }
    }
    let res = img_data_hash(img.into());
    assert_eq!(res, str);
}

#[test]
fn image_hash_u8_test() {
    type T = u8;
    let params = vec![
        ((1, 1), "51bfa529a7c5f047217dc0867aa731d4"),
        ((1, 2), "db1d3730e3696baac948d434f9080b0b"),
        ((2, 1), "bd2a3ff9e3c81c01a83f6372df0d867a"),
        ((2, 2), "bfc1a79553bcb9377781ddad623cf39b"),
    ];
    for ((w, h), str) in params {
        image_hash_test::<T>(
            Dimensions2D::new(RowLength::new(w), RowCount::new(h)),
            str,
        );
    }
}

#[test]
fn image_hash_u16_test() {
    type T = u16;
    let params = vec![
        ((1, 1), "cfebf85e838306c289a4cc291e544f9e"),
        ((1, 2), "24a03faae6de848468d1b539f162c032"),
        ((2, 1), "14ce3541e1b1eecefd479aaf148ca556"),
        ((2, 2), "5fb0ee343ec52da356b1b20c719c78df"),
    ];
    for ((w, h), str) in params {
        image_hash_test::<T>(
            Dimensions2D::new(RowLength::new(w), RowCount::new(h)),
            str,
        );
    }
}

const REF_CAMERAS: &str = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\" mode=\"A Mode\">
            <ID make=\"Canonical Make\" model=\"Canonical Model\">Canonical ID</ID>
            <CFA width=\"2\" height=\"2\">
                <Color x=\"0\" y=\"0\">RED</Color>
                <Color x=\"1\" y=\"0\">GREEN</Color>
                <Color x=\"0\" y=\"1\">GREEN</Color>
                <Color x=\"1\" y=\"1\">BLUE</Color>
            </CFA>
            <Crop x=\"0\" y=\"1\" width=\"-1\" height=\"0\"/>
            <Sensor black=\"16\" white=\"255\"/>
            <BlackAreas>
                <Vertical x=\"10\" width=\"20\"/>
                <Horizontal y=\"30\" height=\"40\"/>
            </BlackAreas>
            <Hints>
                <Hint name=\"filesize\" value=\"8\"/>
                <Hint name=\"full_width\" value=\"4\"/>
                <Hint name=\"full_height\" value=\"2\"/>
                <Hint name=\"order\" value=\"plain\"/>
            </Hints>
            <ColorMatrices>
                <ColorMatrix planes=\"3\">
                    <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
                </ColorMatrix>
            </ColorMatrices>
        </Camera>
    </Cameras>";

const REF_INPUT: [u8; 8] = [11, 12, 13, 14, 21, 22, 23, 24];

const REF_HASH: &str = concat!(
    "make: Make\n",
    "model: Model\n",
    "mode: A Mode\n",
    "canonical_make: Canonical Make\n",
    "canonical_model: Canonical Model\n",
    "canonical_alias: Model\n",
    "canonical_id: Canonical ID\n",
    "isoSpeed: 0\n",
    "blackLevel: 16\n",
    "whitePoint: 255\n",
    "blackLevelSeparate: none\n",
    "wbCoeffs: (none)\n",
    "colorMatrix: 0/10000 1/10000 2/10000 3/10000 -4/10000 5/10000 6/10000 7/10000 8/10000\n",
    "isCFA: 1\n",
    "cfa: RED,GREEN\n",
    "GREEN,BLUE\n",
    "\n",
    "filters: 0x94949494\n",
    "bpp: 2\n",
    "cpp: 1\n",
    "dataType: 0\n",
    "dimUncropped: 4x2\n",
    "dimCropped: 3x1\n",
    "cropOffset: 0x1\n",
    "blackAreas: 1:10x20, 0:30x40, \n",
    "fuji_rotation_pos: 0\n",
    "pixel_aspect_ratio: 1.000000\n",
    "badPixelPositions: \n",
    "md5sum of per-line md5sums: 441ee0c3c5e0033cde9d9dcee7ac46fb\n",
);

#[test]
fn image_hashfile_test() {
    let cameras = xmlparser::parse_str::<Cameras<'_>>(REF_CAMERAS).unwrap();
    let (res, out_buf_request) = RawParser::get_decoder(
        &REF_INPUT,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_buf = out_buf_request.fulfill().unwrap();
    let mut output = output_buf.get_mut();
    res.decode(&mut output).unwrap();
    assert_eq!(
        Hash {
            hash: REF_HASH.to_owned()
        },
        img_hash(&*res, output.into())
    );
}

const REF_CAMERAS_BAREBONES: &str = "
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

const REF_HASH_BAREBONES: &str = concat!(
    "make: Make\n",
    "model: Model\n",
    "mode: \n",
    "canonical_make: Make\n",
    "canonical_model: Model\n",
    "canonical_alias: Model\n",
    "canonical_id: Make Model\n",
    "isoSpeed: 0\n",
    "blackLevel: -1\n",
    "whitePoint: unknown\n",
    "blackLevelSeparate: none\n",
    "wbCoeffs: (none)\n",
    "colorMatrix: (none)\n",
    "isCFA: 0\n",
    "cfa: \n",
    "filters: 0x1\n",
    "bpp: 2\n",
    "cpp: 1\n",
    "dataType: 0\n",
    "dimUncropped: 4x2\n",
    "dimCropped: 4x2\n",
    "cropOffset: 0x0\n",
    "blackAreas: \n",
    "fuji_rotation_pos: 0\n",
    "pixel_aspect_ratio: 1.000000\n",
    "badPixelPositions: \n",
    "md5sum of per-line md5sums: 441ee0c3c5e0033cde9d9dcee7ac46fb\n",
);

#[test]
fn image_barebones_hashfile_test() {
    let cameras =
        xmlparser::parse_str::<Cameras<'_>>(REF_CAMERAS_BAREBONES).unwrap();
    let (res, out_buf_request) = RawParser::get_decoder(
        &REF_INPUT,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_buf = out_buf_request.fulfill().unwrap();
    let mut output = output_buf.get_mut();
    res.decode(&mut output).unwrap();
    assert_eq!(
        Hash {
            hash: REF_HASH_BAREBONES.to_owned()
        },
        img_hash(&*res, output.into())
    );
}

#[inline(always)]
#[expect(clippy::inline_always)]
fn assert_contains(patterns: &[&str], input: &[String]) {
    assert_eq!(patterns.len(), input.len());
    for (a, str) in patterns.iter().zip(input) {
        assert!(str.starts_with(a), "needle: {a}\nstr: {str}");
    }
}

mod fs {
    mod create_unless_exists;
    mod decode_and_verify_if_exists;
    mod recreate;
    mod verify_if_exists;
}
