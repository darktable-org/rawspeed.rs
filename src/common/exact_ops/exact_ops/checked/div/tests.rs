use super::*;

macro_rules! test_u8 {
    ($(($lhs:expr) e/ ($rhs:expr) == $res:expr,)+) => {
        $(
            assert_eq!(
                <u8 as CheckedDivExact>::checked_div_exact($lhs, $rhs),
                $res.into()
            );
        )+
    };
}

macro_rules! test_i8 {
    ($(($lhs:expr) e/ ($rhs:expr) == $res:expr,)+) => {
        $(
            assert_eq!(
                <i8 as CheckedDivExact>::checked_div_exact($lhs, $rhs),
                $res.into()
            );
        )+
    };
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn u8_test() {
    test_u8!(
        (0) e/ (0) == None,
        (0) e/ (1) == 0,
        (0) e/ (2) == 0,
        (0) e/ (3) == 0,
        (0) e/ (4) == 0,
        (0) e/ (5) == 0,
        (0) e/ (6) == 0,
        (1) e/ (0) == None,
        (1) e/ (1) == 1,
        (1) e/ (2) == None,
        (1) e/ (3) == None,
        (1) e/ (4) == None,
        (1) e/ (5) == None,
        (1) e/ (6) == None,
        (2) e/ (0) == None,
        (2) e/ (1) == 2,
        (2) e/ (2) == 1,
        (2) e/ (3) == None,
        (2) e/ (4) == None,
        (2) e/ (5) == None,
        (2) e/ (6) == None,
        (3) e/ (0) == None,
        (3) e/ (1) == 3,
        (3) e/ (2) == None,
        (3) e/ (3) == 1,
    );
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn i8_test() {
    test_i8!(
        (-4) e/ (-3) == None,
        (-4) e/ (-2) == 2,
        (-4) e/ (-1) == 4,
        (-4) e/ (0) == None,
        (-4) e/ (1) == -4,
        (-4) e/ (2) == -2,
        (-4) e/ (3) == None,
        (-3) e/ (-3) == 1,
        (-3) e/ (-2) == None,
        (-3) e/ (-1) == 3,
        (-3) e/ (0) == None,
        (-3) e/ (1) == -3,
        (-3) e/ (2) == None,
        (-3) e/ (3) == -1,
        (-2) e/ (-3) == None,
        (-2) e/ (-2) == 1,
        (-2) e/ (-1) == 2,
        (-2) e/ (0) == None,
        (-2) e/ (1) == -2,
        (-2) e/ (2) == -1,
        (-2) e/ (3) == None,
        (-1) e/ (-3) == None,
        (-1) e/ (-2) == None,
        (-1) e/ (-1) == 1,
        (-1) e/ (0) == None,
        (-1) e/ (1) == -1,
        (-1) e/ (2) == None,
        (-1) e/ (3) == None,
        (0) e/ (-3) == 0,
        (0) e/ (-2) == 0,
        (0) e/ (-1) == 0,
        (0) e/ (0) == None,
        (0) e/ (1) == 0,
        (0) e/ (2) == 0,
        (0) e/ (3) == 0,
        (1) e/ (-3) == None,
        (1) e/ (-2) == None,
        (1) e/ (-1) == -1,
        (1) e/ (0) == None,
        (1) e/ (1) == 1,
        (1) e/ (2) == None,
        (1) e/ (3) == None,
        (2) e/ (-3) == None,
        (2) e/ (-2) == -1,
        (2) e/ (-1) == -2,
        (2) e/ (0) == None,
        (2) e/ (1) == 2,
        (2) e/ (2) == 1,
        (2) e/ (3) == None,
        (3) e/ (-3) == -1,
        (3) e/ (-2) == None,
        (3) e/ (-1) == -3,
        (3) e/ (0) == None,
        (3) e/ (1) == 3,
        (3) e/ (2) == None,
        (3) e/ (3) == 1,
        //
        (-128) e/ (-2) == 64,
        (-128) e/ (-1) == None,
    );
}
