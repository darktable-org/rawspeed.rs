use crate::range_rotation::rotated_range::RotatableRange;

#[test]
fn exhaustive_test() {
    let max_range = 0_u8..=6;
    for lb in max_range.clone() {
        for ub in max_range.clone() {
            let cur_range = lb..ub;
            let ulen = cur_range.len();
            let ilen: isize = ulen.try_into().unwrap();
            for shamt in -3 * ilen..=3 * ilen {
                let correct = {
                    let mut v = cur_range.clone().collect::<Vec<_>>();
                    if ulen != 0 {
                        if shamt >= 0 {
                            v.rotate_left(shamt.unsigned_abs() % ulen);
                        } else {
                            v.rotate_right(shamt.unsigned_abs() % ulen);
                        }
                    }
                    v
                };
                let actual = RotatableRange::rotate(cur_range.clone(), shamt)
                    .into_iter()
                    .collect::<Vec<_>>();
                assert_eq!(actual, correct);
            }
        }
    }
}

macro_rules! test {
    ($($lb:literal..$ub:literal r<< $mid:literal == $res:tt,)+) => {
        $(
            {
                let actual = RotatableRange::rotate($lb..$ub, $mid)
                    .into_iter().collect::<Vec<_>>();
                let expected_output = $res;
                assert_eq!(actual, expected_output);
            }
        )+
    };
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn rotate_test() {
    test!(
        //
        0..0 r<< 0 == [],
        0..0 r<< 1 == [],
        0..0 r<< 2 == [],
        //
        0..1 r<< 0 == [0],
        0..1 r<< 1 == [0],
        0..1 r<< 2 == [0],
        0..1 r<< -1 == [0],
        0..1 r<< -2 == [0],
        //
        1..2 r<< 0 == [1],
        1..2 r<< 1 == [1],
        1..2 r<< 2 == [1],
        1..2 r<< -1 == [1],
        1..2 r<< -2 == [1],
        //
        0..2 r<< 0 == [0, 1],
        0..2 r<< 1 == [1, 0],
        0..2 r<< 2 == [0, 1],
        0..2 r<< 3 == [1, 0],
        0..2 r<< 4 == [0, 1],
        0..2 r<< -1 == [1, 0],
        0..2 r<< -2 == [0, 1],
        0..2 r<< -3 == [1, 0],
        0..2 r<< -4 == [0, 1],
        //
        1..3 r<< 0 == [1, 2],
        1..3 r<< 1 == [2, 1],
        1..3 r<< 2 == [1, 2],
        1..3 r<< 3 == [2, 1],
        1..3 r<< 4 == [1, 2],
        1..3 r<< -1 == [2, 1],
        1..3 r<< -2 == [1, 2],
        1..3 r<< -3 == [2, 1],
        1..3 r<< -4 == [1, 2],
        //
        0..3 r<< 0 == [0, 1, 2],
        0..3 r<< 1 == [1, 2, 0],
        0..3 r<< 2 == [2, 0, 1],
        0..3 r<< -1 == [2, 0, 1],
        0..3 r<< -2 == [1, 2, 0],
        //
        0..4 r<< 0 == [0, 1, 2, 3],
        0..4 r<< 1 == [1, 2, 3, 0],
        0..4 r<< 2 == [2, 3, 0, 1],
        0..4 r<< 3 == [3, 0, 1, 2],
        0..4 r<< -1 == [3, 0, 1, 2],
        0..4 r<< -2 == [2, 3, 0, 1],
        0..4 r<< -3 == [1, 2, 3, 0],
    );
}
