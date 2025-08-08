use crate::ndsliceprocurement::Align;
use crate::ndsliceprocurement::ByteCount;
use crate::ndsliceprocurement::EltCount;
use crate::ndsliceprocurement::{
    NDSliceProcurementRequest, NDSliceProcurementRequestError,
};
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std::coord_common::{
    ColIndex, Coord2D, Dimensions2D, RowCount, RowIndex, RowLength,
};

fn runtime_align_of<T>(x: &T) -> Align {
    let ptr: *const T = x;
    let addr = ptr as usize;
    let num_tz = addr.trailing_zeros();
    let align = 1_usize.checked_shl(num_tz).unwrap();
    Align::new(ByteCount::new(align)).unwrap()
}

#[test]
fn layout_u8_test() {
    type T = u8;
    type Params = ((usize, usize), usize, usize, usize);
    type Result = ((usize, usize), usize);
    let params: Vec<(Params, Result)> = vec![
        (((1, 1), 0, 1, 1), ((1, 1), 1)),
        (((1, 1), 0, 1, 2), ((1, 2), 1)),
        (((1, 1), 0, 2, 1), ((1, 2), 1)),
        (((1, 1), 0, 2, 2), ((1, 2), 1)),
        (((1, 1), 1, 1, 1), ((2, 1), 2)),
        (((1, 1), 1, 1, 2), ((2, 2), 2)),
        (((1, 1), 1, 2, 1), ((2, 2), 2)),
        (((1, 1), 1, 2, 2), ((2, 2), 2)),
        (((1, 2), 0, 1, 1), ((2, 1), 1)),
        (((1, 2), 0, 1, 2), ((2, 2), 1)),
        (((1, 2), 0, 2, 1), ((4, 2), 2)),
        (((1, 2), 0, 2, 2), ((4, 2), 2)),
        (((1, 2), 1, 1, 1), ((4, 1), 2)),
        (((1, 2), 1, 1, 2), ((4, 2), 2)),
        (((1, 2), 1, 2, 1), ((4, 2), 2)),
        (((1, 2), 1, 2, 2), ((4, 2), 2)),
        (((2, 1), 0, 1, 1), ((2, 1), 2)),
        (((2, 1), 0, 1, 2), ((2, 2), 2)),
        (((2, 1), 0, 2, 1), ((2, 2), 2)),
        (((2, 1), 0, 2, 2), ((2, 2), 2)),
        (((2, 1), 1, 1, 1), ((3, 1), 3)),
        (((2, 1), 1, 1, 2), ((3, 2), 3)),
        (((2, 1), 1, 2, 1), ((3, 2), 3)),
        (((2, 1), 1, 2, 2), ((3, 2), 3)),
        (((2, 2), 0, 1, 1), ((4, 1), 2)),
        (((2, 2), 0, 1, 2), ((4, 2), 2)),
        (((2, 2), 0, 2, 1), ((4, 2), 2)),
        (((2, 2), 0, 2, 2), ((4, 2), 2)),
        (((2, 2), 1, 1, 1), ((6, 1), 3)),
        (((2, 2), 1, 1, 2), ((6, 2), 3)),
        (((2, 2), 1, 2, 1), ((8, 2), 4)),
        (((2, 2), 1, 2, 2), ((8, 2), 4)),
    ];
    for (
        ((width, height), extra_row_padding, row_alignment, base_alignment),
        ((layout_size, layout_align), row_pitch),
    ) in params
    {
        let layout = NDSliceProcurementRequest::<T>::new(Dimensions2D::new(
            RowLength::new(width),
            RowCount::new(height),
        ))
        .set_extra_row_padding(EltCount::new(extra_row_padding))
        .set_row_alignment(Align::new(ByteCount::new(row_alignment)).unwrap())
        .set_base_alignment(Align::new(ByteCount::new(base_alignment)).unwrap())
        .get_layout();
        let expected_layout;
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            expected_layout = Ok((
                core::alloc::Layout::from_size_align_unchecked(
                    layout_size,
                    layout_align,
                ),
                RowPitch::new(row_pitch),
            ));
        }
        assert_eq!(expected_layout, layout);
    }
}

#[test]
fn layout_u16_test() {
    type T = u16;
    type Params = ((usize, usize), usize, usize, usize);
    type Result = ((usize, usize), usize);
    let params: Vec<(Params, Result)> = vec![
        (((1, 1), 0, 1, 1), ((2, 2), 1)),
        (((1, 1), 0, 1, 2), ((2, 2), 1)),
        (((1, 1), 0, 2, 1), ((2, 2), 1)),
        (((1, 1), 0, 2, 2), ((2, 2), 1)),
        (((1, 1), 1, 1, 1), ((4, 2), 2)),
        (((1, 1), 1, 1, 2), ((4, 2), 2)),
        (((1, 1), 1, 2, 1), ((4, 2), 2)),
        (((1, 1), 1, 2, 2), ((4, 2), 2)),
        (((1, 2), 0, 1, 1), ((4, 2), 1)),
        (((1, 2), 0, 1, 2), ((4, 2), 1)),
        (((1, 2), 0, 2, 1), ((4, 2), 1)),
        (((1, 2), 0, 2, 2), ((4, 2), 1)),
        (((1, 2), 1, 1, 1), ((8, 2), 2)),
        (((1, 2), 1, 1, 2), ((8, 2), 2)),
        (((1, 2), 1, 2, 1), ((8, 2), 2)),
        (((1, 2), 1, 2, 2), ((8, 2), 2)),
        (((2, 1), 0, 1, 1), ((4, 2), 2)),
        (((2, 1), 0, 1, 2), ((4, 2), 2)),
        (((2, 1), 0, 2, 1), ((4, 2), 2)),
        (((2, 1), 0, 2, 2), ((4, 2), 2)),
        (((2, 1), 1, 1, 1), ((6, 2), 3)),
        (((2, 1), 1, 1, 2), ((6, 2), 3)),
        (((2, 1), 1, 2, 1), ((6, 2), 3)),
        (((2, 1), 1, 2, 2), ((6, 2), 3)),
        (((2, 2), 0, 1, 1), ((8, 2), 2)),
        (((2, 2), 0, 1, 2), ((8, 2), 2)),
        (((2, 2), 0, 2, 1), ((8, 2), 2)),
        (((2, 2), 0, 2, 2), ((8, 2), 2)),
        (((2, 2), 1, 1, 1), ((12, 2), 3)),
        (((2, 2), 1, 1, 2), ((12, 2), 3)),
        (((2, 2), 1, 2, 1), ((12, 2), 3)),
        (((2, 2), 1, 2, 2), ((12, 2), 3)),
    ];
    for (
        ((width, height), extra_row_padding, row_alignment, base_alignment),
        ((layout_size, layout_align), row_pitch),
    ) in params
    {
        let layout = NDSliceProcurementRequest::<T>::new(Dimensions2D::new(
            RowLength::new(width),
            RowCount::new(height),
        ))
        .set_extra_row_padding(EltCount::new(extra_row_padding))
        .set_row_alignment(Align::new(ByteCount::new(row_alignment)).unwrap())
        .set_base_alignment(Align::new(ByteCount::new(base_alignment)).unwrap())
        .get_layout();
        let expected_layout;
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            expected_layout = Ok((
                core::alloc::Layout::from_size_align_unchecked(
                    layout_size,
                    layout_align,
                ),
                RowPitch::new(row_pitch),
            ));
        }
        assert_eq!(expected_layout, layout);
    }
}

fn test_config<T>(
    dims: Dimensions2D,
    extra_row_padding: EltCount,
    row_alignment: Align,
    base_alignment: Align,
) {
    let mut res = NDSliceProcurementRequest::<T>::new(dims)
        .set_extra_row_padding(extra_row_padding)
        .set_row_alignment(row_alignment)
        .set_base_alignment(base_alignment)
        .fulfill()
        .unwrap();
    let img = res.get_mut();
    assert!(
        runtime_align_of(
            &img[Coord2D::new(RowIndex::new(0), ColIndex::new(0))]
        ) >= base_alignment
    );
    for row in 0..*dims.row_count() {
        assert!(
            runtime_align_of(
                &img[Coord2D::new(RowIndex::new(row), ColIndex::new(0))]
            ) >= row_alignment
        );
    }
}

fn basic_test<T>() {
    for width in 1..=2 {
        for height in 1..=2 {
            let dims =
                Dimensions2D::new(RowLength::new(width), RowCount::new(height));
            for extra_row_padding in 0..=1 {
                let extra_row_padding = EltCount::new(extra_row_padding);
                for row_alignment in 0..=1 {
                    let row_alignment = Align::new(ByteCount::new(
                        1_usize.checked_shl(row_alignment).unwrap(),
                    ))
                    .unwrap();
                    for base_alignment in 0..=1 {
                        let base_alignment = Align::new(ByteCount::new(
                            1_usize.checked_shl(base_alignment).unwrap(),
                        ))
                        .unwrap();
                        test_config::<T>(
                            dims,
                            extra_row_padding,
                            row_alignment,
                            base_alignment,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn basic_u8_test() {
    type T = u8;
    basic_test::<T>();
}

#[test]
#[cfg_attr(miri, ignore)]
fn usable_max_test() {
    assert_eq!(
        Err(NDSliceProcurementRequestError::OutOfMemory),
        NDSliceProcurementRequest::<u8>::new(Dimensions2D::new(
            RowLength::new(isize::MAX.try_into().unwrap()),
            RowCount::new(1)
        ))
        .fulfill()
    );
}
