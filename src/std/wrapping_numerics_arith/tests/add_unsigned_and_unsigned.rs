use crate::{
    bound_numerics::{Bound, BoundUnsigned},
    wrapping_numerics::WrappingUnsigned,
};

type T = u8;

fn naive_wrapping_add(lhs: BoundUnsigned<T>, rhs: T) -> WrappingUnsigned<T> {
    let bound = *lhs.domain();
    let res = (u16::checked_add((*lhs).into(), rhs.into()).unwrap())
        .checked_rem((*bound).into())
        .unwrap()
        .try_into()
        .unwrap();
    WrappingUnsigned::new(BoundUnsigned::new(bound, res).unwrap())
}

fn run_test(lhs: BoundUnsigned<T>, rhs: T) {
    let expected = naive_wrapping_add(lhs, rhs);
    let lhs = WrappingUnsigned::new(lhs);
    let res = lhs + rhs;
    assert_eq!(res, expected);
}

#[test]
#[cfg(not(miri))]
fn add_test_exhaustive() {
    for bound in u8::MIN..=u8::MAX {
        let Some(bound) = Bound::new(bound) else {
            continue;
        };
        for lhs in u8::MIN..=u8::MAX {
            let Some(lhs) = BoundUnsigned::new(bound, lhs) else {
                continue;
            };
            for rhs in u8::MIN..=u8::MAX {
                run_test(lhs, rhs);
            }
        }
    }
}

macro_rules! test {
    ($((($lhs:expr) + ($rhs:expr)) (mod ($bound:expr)) == ($res:expr),)+) => {
        $(
            {
                let bound = Bound::new($bound).unwrap();
                let lhs = BoundUnsigned::new(bound, $lhs).unwrap();
                assert_eq!(
                    WrappingUnsigned::new(BoundUnsigned::new(bound, $res).unwrap()),
                    naive_wrapping_add(lhs, $rhs),
                    "Self-check in wider types"
                );
                run_test(lhs, $rhs);
            }
        )+
    };
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn add_test() {
    test!(
        //
        ((0) + (0)) (mod (1)) == (0),
        ((0) + (1)) (mod (1)) == (0),
        ((0) + (2)) (mod (1)) == (0),
        //
        ((0) + (0)) (mod (2)) == (0),
        ((0) + (1)) (mod (2)) == (1),
        ((0) + (2)) (mod (2)) == (0),
        ((0) + (3)) (mod (2)) == (1),
        //
        ((1) + (0)) (mod (2)) == (1),
        ((1) + (1)) (mod (2)) == (0),
        ((1) + (2)) (mod (2)) == (1),
        ((1) + (3)) (mod (2)) == (0),
        //
        ((0) + (0)) (mod (3)) == (0),
        ((0) + (1)) (mod (3)) == (1),
        ((0) + (2)) (mod (3)) == (2),
        ((0) + (3)) (mod (3)) == (0),
        ((0) + (4)) (mod (3)) == (1),
        ((0) + (5)) (mod (3)) == (2),
        //
        ((1) + (0)) (mod (3)) == (1),
        ((1) + (1)) (mod (3)) == (2),
        ((1) + (2)) (mod (3)) == (0),
        ((1) + (3)) (mod (3)) == (1),
        ((1) + (4)) (mod (3)) == (2),
        ((1) + (5)) (mod (3)) == (0),
        //
        ((2) + (0)) (mod (3)) == (2),
        ((2) + (1)) (mod (3)) == (0),
        ((2) + (2)) (mod (3)) == (1),
        ((2) + (3)) (mod (3)) == (2),
        ((2) + (4)) (mod (3)) == (0),
        ((2) + (5)) (mod (3)) == (1),
        //--------------------------------------
        ((0) + (u8::MAX-3)) (mod (1)) == (0),
        ((0) + (u8::MAX-2)) (mod (1)) == (0),
        ((0) + (u8::MAX-1)) (mod (1)) == (0),
        ((0) + (u8::MAX)) (mod (1)) == (0),
        //
        ((0) + (u8::MAX-3)) (mod (2)) == (0),
        ((0) + (u8::MAX-2)) (mod (2)) == (1),
        ((0) + (u8::MAX-1)) (mod (2)) == (0),
        ((0) + (u8::MAX)) (mod (2)) == (1),
        //
        ((1) + (u8::MAX-3)) (mod (2)) == (1),
        ((1) + (u8::MAX-2)) (mod (2)) == (0),
        ((1) + (u8::MAX-1)) (mod (2)) == (1),
        ((1) + (u8::MAX)) (mod (2)) == (0),
        //
        ((0) + (u8::MAX-5)) (mod (3)) == (1),
        ((0) + (u8::MAX-4)) (mod (3)) == (2),
        ((0) + (u8::MAX-3)) (mod (3)) == (0),
        ((0) + (u8::MAX-2)) (mod (3)) == (1),
        ((0) + (u8::MAX-1)) (mod (3)) == (2),
        ((0) + (u8::MAX)) (mod (3)) == (0),
        //
        ((1) + (u8::MAX-5)) (mod (3)) == (2),
        ((1) + (u8::MAX-4)) (mod (3)) == (0),
        ((1) + (u8::MAX-3)) (mod (3)) == (1),
        ((1) + (u8::MAX-2)) (mod (3)) == (2),
        ((1) + (u8::MAX-1)) (mod (3)) == (0),
        ((1) + (u8::MAX)) (mod (3)) == (1), // <- needs rhs mod dom
        //
        ((2) + (u8::MAX-5)) (mod (3)) == (0),
        ((2) + (u8::MAX-4)) (mod (3)) == (1),
        ((2) + (u8::MAX-3)) (mod (3)) == (2),
        ((2) + (u8::MAX-2)) (mod (3)) == (0),
        ((2) + (u8::MAX-1)) (mod (3)) == (1), // <- needs rhs mod dom
        ((2) + (u8::MAX)) (mod (3)) == (2), // <- needs rhs mod dom
        //
        ((128) + (128)) (mod (129)) == (127),
    );
}
