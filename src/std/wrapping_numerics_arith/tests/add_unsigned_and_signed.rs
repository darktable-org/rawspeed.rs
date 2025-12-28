use crate::{
    bound_numerics::{Bound, BoundUnsigned},
    wrapping_numerics::WrappingUnsigned,
};

type T = u8;
type U = i8;

fn naive_wrapping_add_signed(
    lhs: BoundUnsigned<T>,
    rhs: U,
) -> WrappingUnsigned<T> {
    let bound = *lhs.domain();
    let rhs: T = {
        let mut rhs: i16 = rhs.into();
        while rhs < 0 {
            rhs = rhs.checked_add_unsigned((*bound).into()).unwrap();
        }
        rhs.try_into().unwrap()
    };
    let res = u16::checked_add((*lhs).into(), rhs.into())
        .unwrap()
        .checked_rem((*bound).into())
        .unwrap()
        .try_into()
        .unwrap();
    WrappingUnsigned::new(BoundUnsigned::new(bound, res).unwrap())
}

fn run_test(lhs: BoundUnsigned<T>, rhs: U) {
    let expected = naive_wrapping_add_signed(lhs, rhs);
    let lhs = WrappingUnsigned::new(lhs);
    let res = lhs + rhs;
    assert_eq!(res, expected);
}

#[test]
#[cfg(not(miri))]
fn exhaustive_test() {
    for bound in T::MIN..=T::MAX {
        let Some(bound) = Bound::new(bound) else {
            continue;
        };
        for lhs in T::MIN..=T::MAX {
            let Some(lhs) = BoundUnsigned::new(bound, lhs) else {
                continue;
            };
            for rhs in U::MIN..=U::MAX {
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
                    naive_wrapping_add_signed(lhs, $rhs),
                    "Self-check in wider types"
                );
                run_test(lhs, $rhs);
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
        ((0) + (-1)) (mod (127)) == (126),
        ((0) + (-127)) (mod (127)) == (0),
        ((0) + (-128)) (mod (127)) == (126),
        //-
        ((0) + (-1)) (mod (128)) == (127),
        ((0) + (-127)) (mod (128)) == (1),
        ((0) + (-128)) (mod (128)) == (0),
        //-
        ((0) + (-1)) (mod (129)) == (128),
        ((0) + (-127)) (mod (129)) == (2),
        ((0) + (-128)) (mod (129)) == (1),
        //
        //
    );
}
