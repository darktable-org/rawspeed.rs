use super::Array2DRefMut;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;

fn get_copy<'a, T>(input: &'a Array2DRefMut<'a, T>) -> Vec<Vec<Option<T>>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<Option<T>>> = vec![];
    for row in 0..input.num_rows() {
        let mut elts: Vec<Option<T>> = vec![];
        for col in 0..input.row_length() {
            elts.push(
                input
                    .get_elt(Coord2D::new(
                        RowIndex::new(row),
                        ColIndex::new(col),
                    ))
                    .copied(),
            );
        }
        rows.push(elts);
    }
    rows
}

fn get_copy_mut<'a, T>(
    input: &'a mut Array2DRefMut<'a, T>,
) -> Vec<Vec<Option<T>>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<Option<T>>> = vec![];
    for row in 0..input.num_rows() {
        let mut elts: Vec<Option<T>> = vec![];
        for col in 0..input.row_length() {
            elts.push(
                input
                    .get_elt_mut(Coord2D::new(
                        RowIndex::new(row),
                        ColIndex::new(col),
                    ))
                    .copied(),
            );
        }
        rows.push(elts);
    }
    rows
}

fn get_copy_index<'a, T>(input: &'a Array2DRefMut<'a, T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<T>> = vec![];
    for row in 0..input.num_rows() {
        let mut elts: Vec<T> = vec![];
        for col in 0..input.row_length() {
            elts.push(
                input[Coord2D::new(RowIndex::new(row), ColIndex::new(col))],
            );
        }
        rows.push(elts);
    }
    rows
}

fn get_copy_indexmut<'a, T>(input: &'a mut Array2DRefMut<'a, T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<T>> = vec![];
    for row in 0..input.num_rows() {
        let mut elts: Vec<T> = vec![];
        for col in 0..input.row_length() {
            elts.push(
                input[Coord2D::new(RowIndex::new(row), ColIndex::new(col))],
            );
        }
        rows.push(elts);
    }
    rows
}

#[test]
fn basic_copy_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy(&Array2DRefMut::new(
            &mut input,
            RowLength::new(3),
            RowPitch::new(3)
        )),
        vec![
            vec![Some(1), Some(2), Some(3)],
            vec![Some(4), Some(5), Some(6)]
        ]
    );
}

#[test]
fn basic_padded_copy_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy(&Array2DRefMut::new(
            &mut input,
            RowLength::new(2),
            RowPitch::new(3)
        )),
        vec![vec![Some(1), Some(2)], vec![Some(4), Some(5)]]
    );
}

#[test]
fn basic_copy_mut_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_mut(&mut Array2DRefMut::new(
            &mut input,
            RowLength::new(3),
            RowPitch::new(3)
        )),
        vec![
            vec![Some(1), Some(2), Some(3)],
            vec![Some(4), Some(5), Some(6)]
        ]
    );
}

#[test]
fn basic_padded_copy_mut_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_mut(&mut Array2DRefMut::new(
            &mut input,
            RowLength::new(2),
            RowPitch::new(3)
        )),
        vec![vec![Some(1), Some(2)], vec![Some(4), Some(5)]]
    );
}

#[test]
fn basic_copy_index_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_index(&Array2DRefMut::new(
            &mut input,
            RowLength::new(3),
            RowPitch::new(3)
        )),
        vec![vec![1, 2, 3], vec![4, 5, 6]]
    );
}

#[test]
fn basic_padded_copy_index_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_index(&Array2DRefMut::new(
            &mut input,
            RowLength::new(2),
            RowPitch::new(3)
        )),
        vec![vec![1, 2], vec![4, 5]]
    );
}

#[test]
fn basic_copy_indexmut_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_indexmut(&mut Array2DRefMut::new(
            &mut input,
            RowLength::new(3),
            RowPitch::new(3)
        )),
        vec![vec![1, 2, 3], vec![4, 5, 6]]
    );
}

#[test]
fn basic_padded_copy_indexmut_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    assert_eq!(
        get_copy_indexmut(&mut Array2DRefMut::new(
            &mut input,
            RowLength::new(2),
            RowPitch::new(3)
        )),
        vec![vec![1, 2], vec![4, 5]]
    );
}

#[test]
fn basic_mut_test() {
    let mut storage: Vec<String> = vec![];
    storage.resize(6, String::new());
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(3), RowPitch::new(3));
    for row in 0..input.num_rows() {
        for col in 0..input.row_length() {
            if let Some(dst) = input.get_elt_mut(Coord2D::new(
                RowIndex::new(row),
                ColIndex::new(col),
            )) {
                *dst = format!("row {row} col {col}").to_owned();
            }
        }
    }
    assert_eq!(
        storage,
        vec![
            "row 0 col 0",
            "row 0 col 1",
            "row 0 col 2",
            "row 1 col 0",
            "row 1 col 1",
            "row 1 col 2"
        ]
    );
}

#[test]
fn basic_padded_mut_test() {
    let mut storage: Vec<String> = vec![];
    storage.resize(8, String::new());
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(3), RowPitch::new(4));
    for row in 0..input.num_rows() {
        for col in 0..input.row_length() {
            if let Some(dst) = input.get_elt_mut(Coord2D::new(
                RowIndex::new(row),
                ColIndex::new(col),
            )) {
                *dst = format!("row {row} col {col}").to_owned();
            }
        }
    }
    assert_eq!(
        storage,
        vec![
            "row 0 col 0",
            "row 0 col 1",
            "row 0 col 2",
            "",
            "row 1 col 0",
            "row 1 col 1",
            "row 1 col 2",
            ""
        ]
    );
}

#[test]
fn basic_index_mut_test() {
    let mut storage: Vec<String> = vec![];
    storage.resize(6, String::new());
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(3), RowPitch::new(3));
    for row in 0..input.num_rows() {
        for col in 0..input.row_length() {
            input[Coord2D::new(RowIndex::new(row), ColIndex::new(col))] =
                format!("row {row} col {col}").to_owned();
        }
    }
    assert_eq!(
        storage,
        vec![
            "row 0 col 0",
            "row 0 col 1",
            "row 0 col 2",
            "row 1 col 0",
            "row 1 col 1",
            "row 1 col 2"
        ]
    );
}

#[test]
fn basic_padded_index_mut_test() {
    let mut storage: Vec<String> = vec![];
    storage.resize(8, String::new());
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(3), RowPitch::new(4));
    for row in 0..input.num_rows() {
        for col in 0..input.row_length() {
            input[Coord2D::new(RowIndex::new(row), ColIndex::new(col))] =
                format!("row {row} col {col}").to_owned();
        }
    }
    assert_eq!(
        storage,
        vec![
            "row 0 col 0",
            "row 0 col 1",
            "row 0 col 2",
            "",
            "row 1 col 0",
            "row 1 col 1",
            "row 1 col 2",
            ""
        ]
    );
}

#[test]
#[should_panic(expected = "row_length.val() > 0")]
fn no_cols_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(0), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "pitch.val() > 0")]
fn no_pitch_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(3), RowPitch::new(0));
}

#[test]
#[should_panic(expected = "pitch.val() >= row_length")]
fn unsufficient_pitch_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(3), RowPitch::new(2));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_small_test() {
    let mut input = vec![];
    for i in 1..=5 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(3), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_small_for_single_row_test() {
    let mut input = vec![];
    for i in 1..=1 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(1), RowPitch::new(2));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_large_test() {
    let mut input = vec![];
    for i in 1..=7 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(3), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_large_for_single_row_test() {
    let mut input = vec![];
    for i in 1..=3 {
        input.push(i);
    }
    Array2DRefMut::new(&mut input, RowLength::new(1), RowPitch::new(2));
}

#[test]
fn elt_oob_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn padded_elt_oob_test() {
    let mut storage = vec![1, 0];
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt_mut(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn index_oob_00_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
fn indexmut_oob_00_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(
        (&mut input)[Coord2D::new(RowIndex::new(0), ColIndex::new(0))],
        1
    );
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_01_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn indexmut_oob_01_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_10_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn indexmut_oob_10_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_11_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn indexmut_oob_11_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(1));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
fn padded_index_oob_00_test() {
    let mut storage = vec![1, 0];
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
fn padded_indexmut_oob_00_test() {
    let mut storage = vec![1, 0];
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(
        (&mut input)[Coord2D::new(RowIndex::new(0), ColIndex::new(0))],
        1
    );
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_01_test() {
    let mut storage = vec![1, 0];
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_indexmut_oob_01_test() {
    let mut storage = vec![1, 0];
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_10_test() {
    let mut storage = vec![1, 0];
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_indexmut_oob_10_test() {
    let mut storage = vec![1, 0];
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_11_test() {
    let mut storage = vec![1, 0];
    let input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_indexmut_oob_11_test() {
    let mut storage = vec![1, 0];
    let mut input =
        Array2DRefMut::new(&mut storage, RowLength::new(1), RowPitch::new(2));
    let _ = (&mut input)[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}
