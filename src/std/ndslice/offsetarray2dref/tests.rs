use crate::offsetarray2dref::OffsetArray2DRef;

use super::Array2DRef;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::ColOffset;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::CoordOffset2D;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowOffset;
use rawspeed_std::coord_common::RowPitch;

fn get_copy_index<'a, T>(input: &'a OffsetArray2DRef<'a, T>) -> Vec<Vec<T>>
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
fn basic_copy_2x2_test() {
    let mut input = vec![];
    for i in 1..=4 {
        input.push(i);
    }
    let base = Array2DRef::new(&input, RowLength::new(2), RowPitch::new(2));
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0))
        )),
        vec![vec![1, 2], vec![3, 4]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(1), ColOffset::new(0))
        )),
        vec![vec![3, 4], vec![1, 2]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(0), ColOffset::new(1))
        )),
        vec![vec![2, 1], vec![4, 3]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(1), ColOffset::new(1))
        )),
        vec![vec![4, 3], vec![2, 1]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(-1), ColOffset::new(0))
        )),
        vec![vec![3, 4], vec![1, 2]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(-1), ColOffset::new(-1))
        )),
        vec![vec![4, 3], vec![2, 1]]
    );
    assert_eq!(
        get_copy_index(&OffsetArray2DRef::new(
            base,
            CoordOffset2D::new(RowOffset::new(0), ColOffset::new(-1))
        )),
        vec![vec![2, 1], vec![4, 3]]
    );
}

#[test]
fn elt_oob_1x1_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
    let base = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
    let base = Array2DRef::new(&storage, RowLength::new(2), RowPitch::new(3));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
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
fn index_oob_00_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_01_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_10_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn index_oob_11_test() {
    let mut storage = vec![];
    for i in 1..=1 {
        storage.push(i);
    }
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(1));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}

#[test]
fn padded_index_oob_00_test() {
    let storage = vec![1, 0];
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    assert_eq!(input[Coord2D::new(RowIndex::new(0), ColIndex::new(0))], 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_01_test() {
    let storage = vec![1, 0];
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(0), ColIndex::new(1))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_10_test() {
    let storage = vec![1, 0];
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(0))];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn padded_index_oob_11_test() {
    let storage = vec![1, 0];
    let base = Array2DRef::new(&storage, RowLength::new(1), RowPitch::new(2));
    let input = OffsetArray2DRef::new(
        base,
        CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0)),
    );
    let _ = input[Coord2D::new(RowIndex::new(1), ColIndex::new(1))];
}
