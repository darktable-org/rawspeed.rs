use rawspeed_std::coord_common::{
    ColOffset, CoordOffset2D, RowLength, RowOffset, RowPitch,
};

use crate::{array2dref::Array2DRef, offsetarray2dref::OffsetArray2DRef};

macro_rules! test {
    ($($data:tt r<< Offset(Row($row_off:expr), Col($col_off:expr)) == $view:tt,)+) => {
        $(
            {
                let view = $view;
                let origin = CoordOffset2D::new(
                    RowOffset::new($row_off),
                    ColOffset::new($col_off),
                );
                let view_vec = view
                    .into_iter()
                    .map(|row| row.into_iter().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let row_length = core::num::NonZero::new($data.get(0).unwrap().len()).unwrap();
                for padding_len in 0..=1 {
                    let data_storage = $data
                        .iter()
                        .map(|row| row.as_slice())
                        .flat_map(|row| {
                            row.iter().copied().chain(core::iter::repeat_n(
                                Default::default(),
                                padding_len,
                            ))
                        })
                        .collect::<Vec<u8>>();
                    let base = Array2DRef::new(
                        &data_storage,
                        RowLength::new(row_length),
                        RowPitch::new(row_length.checked_add(padding_len).unwrap()),
                    );
                    let input = OffsetArray2DRef::new(base, origin);
                    let mut res = vec![];
                    for row in input.rows() {
                        res.push(vec![]);
                        let out = res.last_mut().unwrap();
                        for col in row.cols() {
                            out.push(*col);
                        }
                    }
                    assert_eq!(view_vec, res);
                }
            }
        )+
    };
}

#[test]
fn basic_test() {
    test!(
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]] r<< Offset(Row(1), Col(2)) == [[7, 8, 5, 6], [11, 12, 9, 10], [3, 4, 1, 2]],
    );
}

#[test]
#[should_panic(
    expected = "right: [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]"
)]
fn basic_negative_test() {
    test!(
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]] r<< Offset(Row(0), Col(0)) == [[7, 8, 5, 6], [11, 12, 9, 10], [3, 4, 1, 2]],
    );
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn exhaustive_test() {
    let max_size = {
        #[cfg(not(miri))]
        {
            8
        }
        #[cfg(miri)]
        {
            3
        }
    };
    for rows in 1..=max_size {
        for cols in 1..=max_size {
            type T = u8;
            let data_storage = (1..=(rows * cols))
                .map(|e| T::try_from(e).unwrap())
                .collect::<Vec<_>>();
            let data_nd = data_storage
                .chunks_exact(cols)
                .map(<[T]>::to_vec)
                .collect::<Vec<_>>();
            let irows = isize::try_from(rows).unwrap();
            let icols = isize::try_from(cols).unwrap();
            for row_origin in -irows..=irows {
                for col_origin in -icols..=icols {
                    let mut view = data_nd.clone();
                    if row_origin >= 0 {
                        view.rotate_left(row_origin.unsigned_abs());
                    } else {
                        view.rotate_right(row_origin.unsigned_abs());
                    }
                    for row in &mut view {
                        if col_origin >= 0 {
                            row.rotate_left(col_origin.unsigned_abs());
                        } else {
                            row.rotate_right(col_origin.unsigned_abs());
                        }
                    }
                    test!(
                        data_nd r<< Offset(Row(row_origin), Col(col_origin)) == view,
                    );
                }
            }
        }
    }
}
