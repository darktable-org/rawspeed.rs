use super::{binary, euclid, naive, sub};

macro_rules! test {
    ($fn:ident: $(gcd ($lhs:expr, $rhs:expr) == $res:expr,)+) => {
        $(
            assert_eq!(
                <u8 as $fn::GCD>::gcd($lhs, $rhs), $res
            );
        )+
    };
}

macro_rules! impl_test {
    ($fn:ident) => {
        test!(
            $fn:
            //
            gcd(0, 0) == 0,
            gcd(0, 1) == 1,
            gcd(0, 2) == 2,
            gcd(0, 3) == 3,
            gcd(0, 4) == 4,
            gcd(0, 5) == 5,
            gcd(0, 6) == 6,
            //
            gcd(1, 0) == 1,
            gcd(1, 1) == 1,
            gcd(1, 2) == 1,
            gcd(1, 3) == 1,
            gcd(1, 4) == 1,
            gcd(1, 5) == 1,
            gcd(1, 6) == 1,
            //
            gcd(2, 0) == 2,
            gcd(2, 1) == 1,
            gcd(2, 2) == 2,
            gcd(2, 3) == 1,
            gcd(2, 4) == 2,
            gcd(2, 5) == 1,
            gcd(2, 6) == 2,
            //
            gcd(3, 0) == 3,
            gcd(3, 1) == 1,
            gcd(3, 2) == 1,
            gcd(3, 3) == 3,
            gcd(3, 4) == 1,
            gcd(3, 5) == 1,
            gcd(3, 6) == 3,
            //
            gcd(4, 0) == 4,
            gcd(4, 1) == 1,
            gcd(4, 2) == 2,
            gcd(4, 3) == 1,
            gcd(4, 4) == 4,
            gcd(4, 5) == 1,
            gcd(4, 6) == 2,
            //
            gcd(5, 0) == 5,
            gcd(5, 1) == 1,
            gcd(5, 2) == 1,
            gcd(5, 3) == 1,
            gcd(5, 4) == 1,
            gcd(5, 5) == 5,
            gcd(5, 6) == 1,
            //
            gcd(6, 0) == 6,
            gcd(6, 1) == 1,
            gcd(6, 2) == 2,
            gcd(6, 3) == 3,
            gcd(6, 4) == 2,
            gcd(6, 5) == 1,
            gcd(6, 6) == 6,
        );
    }
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn naive_test() {
    impl_test!(naive);
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn sub_test() {
    impl_test!(sub);
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn euclid_test() {
    impl_test!(euclid);
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn binary_test() {
    impl_test!(binary);
}

#[test]
#[cfg_attr(miri, ignore)]
fn equivalency_test() {
    for a in u8::MIN..=u8::MAX {
        for b in u8::MIN..=u8::MAX {
            let naive = <u8 as naive::GCD>::gcd(a, b);
            let sub = <u8 as sub::GCD>::gcd(a, b);
            let euclid = <u8 as euclid::GCD>::gcd(a, b);
            let binary = <u8 as binary::GCD>::gcd(a, b);
            assert_eq!(naive, sub);
            assert_eq!(sub, euclid);
            assert_eq!(euclid, binary);
        }
    }
}
