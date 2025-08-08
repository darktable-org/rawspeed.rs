use crate::layoutfulbox::{LayoutfulBox, LayoutfulBoxError};

#[test]
fn basic_1xu8_align_1_as_u8_test() {
    LayoutfulBox::<u8>::new(
        core::alloc::Layout::array::<u8>(1)
            .unwrap()
            .align_to(1)
            .unwrap(),
    )
    .unwrap();
}

#[test]
fn basic_1xu8_align_2_as_u8_test() {
    LayoutfulBox::<u8>::new(
        core::alloc::Layout::array::<u8>(1)
            .unwrap()
            .align_to(2)
            .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "layout.size().is_multiple_of(size_of::<T>())")]
fn basic_1xu8_align_1_as_u16_test() {
    LayoutfulBox::<u16>::new(
        core::alloc::Layout::array::<u8>(1)
            .unwrap()
            .align_to(1)
            .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "layout.size().is_multiple_of(size_of::<T>())")]
fn basic_1xu8_align_2_as_u16_test() {
    LayoutfulBox::<u16>::new(
        core::alloc::Layout::array::<u8>(1)
            .unwrap()
            .align_to(2)
            .unwrap(),
    )
    .unwrap();
}

#[test]
fn basic_2xu8_align_1_as_u8_test() {
    LayoutfulBox::<u8>::new(
        core::alloc::Layout::array::<u8>(2)
            .unwrap()
            .align_to(1)
            .unwrap(),
    )
    .unwrap();
}

#[test]
fn basic_2xu8_align_2_as_u8_test() {
    LayoutfulBox::<u8>::new(
        core::alloc::Layout::array::<u8>(2)
            .unwrap()
            .align_to(2)
            .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "layout.align().is_multiple_of(align_of::<T>())")]
fn basic_2xu8_align_1_as_u16_test() {
    LayoutfulBox::<u16>::new(
        core::alloc::Layout::array::<u8>(2)
            .unwrap()
            .align_to(1)
            .unwrap(),
    )
    .unwrap();
}

#[test]
fn basic_2xu8_align_2_as_u16_test() {
    LayoutfulBox::<u16>::new(
        core::alloc::Layout::array::<u8>(2)
            .unwrap()
            .align_to(2)
            .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "layout.size() > 0")]
fn usable_0xu8_test() {
    drop(LayoutfulBox::<u8>::new(
        core::alloc::Layout::array::<u8>(0).unwrap(),
    ));
}

#[test]
#[cfg_attr(miri, ignore)]
fn usable_max_test() {
    assert_eq!(
        Err(LayoutfulBoxError::OutOfMemory),
        LayoutfulBox::<u8>::new(
            core::alloc::Layout::array::<u8>(isize::MAX.try_into().unwrap())
                .unwrap()
        )
    );
}

#[test]
fn slice_len_u8_test() {
    for len in 1..=64 {
        let mut buf = LayoutfulBox::<u8>::new(
            core::alloc::Layout::array::<u8>(len).unwrap(),
        )
        .unwrap();
        let slice = buf.get_slice_mut();
        assert_eq!(slice.len(), len);
        for (i, e) in slice.iter_mut().enumerate() {
            *e = i.try_into().unwrap();
        }
    }
}

#[test]
fn slice_len_u64_test() {
    for len in 1..=64 {
        let mut buf = LayoutfulBox::<u64>::new(
            core::alloc::Layout::array::<u64>(len).unwrap(),
        )
        .unwrap();
        let slice = buf.get_slice_mut();
        assert_eq!(slice.len(), len);
        for (i, e) in slice.iter_mut().enumerate() {
            *e = i.try_into().unwrap();
        }
    }
}
