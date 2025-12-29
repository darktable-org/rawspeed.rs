use crate::{
    coord_common::{RowCount, RowIndex},
    wrapping_coord::BoundRowIndex,
};

#[test]
fn bound_test() {
    let c = RowCount::new(1);
    let i = RowIndex::new(0);
    let b = BoundRowIndex::new(c, i).unwrap();
    assert_eq!(*b, i);
    assert_eq!(b.value(), i);
    assert_eq!(b.domain(), c);
}

#[test]
fn zero_bound_test() {
    let c = RowCount::new(0);
    let i = RowIndex::new(0);
    assert!(BoundRowIndex::new(c, i).is_none());
}

#[test]
fn bound_ov_test() {
    let c = RowCount::new(1);
    let i = RowIndex::new(1);
    assert!(BoundRowIndex::new(c, i).is_none());
}
