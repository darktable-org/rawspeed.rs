use super::{AsSlice, img_data_hash};
use rawspeed_common::bit_transmutation::ToLeBytes;
use rawspeed_common::common::Bitwidth;
use rawspeed_memory_nd_slice_procurement::ndsliceprocurement::NDSliceProcurementRequest;
use rawspeed_std::coord_common::{
    ColIndex, Coord2D, Dimensions2D, RowCount, RowIndex, RowLength,
};

fn from_halves<T>(hi: T, lo: T) -> T
where
    T: Bitwidth + core::ops::Shl<usize>,
    <T as core::ops::Shl<usize>>::Output: core::ops::BitOr<T, Output = T>,
{
    let bitwidth = T::BITWIDTH;
    assert!(bitwidth.is_multiple_of(2));
    (hi << (bitwidth / 2)) | lo
}

fn image_hash_test<T>(dims: Dimensions2D, str: &'static str)
where
    T: Bitwidth + core::ops::Shl<usize> + Copy + TryFrom<usize> + ToLeBytes,
    <T as core::ops::Shl<usize>>::Output: core::ops::BitOr<T, Output = T>,
    <T as ToLeBytes>::Output: AsSlice<Element = u8>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
{
    let mut out = NDSliceProcurementRequest::<T>::new(dims).fulfill().unwrap();
    let mut img = out.get_mut();
    for row in 1..=*dims.row_count() {
        for col in 1..=*dims.row_len() {
            img[Coord2D::new(RowIndex::new(row - 1), ColIndex::new(col - 1))] =
                from_halves::<T>(
                    row.try_into().unwrap(),
                    col.try_into().unwrap(),
                );
        }
    }
    let res = img_data_hash(img.into());
    assert_eq!(res, str);
}

#[test]
fn image_hash_u8_test() {
    type T = u8;
    let params = vec![
        ((1, 1), "51bfa529a7c5f047217dc0867aa731d4"),
        ((1, 2), "db1d3730e3696baac948d434f9080b0b"),
        ((2, 1), "bd2a3ff9e3c81c01a83f6372df0d867a"),
        ((2, 2), "bfc1a79553bcb9377781ddad623cf39b"),
    ];
    for ((w, h), str) in params {
        image_hash_test::<T>(
            Dimensions2D::new(RowLength::new(w), RowCount::new(h)),
            str,
        );
    }
}

#[test]
fn image_hash_u16_test() {
    type T = u16;
    let params = vec![
        ((1, 1), "cfebf85e838306c289a4cc291e544f9e"),
        ((1, 2), "24a03faae6de848468d1b539f162c032"),
        ((2, 1), "14ce3541e1b1eecefd479aaf148ca556"),
        ((2, 2), "5fb0ee343ec52da356b1b20c719c78df"),
    ];
    for ((w, h), str) in params {
        image_hash_test::<T>(
            Dimensions2D::new(RowLength::new(w), RowCount::new(h)),
            str,
        );
    }
}
