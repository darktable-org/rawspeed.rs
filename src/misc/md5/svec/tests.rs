use super::*;

#[test]
fn n0_empty_test() {
    const N: usize = 0;
    type T = u8;
    let vec: SVec<T, N> = SVec::default();
    assert_eq!(vec[..], []);
}

#[test]
#[should_panic(expected = "Buffer overflow")]
fn n0_extend_overflow_test() {
    const N: usize = 0;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42]);
}

#[test]
#[should_panic(expected = "Buffer is not full yet!")]
fn n1_empty_test() {
    const N: usize = 1;
    type T = u8;
    let vec: SVec<T, N> = SVec::default();
    assert_eq!(vec[..], []);
}

#[test]
fn n1_extend_1_test() {
    const N: usize = 1;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42]);
    assert_eq!(vec[..], [42]);
}

#[test]
#[should_panic(expected = "Buffer overflow")]
fn n1_extend_2_test() {
    const N: usize = 1;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42, 24]);
    assert_eq!(vec[..], []);
}

#[test]
#[should_panic(expected = "Buffer is not full yet!")]
fn n2_empty_test() {
    const N: usize = 2;
    type T = u8;
    let vec: SVec<T, N> = SVec::default();
    assert_eq!(vec[..], []);
}

#[test]
#[should_panic(expected = "Buffer is not full yet!")]
fn n2_extend_1_test() {
    const N: usize = 2;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42]);
    assert_eq!(vec[..], []);
}

#[test]
fn n2_extend_2_test() {
    const N: usize = 2;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42, 24]);
    assert_eq!(vec[..], [42, 24]);
}

#[test]
fn n2_extend_2_separately_test() {
    const N: usize = 2;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42]);
    vec.extend(&[24]);
    assert_eq!(vec[..], [42, 24]);
}

#[test]
#[should_panic(expected = "Buffer overflow")]
fn n2_extend_3_test() {
    const N: usize = 2;
    type T = u8;
    let mut vec: SVec<T, N> = SVec::default();
    vec.extend(&[42, 24, 44]);
    assert_eq!(vec[..], []);
}
