use crate::{
    bound_numerics::{Bound, BoundUnsigned},
    wrapping_numerics::WrappingUnsigned,
};

#[test]
fn wrapping_test() {
    let b = Bound::<u8>::new(1);
    let v = BoundUnsigned::new(b, 0).unwrap();
    let r = WrappingUnsigned::new(v);
    assert_eq!(*r.value(), v);
    assert_eq!(*r, v);
}
