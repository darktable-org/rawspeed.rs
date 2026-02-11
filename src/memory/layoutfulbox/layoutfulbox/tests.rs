use rawspeed_common_exact_ops::exact_ops::shl::CheckedShlExact;
use rawspeed_std::coord_common::{Align, ByteMultiple};

use crate::layoutfulbox::{LayoutfulBox, LayoutfulBoxError};

fn runtime_align_of<T>(x: &T) -> Align {
    let ptr: *const T = x;
    let addr = ptr as usize;
    let num_tz = addr.trailing_zeros();
    let align = CheckedShlExact::checked_shl_exact(1_usize, num_tz).unwrap();
    Align::new(ByteMultiple::new(align)).unwrap()
}

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
#[cfg_attr(
    any(not(target_pointer_width = "64"), miri),
    ignore = "this test only makes sense for ridiculous allocations sizes (1PB+?), but at the same time requires native compilation (i.e. no interpreters/sanitizers)"
)]
fn usable_max_test() {
    assert_eq!(
        Err(LayoutfulBoxError::OutOfMemory),
        LayoutfulBox::<u8>::new(
            core::alloc::Layout::array::<u8>(isize::MAX.try_into().unwrap())
                .unwrap()
        )
    );
}

fn slice_test<T>()
where
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
{
    for align in 0..=20 {
        for len in 1..=64 {
            let align =
                CheckedShlExact::checked_shl_exact(1_usize, align).unwrap();
            let mut buf = LayoutfulBox::<T>::new(
                core::alloc::Layout::array::<T>(len)
                    .unwrap()
                    .align_to(align)
                    .unwrap(),
            )
            .unwrap();
            let slice = buf.get_slice_mut();
            assert_eq!(slice.len(), len);
            assert!(
                runtime_align_of(slice.first().unwrap())
                    >= Align::new(ByteMultiple::new(align)).unwrap()
            );
            for (i, e) in slice.iter_mut().enumerate() {
                *e = i.try_into().unwrap();
            }
        }
    }
}

#[test]
fn slice_len_8_test() {
    slice_test::<u8>();
}

#[test]
fn slice_len_u64_test() {
    slice_test::<u64>();
}
