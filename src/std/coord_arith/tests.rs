use crate::coord_common::{
    ColIndex, ColOffset, Coord2D, CoordOffset2D, RowIndex, RowOffset,
};

#[inline]
fn enumerate_neg_tests<T>(inner: T)
where
    T: Fn(isize, Option<isize>),
{
    const TESTS: [(isize, Option<isize>); 8] = [
        ((0), Some(0)),
        ((1), Some(-1)),
        ((-1), Some(1)),
        ((2), Some(-2)),
        ((-2), Some(2)),
        ((isize::MAX), Some(isize::MIN + 1)),
        ((isize::MIN + 1), Some(isize::MAX)),
        ((isize::MIN), None),
    ];
    for (a, b) in TESTS {
        inner(a, b);
    }
}

#[test]
fn rowoffset_neg_test() {
    type T = RowOffset;
    enumerate_neg_tests(|a, b| assert_eq!(-T::new(a), b.map(T::new)));
}

#[test]
fn coloffset_neg_test() {
    type T = ColOffset;
    enumerate_neg_tests(|a, b| assert_eq!(-T::new(a), b.map(T::new)));
}

#[test]
fn coordoffset2d_neg_test() {
    enumerate_neg_tests(|a, c| {
        enumerate_neg_tests(|b, d| {
            let res = match (c, d) {
                (Some(c), Some(d)) => Some(CoordOffset2D::new(
                    RowOffset::new(c),
                    ColOffset::new(d),
                )),
                _ => None,
            };
            assert_eq!(
                -CoordOffset2D::new(RowOffset::new(a), ColOffset::new(b)),
                res,
            );
        });
    });
}

#[inline]
fn enumerate_dist_tests<T>(inner: T)
where
    T: Fn((usize, usize), Option<isize>),
{
    let thr: usize = isize::MAX.try_into().unwrap();
    let tests = [
        ((0, 0), Some(0)),
        ((0, 1), Some(-1)),
        ((0, 2), Some(-2)),
        ((0, thr - 2), Some(isize::MIN + 3)),
        ((0, thr - 1), Some(isize::MIN + 2)),
        ((0, thr), Some(isize::MIN + 1)),
        ((0, thr + 1), Some(isize::MIN)),
        ((0, thr + 2), None),
        //
        ((1, 0), Some(1)),
        ((1, 1), Some(0)),
        ((1, 2), Some(-1)),
        ((1, thr - 2), Some(isize::MIN + 4)),
        ((1, thr - 1), Some(isize::MIN + 3)),
        ((1, thr), Some(isize::MIN + 2)),
        ((1, thr + 1), Some(isize::MIN + 1)),
        ((1, thr + 2), Some(isize::MIN)),
        ((1, thr + 3), None),
        //
        ((0, 0), Some(0)),
        ((1, 0), Some(1)),
        ((2, 0), Some(2)),
        ((thr - 2, 0), Some(isize::MAX - 2)),
        ((thr - 1, 0), Some(isize::MAX - 1)),
        ((thr, 0), Some(isize::MAX)),
        ((thr + 1, 0), None),
        //
        ((0, 1), Some(-1)),
        ((1, 1), Some(0)),
        ((2, 1), Some(1)),
        ((thr - 2, 1), Some(isize::MAX - 3)),
        ((thr - 1, 1), Some(isize::MAX - 2)),
        ((thr, 1), Some(isize::MAX - 1)),
        ((thr + 1, 1), None),
        //
        ((thr - 1, 0), Some(isize::MAX - 1)),
        ((thr - 1, 1), Some(isize::MAX - 2)),
        ((thr - 1, 2), Some(isize::MAX - 3)),
        ((thr - 1, thr - 3), Some(2)),
        ((thr - 1, thr - 2), Some(1)),
        ((thr - 1, thr - 1), Some(0)),
        ((thr - 1, thr), Some(-1)),
        ((thr - 1, thr + 1), Some(-2)),
        //
        ((thr, 0), Some(isize::MAX)),
        ((thr, 1), Some(isize::MAX - 1)),
        ((thr, 2), Some(isize::MAX - 2)),
        ((thr, thr - 3), Some(3)),
        ((thr, thr - 2), Some(2)),
        ((thr, thr - 1), Some(1)),
        ((thr, thr), Some(0)),
        ((thr, thr + 1), Some(-1)),
        //
        ((thr + 1, 0), None),
        ((thr + 1, 1), None),
        ((thr + 1, 2), None),
        ((thr + 1, thr - 3), None),
        ((thr + 1, thr - 2), None),
        ((thr + 1, thr - 1), None),
        ((thr + 1, thr), None),
        ((thr + 1, thr + 1), None),
    ];
    for ((a, b), c) in tests {
        inner((a, b), c);
    }
}

#[test]
fn rowindex_dist_test() {
    enumerate_dist_tests(|(a, b), c| {
        assert_eq!(RowIndex::new(a) - RowIndex::new(b), c.map(RowOffset::new));
    });
}

#[test]
fn colindex_dist_test() {
    enumerate_dist_tests(|(a, b), c| {
        assert_eq!(ColIndex::new(a) - ColIndex::new(b), c.map(ColOffset::new));
    });
}

#[test]
fn coord2d_dist_test() {
    enumerate_dist_tests(|a, c| {
        enumerate_dist_tests(|b, d| {
            let res = match (c, d) {
                (Some(c), Some(d)) => Some(CoordOffset2D::new(
                    RowOffset::new(c),
                    ColOffset::new(d),
                )),
                _ => None,
            };
            assert_eq!(
                Coord2D::new(RowIndex::new(a.0), ColIndex::new(b.0))
                    - Coord2D::new(RowIndex::new(a.1), ColIndex::new(b.1)),
                res,
            );
        });
    });
}

#[inline]
fn enumerate_offset_tests<T>(inner: T)
where
    T: Fn((usize, isize), Option<usize>),
{
    let thr: usize = isize::MAX.try_into().unwrap();
    let tests = [
        //
        ((0, isize::MIN), Some(thr + 1)),
        ((0, isize::MIN + 1), Some(thr)),
        ((0, (isize::MIN + 2)), Some(thr - 1)),
        ((0, -2_isize), Some(2)),
        ((0, -1), Some(1)),
        ((0, 0), Some(0_usize)),
        ((0, 1), None),
        ((0, 2), None),
        //
        ((1, isize::MIN), Some(thr + 2)),
        ((1, isize::MIN + 1), Some(thr + 1)),
        ((1, (isize::MIN + 2)), Some(thr)),
        ((1, (isize::MIN + 3)), Some(thr - 1)),
        ((1, -2), Some(3)),
        ((1, -1), Some(2)),
        ((1, 0), Some(1)),
        ((1, 1), Some(0)),
        ((1, 2), None),
        ((1, 3), None),
        //
        ((thr - 1, isize::MIN), Some(2 * thr)),
        ((thr - 1, isize::MIN + 1), Some(2 * thr - 1)),
        ((thr - 1, (isize::MIN + 2)), Some(2 * thr - 2)),
        ((thr - 1, (isize::MIN + 3)), Some(2 * thr - 3)),
        ((thr - 1, -2), Some(thr + 1)),
        ((thr - 1, -1), Some(thr)),
        ((thr - 1, 0), Some(thr - 1)),
        ((thr - 1, 1), Some(thr - 2)),
        ((thr - 1, 2), Some(thr - 3)),
        ((thr - 1, isize::MAX - 3), Some(2)),
        ((thr - 1, isize::MAX - 2), Some(1)),
        ((thr - 1, isize::MAX - 1), Some(0)),
        ((thr - 1, isize::MAX), None),
        //
        ((thr, isize::MIN), Some(2 * thr + 1)),
        ((thr, isize::MIN + 1), Some(2 * thr)),
        ((thr, (isize::MIN + 2)), Some(2 * thr - 1)),
        ((thr, (isize::MIN + 3)), Some(2 * thr - 2)),
        ((thr, -2), Some(thr + 2)),
        ((thr, -1), Some(thr + 1)),
        ((thr, 0), Some(thr)),
        ((thr, 1), Some(thr - 1)),
        ((thr, 2), Some(thr - 2)),
        ((thr, isize::MAX - 3), Some(3)),
        ((thr, isize::MAX - 2), Some(2)),
        ((thr, isize::MAX - 1), Some(1)),
        ((thr, isize::MAX), Some(0)),
        //
        ((thr + 1, isize::MIN), None),
        ((thr + 1, isize::MIN + 1), Some(2 * thr + 1)),
        ((thr + 1, (isize::MIN + 2)), Some(2 * thr)),
        ((thr + 1, (isize::MIN + 3)), Some(2 * thr - 1)),
        ((thr + 1, -2), Some(thr + 3)),
        ((thr + 1, -1), Some(thr + 2)),
        ((thr + 1, 0), Some(thr + 1)),
        ((thr + 1, 1), Some(thr)),
        ((thr + 1, 2), Some(thr - 1)),
        ((thr + 1, isize::MAX - 3), Some(4)),
        ((thr + 1, isize::MAX - 2), Some(3)),
        ((thr + 1, isize::MAX - 1), Some(2)),
        ((thr + 1, isize::MAX), Some(1)),
    ];
    for ((a, b), c) in tests {
        inner((a, b), c);
    }
}

#[test]
fn rowindex_offset_test() {
    enumerate_offset_tests(|(a, b), c| {
        assert_eq!(
            RowIndex::new(a) - RowOffset::new(b),
            c.map(RowIndex::new),
            "{a} - {b}",
        );
    });
}

#[test]
fn colindex_offset_test() {
    enumerate_offset_tests(|(a, b), c| {
        assert_eq!(ColIndex::new(a) - ColOffset::new(b), c.map(ColIndex::new));
    });
}

#[test]
fn coord2d_offset_test() {
    enumerate_offset_tests(|a, c| {
        enumerate_offset_tests(|b, d| {
            let res = match (c, d) {
                (Some(c), Some(d)) => {
                    Some(Coord2D::new(RowIndex::new(c), ColIndex::new(d)))
                }
                _ => None,
            };
            assert_eq!(
                Coord2D::new(RowIndex::new(a.0), ColIndex::new(b.0))
                    - CoordOffset2D::new(
                        RowOffset::new(a.1),
                        ColOffset::new(b.1)
                    ),
                res,
            );
        });
    });
}
