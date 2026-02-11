use rawspeed_std::coord_common::{RowLength, RowPitch};
use rawspeed_std_ndslice::array2dref::Array2DRef;

use crate::colorfilterarray::{
    ColorVariant,
    dcraw_filter::{DCrawFilter, DCrawFilterError},
};

#[test]
fn bayer_rggb() {
    let data = [
        ColorVariant::Red,
        ColorVariant::Green,
        ColorVariant::Green,
        ColorVariant::Blue,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x9494_9494);
}

#[test]
fn bayer_gbrg() {
    let data = [
        ColorVariant::Green,
        ColorVariant::Blue,
        ColorVariant::Red,
        ColorVariant::Green,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x4949_4949);
}

#[test]
fn bayer_bggr() {
    let data = [
        ColorVariant::Blue,
        ColorVariant::Green,
        ColorVariant::Green,
        ColorVariant::Red,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x1616_1616);
}

#[test]
fn bayer_grgb() {
    let data = [
        ColorVariant::Green,
        ColorVariant::Red,
        ColorVariant::Blue,
        ColorVariant::Green,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x6161_6161);
}

#[test]
fn bayer_rygb() {
    let data = [
        ColorVariant::Red,
        ColorVariant::Yellow,
        ColorVariant::Green,
        ColorVariant::Blue,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x9c9c_9c9c);
}

#[test]
fn bayer_gmyc() {
    let data = [
        ColorVariant::FujiGreen,
        ColorVariant::Magenta,
        ColorVariant::Yellow,
        ColorVariant::Cyan,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0xb4b4_b4b4);
}

#[test]
fn invalid_basis() {
    let data = [
        ColorVariant::Red,
        ColorVariant::Green,
        ColorVariant::Green,
        ColorVariant::Magenta,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(
        DCrawFilter::try_from(mat),
        Err(DCrawFilterError::UnknownCFABasis)
    );
}

#[test]
fn bayer_rgbr() {
    let data = [
        ColorVariant::Red,
        ColorVariant::Green,
        ColorVariant::Blue,
        ColorVariant::Red,
    ];
    let mat = Array2DRef::new(&data, RowLength::new(2), RowPitch::new(2));
    assert_eq!(DCrawFilter::try_from(mat).unwrap().filter(), 0x2424_2424);
}

#[test]
fn xtrans() {
    let data =
        core::iter::repeat_n(ColorVariant::Red, 6 * 6).collect::<Vec<_>>();
    let mat = Array2DRef::new(&data, RowLength::new(6), RowPitch::new(6));
    assert_eq!(DCrawFilter::try_from(mat), Err(DCrawFilterError::XTrans));
}

#[test]
fn bad_dims() {
    for h in 1..=8 {
        for w in 1..=8 {
            let data = core::iter::repeat_n(ColorVariant::Red, w * h)
                .collect::<Vec<_>>();
            let mat =
                Array2DRef::new(&data, RowLength::new(w), RowPitch::new(w));
            let res = DCrawFilter::try_from(mat);
            let expected = match (w, h) {
                (6, 6) => DCrawFilterError::XTrans,
                (w, h)
                    if w.is_power_of_two() && h.is_power_of_two() && w <= 2 =>
                {
                    DCrawFilterError::UnknownCFABasis
                }
                (_, _) => DCrawFilterError::BadDims,
            };
            assert_eq!(res, Err(expected));
        }
    }
}
