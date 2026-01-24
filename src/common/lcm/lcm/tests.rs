use super::{constant, intersect, naive, via_gcd};

macro_rules! test {
    (($fn:expr): $(lcm ($lhs:expr, $rhs:expr) == $res:expr,)+) => {
        $(
            assert_eq!($fn($lhs, $rhs), Some($res));
        )+
    };
}

macro_rules! impl_test {
    ($fn:expr) => {
        test!(
            ($fn):
            lcm(0, 0) == 0,
            lcm(0, 1) == 0,
            lcm(0, 2) == 0,
            lcm(0, 3) == 0,
            lcm(0, 4) == 0,
            lcm(0, 5) == 0,
            lcm(0, 6) == 0,
            lcm(1, 0) == 0,
            lcm(1, 1) == 1,
            lcm(1, 2) == 2,
            lcm(1, 3) == 3,
            lcm(1, 4) == 4,
            lcm(1, 5) == 5,
            lcm(1, 6) == 6,
            lcm(2, 0) == 0,
            lcm(2, 1) == 2,
            lcm(2, 2) == 2,
            lcm(2, 3) == 6,
            lcm(2, 4) == 4,
            lcm(2, 5) == 10,
            lcm(2, 6) == 6,
            lcm(3, 0) == 0,
            lcm(3, 1) == 3,
            lcm(3, 2) == 6,
            lcm(3, 3) == 3,
            lcm(3, 4) == 12,
            lcm(3, 5) == 15,
            lcm(3, 6) == 6,
            lcm(4, 0) == 0,
            lcm(4, 1) == 4,
            lcm(4, 2) == 4,
            lcm(4, 3) == 12,
            lcm(4, 4) == 4,
            lcm(4, 5) == 20,
            lcm(4, 6) == 12,
            lcm(5, 0) == 0,
            lcm(5, 1) == 5,
            lcm(5, 2) == 10,
            lcm(5, 3) == 15,
            lcm(5, 4) == 20,
            lcm(5, 5) == 5,
            lcm(5, 6) == 30,
            lcm(6, 0) == 0,
            lcm(6, 1) == 6,
            lcm(6, 2) == 6,
            lcm(6, 3) == 6,
            lcm(6, 4) == 12,
            lcm(6, 5) == 30,
            lcm(6, 6) == 6,
        );
    }
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn naive_test() {
    impl_test!(<u8 as naive::LCM>::lcm);
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn intersect_test() {
    impl_test!(<u8 as intersect::LCM>::lcm);
}

#[test]
fn const_time_lcm_test() {
    #[derive(Default)]
    struct Ty([u8; constant::lcm!(2_u8, 3).unwrap() as usize]);
    let q: Ty = Ty::default();
    assert_eq!(q.0.len(), 6);
}

#[test]
#[cfg_attr(miri, ignore)]
fn equivalency_test() {
    for a in u8::MIN..=u8::MAX {
        for b in u8::MIN..=u8::MAX {
            let naive = <u8 as naive::LCM>::lcm(a, b);
            let intersect = <u8 as intersect::LCM>::lcm(a, b);
            let via_gcd = <u8 as via_gcd::LCM>::lcm(a, b);
            let constant = constant::lcm!(a, b);
            assert_eq!(naive, intersect);
            assert_eq!(intersect, via_gcd);
            assert_eq!(via_gcd, constant);
        }
    }
}
