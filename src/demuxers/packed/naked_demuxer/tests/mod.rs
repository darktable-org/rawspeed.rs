macro_rules! impl_generic_tests {
    ($bitorder:expr) => {
        use crate::naked_demuxer::NakedDemuxer;
        use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Cameras;
        use rawspeed_metadata_xmlparser::xmlparser;
        use rawspeed_std::coord_common::*;
        use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;
        use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxer as _;
        use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;

        #[test]
        fn empty_input_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 0];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "Input buffer must be non-empty");
        }

        #[test]
        fn no_cameras_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 1];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "No known cameras match the given input size"
            );
        }

        #[test]
        fn invalid_filesize_hint_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8u\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "No known cameras match the given input size"
            );
        }

        #[test]
        fn no_width_and_height_hints_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The width/height is invalid");
        }

        #[test]
        fn no_height_hint_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The width/height is invalid");
        }

        #[test]
        fn no_width_hint_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The width/height is invalid");
        }

        #[test]
        fn no_bitorder_hint_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn invalid_bitorder_hint_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"what?\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The bit order is invalid");
        }

        #[test]
        fn guessed_bits_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn specified_bits_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                        <Hint name=\"bits\" value=\"8\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn specified_bits_invalid_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                        <Hint name=\"bits\" value=\"8u\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The bitwidth is invalid");
        }

        #[test]
        fn specified_zero_bits_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                        <Hint name=\"bits\" value=\"0\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The bitwidth is must be in [1..16]");
        }

        #[test]
        fn specified_too_large_bits_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"68\"/>
                        <Hint name=\"full_width\" value=\"32\"/>
                        <Hint name=\"full_height\" value=\"1\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                        <Hint name=\"bits\" value=\"17\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 68];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The bitwidth is must be in [1..16]");
        }

        #[test]
        fn offset_but_no_cameras_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"9\"/>
                        <Hint name=\"offset\" value=\"1\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "No known cameras match the given input size"
            );
        }

        #[test]
        fn bad_offset_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"offset\" value=\"8\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The specified offset is invalid");
        }

        #[test]
        fn offset_no_width_height_test() {
            let cameras = "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"9\"/>
                        <Hint name=\"offset\" value=\"1\"/>
                    </Hints>
                    </Camera>
                </Cameras>";
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 9];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(res.unwrap_err(), "The width/height is invalid");
        }

        #[test]
        fn offset_guessed_bits_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"9\"/>
                        <Hint name=\"offset\" value=\"1\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 9];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn unpack_into_wrong_output_dimensions_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![11, 12, 13, 14, 21, 22, 23, 24];
            let (res, _) = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported)
            .unwrap();
            let width = 4;
            let height = 2;
            for test_width in 1..(2 * width * height) {
                for test_height in 1..(2 * width * height) {
                    let mut output_storage =
                        vec![0_u16; test_height * test_width];
                    let mut output = Array2DRefMut::new(
                        &mut output_storage,
                        RowLength::new(test_width),
                        RowPitch::new(test_width),
                    );
                    let res = res.decode(&mut output);
                    if test_width == width && test_height == height {
                        res.unwrap();
                    } else {
                        assert_eq!(
                            res.unwrap_err(),
                            "Output buffer dimensions differ from expected"
                        );
                    }
                }
            }
        }

        #[test]
        fn filesize_not_multiple_of_row_count_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"9\"/>
                        <Hint name=\"full_width\" value=\"8\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 9];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "Input size is not multiple of the row count"
            );
        }

        #[test]
        fn filesize_not_multiple_of_column_count_gt_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"5\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "Input size is not multiple of the column count"
            );
        }

        #[test]
        fn filesize_not_multiple_of_column_count_lt_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"3\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                res.unwrap_err(),
                "Input size is not multiple of the column count"
            );
        }

        #[test]
        fn implicit_camera_support_if_supported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_if_supported);
            res.unwrap();
        }

        #[test]
        fn implicit_camera_support_unless_unsupported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn explicit_camera_support_if_supported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"yes\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_if_supported);
            res.unwrap();
        }

        #[test]
        fn explicit_camera_support_unless_unsupported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"yes\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }

        #[test]
        fn missing_camera_support_if_supported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"no\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_if_supported);
            assert_eq!("This camera is not supported", res.unwrap_err());
        }

        #[test]
        fn missing_camera_support_unless_unsupported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"no\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            assert_eq!(
                "This camera is not supported (explicit)",
                res.unwrap_err()
            );
        }

        #[test]
        fn unknown_camera_support_if_supported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"unknown\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_if_supported);
            assert_eq!("This camera is not supported", res.unwrap_err());
        }

        #[test]
        fn unknown_camera_support_unless_unsupported_test() {
            let cameras = concat!(
                "
                <Cameras>
                    <Camera make=\"Make\" model=\"Model\" supported=\"unknown\">
                    <Hints>
                        <Hint name=\"filesize\" value=\"8\"/>
                        <Hint name=\"full_width\" value=\"4\"/>
                        <Hint name=\"full_height\" value=\"2\"/>
                        <Hint name=\"order\" value=\"",
                $bitorder,
                "\"/>
                    </Hints>
                    </Camera>
                </Cameras>"
            );
            let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
            let input = vec![0_u8; 8];
            let res = NakedDemuxer::new(&input, &cameras, DecodeableCamera::new_unless_unsupported);
            res.unwrap();
        }
    };
}

#[cfg(test)]
mod lsb;

#[cfg(test)]
mod msb;

#[cfg(test)]
mod msb16;

#[cfg(test)]
mod msb32;
