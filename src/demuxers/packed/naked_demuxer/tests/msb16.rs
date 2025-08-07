use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;

impl_generic_tests!("jpeg16");

#[test]
fn unpack_8bytes_into_2_rows_of_4_elts_at_8bpc_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
		<Hints>
			<Hint name=\"filesize\" value=\"8\"/>
			<Hint name=\"full_width\" value=\"4\"/>
			<Hint name=\"full_height\" value=\"2\"/>
            <Hint name=\"order\" value=\"jpeg16\"/>
		</Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![12, 11, 14, 13, 22, 21, 24, 23];
    let res = NakedDemuxer::new(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_storage = vec![0_u16; 8];
    let mut output = Array2DRefMut::new(
        &mut output_storage,
        RowLength::new(4),
        RowPitch::new(4),
    );
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

#[test]
fn unpack_8bytes_into_2_rows_of_2_elts_at_16bpc_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
		<Hints>
			<Hint name=\"filesize\" value=\"8\"/>
			<Hint name=\"full_width\" value=\"2\"/>
			<Hint name=\"full_height\" value=\"2\"/>
            <Hint name=\"order\" value=\"jpeg16\"/>
		</Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![11, 0, 12, 0, 21, 0, 22, 0];
    let res = NakedDemuxer::new(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_storage = vec![0_u16; 4];
    let mut output = Array2DRefMut::new(
        &mut output_storage,
        RowLength::new(2),
        RowPitch::new(2),
    );
    res.decode(&mut output).unwrap();
    for row in 1..=2 {
        for col in 1..=2 {
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
