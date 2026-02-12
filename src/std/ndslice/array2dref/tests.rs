use rawspeed_std::coord_common::{
    ColIndex, Coord2D, RowIndex, RowLength, RowPitch,
};

use super::Array2DRef;

fn get_copy<'a, T>(input: &'a Array2DRef<'a, T>) -> Vec<Vec<Option<T>>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<Option<T>>> = vec![];
    for row in 0..*input.num_rows() {
        let mut elts: Vec<Option<T>> = vec![];
        for col in 0..*input.row_length() {
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

fn get_copy_index<'a, T>(input: &'a Array2DRef<'a, T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut rows: Vec<Vec<T>> = vec![];
    for row in 0..*input.num_rows() {
        let mut elts: Vec<T> = vec![];
        for col in 0..*input.row_length() {
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
        get_copy(&Array2DRef::new(
            &input,
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
        get_copy(&Array2DRef::new(
            &input,
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
        get_copy_index(&Array2DRef::new(
            &input,
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
        get_copy_index(&Array2DRef::new(
            &input,
            RowLength::new(2),
            RowPitch::new(3)
        )),
        vec![vec![1, 2], vec![4, 5]]
    );
}

#[test]
#[should_panic(expected = "row_length.val() > 0")]
fn no_cols_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(0), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "pitch.val() > 0")]
fn no_pitch_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(3), RowPitch::new(0));
}

#[test]
#[should_panic(expected = "pitch.val() >= row_length")]
fn unsufficient_pitch_test() {
    let mut input = vec![];
    for i in 1..=6 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(3), RowPitch::new(2));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_small_test() {
    let mut input = vec![];
    for i in 1..=5 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(3), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_small_for_single_row_test() {
    let mut input = vec![];
    for i in 1..=1 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(1), RowPitch::new(2));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_large_test() {
    let mut input = vec![];
    for i in 1..=7 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(3), RowPitch::new(3));
}

#[test]
#[should_panic(expected = "slice.len()")]
fn slice_too_large_for_single_row_test() {
    let mut input = vec![];
    for i in 1..=3 {
        input.push(i);
    }
    Array2DRef::new(&input, RowLength::new(1), RowPitch::new(2));
}

#[test]
fn elt_oob_1x1_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn elt_oob_1x2_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        Some(2)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(2)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(2)))
            .copied(),
        None
    );
}

#[test]
fn elt_oob_2x1_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        Some(2)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(2), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(2), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn padded_elt_oob_1x1_test() {
    let storage = vec![1, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn padded_elt_oob_1x2_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
            .copied(),
        Some(1)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(1)))
            .copied(),
        Some(2)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(2)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(2)))
            .copied(),
        None
    );
}

#[test]
fn padded_elt_oob_2x1_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(0), ColIndex::new(0)))
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
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(0)))
            .copied(),
        Some(2)
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(1), ColIndex::new(1)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(2), ColIndex::new(0)))
            .copied(),
        None
    );
    assert_eq!(
        input
            .get_elt(Coord2D::new(RowIndex::new(2), ColIndex::new(1)))
            .copied(),
        None
    );
}

#[test]
fn index_oob_1x1_00_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x1_01_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x1_10_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x1_11_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
fn index_oob_1x2_00_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
fn index_oob_1x2_01_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))], 2);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x2_02_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(2))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x2_10_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x2_11_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_1x2_12_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(2))];
}

#[test]
fn index_oob_2x1_00_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_2x1_01_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
fn index_oob_2x1_10_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    assert_eq!(input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))], 2);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_2x1_11_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_2x1_20_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(2), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_2x1_21_test() {
    let mut storage = vec![];
    for i in 1..=2 {
        storage.push(i);
    }
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let _ = input[Coord2D::new(RowIndex::new(2), ColIndex::new(1))];
}

//

#[test]
fn padded_index_oob_1x1_00_test() {
    let storage = vec![1, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x1_01_test() {
    let storage = vec![1, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x1_10_test() {
    let storage = vec![1, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x1_11_test() {
    let storage = vec![1, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
fn padded_index_oob_1x2_00_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
fn padded_index_oob_1x2_01_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))], 2);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x2_02_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(2))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x2_10_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x2_11_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_1x2_12_test() {
    let storage = vec![1, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(2))];
}

#[test]
fn padded_index_oob_2x1_00_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_2x1_01_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
fn padded_index_oob_2x1_10_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    assert_eq!(input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))], 2);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_2x1_11_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_2x1_20_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(2), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_2x1_21_test() {
    let storage = vec![1, 0, 2, 0];
    let input = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let _ = input[Coord2D::new(RowIndex::new(2), ColIndex::new(1))];
}
