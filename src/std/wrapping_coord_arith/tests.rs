use super::*;
use crate::coord_common::RowCount;

macro_rules! test {
    ($((($lhs:expr) + ($rhs:expr)) (mod ($bound:expr)) == ($res:expr),)+) => {
        $(
            {
                let bound = RowCount::new($bound);
                let lhs = BoundRowIndex::new(bound, RowIndex::new($lhs)).unwrap();
                let rhs = RowOffset::new($rhs);
                let res = WrappingRowIndex::new(lhs) + rhs;
                let expected = WrappingRowIndex::new(BoundRowIndex::new(bound, RowIndex::new($res)).unwrap());
                assert_eq!(res, expected);
            }
        )+
    };
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn test() {
    test!(
        //
        ((0) + (0)) (mod (1)) == (0),
        ((0) + (1)) (mod (1)) == (0),
        ((0) + (2)) (mod (1)) == (0),
        ((0) + (-1)) (mod (1)) == (0),
        ((0) + (-2)) (mod (1)) == (0),
        //
        ((0) + (0)) (mod (2)) == (0),
        ((0) + (1)) (mod (2)) == (1),
        ((0) + (2)) (mod (2)) == (0),
        ((0) + (3)) (mod (2)) == (1),
        ((0) + (-1)) (mod (2)) == (1),
        ((0) + (-2)) (mod (2)) == (0),
        ((0) + (-3)) (mod (2)) == (1),
        //
        ((1) + (0)) (mod (2)) == (1),
        ((1) + (1)) (mod (2)) == (0),
        ((1) + (2)) (mod (2)) == (1),
        ((1) + (3)) (mod (2)) == (0),
        ((1) + (-1)) (mod (2)) == (0),
        ((1) + (-2)) (mod (2)) == (1),
        ((1) + (-3)) (mod (2)) == (0),
        //
        ((0) + (0)) (mod (3)) == (0),
        ((0) + (1)) (mod (3)) == (1),
        ((0) + (2)) (mod (3)) == (2),
        ((0) + (3)) (mod (3)) == (0),
        ((0) + (4)) (mod (3)) == (1),
        ((0) + (5)) (mod (3)) == (2),
        ((0) + (-1)) (mod (3)) == (2),
        ((0) + (-2)) (mod (3)) == (1),
        ((0) + (-3)) (mod (3)) == (0),
        ((0) + (-4)) (mod (3)) == (2),
        ((0) + (-5)) (mod (3)) == (1),
        //
        ((1) + (0)) (mod (3)) == (1),
        ((1) + (1)) (mod (3)) == (2),
        ((1) + (2)) (mod (3)) == (0),
        ((1) + (3)) (mod (3)) == (1),
        ((1) + (4)) (mod (3)) == (2),
        ((1) + (5)) (mod (3)) == (0),
        ((1) + (-1)) (mod (3)) == (0),
        ((1) + (-2)) (mod (3)) == (2),
        ((1) + (-3)) (mod (3)) == (1),
        ((1) + (-4)) (mod (3)) == (0),
        ((1) + (-5)) (mod (3)) == (2),
        //
        ((2) + (0)) (mod (3)) == (2),
        ((2) + (1)) (mod (3)) == (0),
        ((2) + (2)) (mod (3)) == (1),
        ((2) + (3)) (mod (3)) == (2),
        ((2) + (4)) (mod (3)) == (0),
        ((2) + (5)) (mod (3)) == (1),
        ((2) + (-1)) (mod (3)) == (1),
        ((2) + (-2)) (mod (3)) == (0),
        ((2) + (-3)) (mod (3)) == (2),
        ((2) + (-4)) (mod (3)) == (1),
        ((2) + (-5)) (mod (3)) == (0),
        //-
        ((0) + (-1)) (mod (isize::MAX as usize )) == ((isize::MAX as usize)-1),
        ((0) + (isize::MIN+1)) (mod (isize::MAX as usize )) == (0),
        ((0) + (isize::MIN )) (mod (isize::MAX as usize )) == ((isize::MAX as usize)-1),
        //-
        ((0) + (-1)) (mod ((isize::MAX as usize)+1)) == (isize::MAX as usize ),
        ((0) + (isize::MIN+1)) (mod ((isize::MAX as usize)+1)) == (1),
        ((0) + (isize::MIN)) (mod ((isize::MAX as usize)+1)) == (0),
        //-
        ((0) + (-1)) (mod ((isize::MAX as usize)+2)) == ((isize::MAX as usize)+1),
        ((0) + (isize::MIN+1)) (mod ((isize::MAX as usize)+2)) == (2),
        ((0) + (isize::MIN)) (mod ((isize::MAX as usize)+2)) == (1),
        //
        //
    );
}
