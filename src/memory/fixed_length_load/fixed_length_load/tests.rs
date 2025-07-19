use super::*;

pub trait Tr {
    type Ty;
}

struct S;

impl Tr for S {
    type Ty = [u8; 3];
}

fn copy_from_slice_wrapper<T>(src: &[u8]) -> <T as Tr>::Ty
where
    T: Tr,
    <T as Tr>::Ty: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <<T as Tr>::Ty as core::ops::Index<core::ops::RangeFull>>::Output:
        CopyFromSlice,
{
    let mut out: <T as Tr>::Ty = Default::default();
    out[..].copy_from_slice_(src);
    out
}

fn load_from_slice_wrapper<T>(src: &[u8]) -> <T as Tr>::Ty
where
    T: Tr,
    <T as Tr>::Ty: Default + core::ops::IndexMut<core::ops::RangeFull>,
    <<T as Tr>::Ty as core::ops::Index<core::ops::RangeFull>>::Output:
        CopyFromSlice,
{
    src.load_from_slice()
}

#[test]
fn copy_from_slice_test() {
    let src: [u8; 3] = [1, 2, 3];
    let out = copy_from_slice_wrapper::<S>(&src[..]);
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn load_from_slice_test() {
    type Ty = <S as Tr>::Ty;
    let src: Ty = [1, 2, 3];
    let out = load_from_slice_wrapper::<S>(&src[..]);
    assert_eq!(out, [1, 2, 3]);
}
