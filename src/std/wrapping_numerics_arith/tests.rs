use crate::{
    bound_numerics::{Bound, BoundUnsigned},
    wrapping_numerics::WrappingUnsigned,
};

struct Test {
    bound: u8,
    lhs: u8,
    rhs: u8,
    res: u8,
}

macro_rules! test_entry {
    ((($lhs:expr) + ($rhs:expr)) mod ($bound:expr) == ($res:expr)) => {
        Test {
            bound: $bound,
            lhs: $lhs,
            rhs: $rhs,
            res: $res,
        }
    };
}

#[test]
fn add_test() {
    let tests = [
        //
        test_entry!(((0) + (0)) mod (1) == (0)),
        test_entry!(((0) + (1)) mod (1) == (0)),
        test_entry!(((0) + (2)) mod (1) == (0)),
        //
        test_entry!(((0) + (0)) mod (2) == (0)),
        test_entry!(((0) + (1)) mod (2) == (1)),
        test_entry!(((0) + (2)) mod (2) == (0)),
        test_entry!(((0) + (3)) mod (2) == (1)),
        //
        test_entry!(((1) + (0)) mod (2) == (1)),
        test_entry!(((1) + (1)) mod (2) == (0)),
        test_entry!(((1) + (2)) mod (2) == (1)),
        test_entry!(((1) + (3)) mod (2) == (0)),
        //
        test_entry!(((0) + (0)) mod (3) == (0)),
        test_entry!(((0) + (1)) mod (3) == (1)),
        test_entry!(((0) + (2)) mod (3) == (2)),
        test_entry!(((0) + (3)) mod (3) == (0)),
        test_entry!(((0) + (4)) mod (3) == (1)),
        test_entry!(((0) + (5)) mod (3) == (2)),
        //
        test_entry!(((1) + (0)) mod (3) == (1)),
        test_entry!(((1) + (1)) mod (3) == (2)),
        test_entry!(((1) + (2)) mod (3) == (0)),
        test_entry!(((1) + (3)) mod (3) == (1)),
        test_entry!(((1) + (4)) mod (3) == (2)),
        test_entry!(((1) + (5)) mod (3) == (0)),
        //
        test_entry!(((2) + (0)) mod (3) == (2)),
        test_entry!(((2) + (1)) mod (3) == (0)),
        test_entry!(((2) + (2)) mod (3) == (1)),
        test_entry!(((2) + (3)) mod (3) == (2)),
        test_entry!(((2) + (4)) mod (3) == (0)),
        test_entry!(((2) + (5)) mod (3) == (1)),
        //--------------------------------------
        test_entry!(((0) + (u8::MAX-3)) mod (1) == (0)),
        test_entry!(((0) + (u8::MAX-2)) mod (1) == (0)),
        test_entry!(((0) + (u8::MAX-1)) mod (1) == (0)),
        test_entry!(((0) + (u8::MAX)) mod (1) == (0)),
        //
        test_entry!(((0) + (u8::MAX-3)) mod (2) == (0)),
        test_entry!(((0) + (u8::MAX-2)) mod (2) == (1)),
        test_entry!(((0) + (u8::MAX-1)) mod (2) == (0)),
        test_entry!(((0) + (u8::MAX)) mod (2) == (1)),
        //
        test_entry!(((1) + (u8::MAX-3)) mod (2) == (1)),
        test_entry!(((1) + (u8::MAX-2)) mod (2) == (0)),
        test_entry!(((1) + (u8::MAX-1)) mod (2) == (1)),
        test_entry!(((1) + (u8::MAX)) mod (2) == (0)),
        //
        test_entry!(((0) + (u8::MAX-5)) mod (3) == (1)),
        test_entry!(((0) + (u8::MAX-4)) mod (3) == (2)),
        test_entry!(((0) + (u8::MAX-3)) mod (3) == (0)),
        test_entry!(((0) + (u8::MAX-2)) mod (3) == (1)),
        test_entry!(((0) + (u8::MAX-1)) mod (3) == (2)),
        test_entry!(((0) + (u8::MAX)) mod (3) == (0)),
        //
        test_entry!(((1) + (u8::MAX-5)) mod (3) == (2)),
        test_entry!(((1) + (u8::MAX-4)) mod (3) == (0)),
        test_entry!(((1) + (u8::MAX-3)) mod (3) == (1)),
        test_entry!(((1) + (u8::MAX-2)) mod (3) == (2)),
        test_entry!(((1) + (u8::MAX-1)) mod (3) == (0)),
        test_entry!(((1) + (u8::MAX)) mod (3) == (1)), // <- needs rhs mod dom
        //
        test_entry!(((2) + (u8::MAX-5)) mod (3) == (0)),
        test_entry!(((2) + (u8::MAX-4)) mod (3) == (1)),
        test_entry!(((2) + (u8::MAX-3)) mod (3) == (2)),
        test_entry!(((2) + (u8::MAX-2)) mod (3) == (0)),
        test_entry!(((2) + (u8::MAX-1)) mod (3) == (1)), // <- needs rhs mod dom
        test_entry!(((2) + (u8::MAX)) mod (3) == (2)),   // <- needs rhs mod dom
    ];
    for test in tests {
        assert_eq!(
            u16::from(test.res),
            (u16::checked_add(test.lhs.into(), test.rhs.into()).unwrap())
                .checked_rem(test.bound.into())
                .unwrap(),
            "Self-check in wider types"
        );

        let bound = Bound::<u8>::new(test.bound);
        let lhs =
            WrappingUnsigned::new(BoundUnsigned::new(bound, test.lhs).unwrap());
        let res = lhs + test.rhs;
        let expected =
            WrappingUnsigned::new(BoundUnsigned::new(bound, test.res).unwrap());
        assert_eq!(res, expected);
    }
}
