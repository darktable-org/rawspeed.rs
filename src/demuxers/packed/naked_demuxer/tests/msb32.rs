impl_generic_tests!("jpeg32");

#[test]
fn unpack_8bytes_into_2_rows_of_4_elts_at_8bpc_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
		<Hints>
			<Hint name=\"filesize\" value=\"8\"/>
			<Hint name=\"full_width\" value=\"4\"/>
			<Hint name=\"full_height\" value=\"2\"/>
            <Hint name=\"order\" value=\"jpeg32\"/>
		</Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![14, 13, 12, 11, 24, 23, 22, 21];
    let (res, out_buf_request) = NakedDemuxer::new(
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

#[test]
fn unpack_8bytes_into_2_rows_of_2_elts_at_16bpc_test() {
    let cameras = "
    <Cameras>
        <Camera make=\"Make\" model=\"Model\">
		<Hints>
			<Hint name=\"filesize\" value=\"8\"/>
			<Hint name=\"full_width\" value=\"2\"/>
			<Hint name=\"full_height\" value=\"2\"/>
            <Hint name=\"order\" value=\"jpeg32\"/>
		</Hints>
        </Camera>
    </Cameras>";
    let cameras = xmlparser::parse_str::<Cameras<'_>>(cameras).unwrap();
    let input = vec![12, 0, 11, 0, 22, 0, 21, 0];
    let (res, out_buf_request) = NakedDemuxer::new(
        &input,
        &cameras,
        DecodeableCamera::new_unless_unsupported,
    )
    .unwrap();
    let mut output_buf = out_buf_request.fulfill().unwrap();
    let mut output = output_buf.get_mut();
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
