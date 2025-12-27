use crate::bound_numerics::{Bound, BoundUnsigned};

#[test]
fn bound_test() {
    let b = Bound::<u8>::new(1).unwrap();
    assert_eq!(*b, 1);
    assert_eq!(*b.domain(), 1);
}

#[test]
fn zero_bound_test() {
    assert!(Bound::<u8>::new(0).is_none());
}

#[test]
fn bounded_test() {
    let b = Bound::<u8>::new(1).unwrap();
    let v = BoundUnsigned::new(b, 0).unwrap();
    assert_eq!(*v, 0);
    assert_eq!(*v.value(), 0);
    assert_eq!(*b.domain(), 1);
}

#[test]
fn bounded_ov_test() {
    let b = Bound::<u8>::new(1).unwrap();
    assert!(BoundUnsigned::new(b, 1).is_none());
}
