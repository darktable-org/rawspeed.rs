use super::{AsSlice, Hash, img_data_hash, img_hash};
use crate::rstest::camerasxml_parser::Cameras;
use rawspeed_common::bit_transmutation::ToLeBytes;
use rawspeed_common::common::Bitwidth;
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
            <Hints>
                <Hint name=\"filesize\" value=\"8\"/>
                <Hint name=\"full_width\" value=\"4\"/>
                <Hint name=\"full_height\" value=\"2\"/>
                <Hint name=\"order\" value=\"plain\"/>
            </Hints>
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
    "blackLevel: FIXME\n",
    "whitePoint: FIXME\n",
    "blackLevelSeparate: FIXME\n",
    "wbCoeffs: FIXME\n",
    "colorMatrix: FIXME\n",
    "isCFA: FIXME\n",
    "cfa: FIXME\n",
    "filters: FIXME\n",
    "bpp: FIXME\n",
    "cpp: 1\n",
    "dataType: 0\n",
    "dimUncropped: FIXME\n",
    "dimCropped: FIXME\n",
    "cropOffset: FIXME\n",
    "blackAreas: FIXME\n",
    "fuji_rotation_pos: FIXME\n",
    "pixel_aspect_ratio: FIXME\n",
    "badPixelPositions: FIXME\n",
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
    "blackLevel: FIXME\n",
    "whitePoint: FIXME\n",
    "blackLevelSeparate: FIXME\n",
    "wbCoeffs: FIXME\n",
    "colorMatrix: FIXME\n",
    "isCFA: FIXME\n",
    "cfa: FIXME\n",
    "filters: FIXME\n",
    "bpp: FIXME\n",
    "cpp: 1\n",
    "dataType: 0\n",
    "dimUncropped: FIXME\n",
    "dimCropped: FIXME\n",
    "cropOffset: FIXME\n",
    "blackAreas: FIXME\n",
    "fuji_rotation_pos: FIXME\n",
    "pixel_aspect_ratio: FIXME\n",
    "badPixelPositions: FIXME\n",
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
