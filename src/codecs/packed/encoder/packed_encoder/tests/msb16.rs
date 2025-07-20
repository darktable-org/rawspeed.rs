use crate::packed_encoder::ExtraPadding;
use crate::packed_encoder::NumBytes;
use crate::packed_encoder::Packer;
use rawspeed_memory_bitstreams::bitstreams::BitOrder;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;
use std::io::Write as _;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::MSB16;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = Cursor::new(vec![]);
    let _packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
}

#[test]
fn arr_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::MSB16;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = [0_u8; 1024];
    let mut buf = Cursor::new(buf.as_mut());
    let _packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
}

#[test]
fn flush_arr_overflow_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::MSB16;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = [0_u8; 0];
    let mut buf = Cursor::new(buf.as_mut());
    let packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
    packer.pack().unwrap_err();
}

#[test]
fn u8_enumeration_test() -> std::io::Result<()> {
    const BIT_ORDER: BitOrder = BitOrder::MSB16;
    type T = u8;
    const NUM_BITS: usize = 8;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![0; num_rows * num_cols];
            let mut img = Array2DRefMut::new(
                &mut storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    img[Coord2D::new(
                        RowIndex::new(row - 1),
                        ColIndex::new(col - 1),
                    )] = (10 * row + col).try_into().unwrap();
                }
            }
            let img = img.into();
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let mut buf = Cursor::new(vec![]);
                let packer =
                    Packer::new(&mut buf, BIT_ORDER, NUM_BITS, img, |_| {
                        ExtraPadding::new(NumBytes::new(num_padding_bytes))
                    });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 11, 0, 0],
        vec![0, 11, 0, 0, 0],
        vec![12, 11, 0, 0],
        vec![12, 11, 0, 0, 0],
        vec![12, 11, 0, 13],
        vec![12, 11, 0, 13, 0],
        vec![12, 11, 14, 13],
        vec![12, 11, 14, 13, 0],
        vec![12, 11, 14, 13, 0, 15, 0, 0],
        vec![12, 11, 14, 13, 0, 15, 0, 0, 0],
        vec![12, 11, 14, 13, 16, 15, 0, 0],
        vec![12, 11, 14, 13, 16, 15, 0, 0, 0],
        vec![12, 11, 14, 13, 16, 15, 0, 17],
        vec![12, 11, 14, 13, 16, 15, 0, 17, 0],
        vec![12, 11, 14, 13, 16, 15, 18, 17],
        vec![12, 11, 14, 13, 16, 15, 18, 17, 0],
        vec![0, 11, 0, 0, 0, 21, 0, 0],
        vec![0, 11, 0, 0, 0, 0, 21, 0, 0, 0],
        vec![12, 11, 0, 0, 22, 21, 0, 0],
        vec![12, 11, 0, 0, 0, 22, 21, 0, 0, 0],
        vec![12, 11, 0, 13, 22, 21, 0, 23],
        vec![12, 11, 0, 13, 0, 22, 21, 0, 23, 0],
        vec![12, 11, 14, 13, 22, 21, 24, 23],
        vec![12, 11, 14, 13, 0, 22, 21, 24, 23, 0],
        vec![12, 11, 14, 13, 0, 15, 0, 0, 22, 21, 24, 23, 0, 25, 0, 0],
        vec![
            12, 11, 14, 13, 0, 15, 0, 0, 0, 22, 21, 24, 23, 0, 25, 0, 0, 0,
        ],
        vec![12, 11, 14, 13, 16, 15, 0, 0, 22, 21, 24, 23, 26, 25, 0, 0],
        vec![
            12, 11, 14, 13, 16, 15, 0, 0, 0, 22, 21, 24, 23, 26, 25, 0, 0, 0,
        ],
        vec![12, 11, 14, 13, 16, 15, 0, 17, 22, 21, 24, 23, 26, 25, 0, 27],
        vec![
            12, 11, 14, 13, 16, 15, 0, 17, 0, 22, 21, 24, 23, 26, 25, 0, 27, 0,
        ],
        vec![
            12, 11, 14, 13, 16, 15, 18, 17, 22, 21, 24, 23, 26, 25, 28, 27,
        ],
        vec![
            12, 11, 14, 13, 16, 15, 18, 17, 0, 22, 21, 24, 23, 26, 25, 28, 27,
            0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u4_enumeration_test() -> std::io::Result<()> {
    const BIT_ORDER: BitOrder = BitOrder::MSB16;
    type T = u8;
    const NUM_BITS: usize = 4;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=7 {
            let mut storage: Vec<T> = vec![0; num_rows * num_cols];
            let mut img = Array2DRefMut::new(
                &mut storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for row in 0..num_rows {
                for col in 0..num_cols {
                    img[Coord2D::new(RowIndex::new(row), ColIndex::new(col))] =
                        (1 + num_cols * row + col).try_into().unwrap();
                }
            }
            let img = img.into();
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let mut buf = Cursor::new(vec![]);
                let packer =
                    Packer::new(&mut buf, BIT_ORDER, NUM_BITS, img, |_| {
                        ExtraPadding::new(NumBytes::new(num_padding_bytes))
                    });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 16, 0, 0],
        vec![0, 16, 0, 0, 0],
        vec![0, 18, 0, 0],
        vec![0, 18, 0, 0, 0],
        vec![48, 18, 0, 0],
        vec![48, 18, 0, 0, 0],
        vec![52, 18, 0, 0],
        vec![52, 18, 0, 0, 0],
        vec![52, 18, 0, 80],
        vec![52, 18, 0, 80, 0],
        vec![52, 18, 0, 86],
        vec![52, 18, 0, 86, 0],
        vec![52, 18, 112, 86],
        vec![52, 18, 112, 86, 0],
        vec![0, 16, 0, 0, 0, 32, 0, 0],
        vec![0, 16, 0, 0, 0, 0, 32, 0, 0, 0],
        vec![0, 18, 0, 0, 0, 52, 0, 0],
        vec![0, 18, 0, 0, 0, 0, 52, 0, 0, 0],
        vec![48, 18, 0, 0, 96, 69, 0, 0],
        vec![48, 18, 0, 0, 0, 96, 69, 0, 0, 0],
        vec![52, 18, 0, 0, 120, 86, 0, 0],
        vec![52, 18, 0, 0, 0, 120, 86, 0, 0, 0],
        vec![52, 18, 0, 80, 137, 103, 0, 160],
        vec![52, 18, 0, 80, 0, 137, 103, 0, 160, 0],
        vec![52, 18, 0, 86, 154, 120, 0, 188],
        vec![52, 18, 0, 86, 0, 154, 120, 0, 188, 0],
        vec![52, 18, 112, 86, 171, 137, 224, 205],
        vec![52, 18, 112, 86, 0, 171, 137, 224, 205, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u16_enumeration_test() -> std::io::Result<()> {
    const BIT_ORDER: BitOrder = BitOrder::MSB16;
    type T = u16;
    const NUM_BITS: usize = 16;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![0; num_rows * num_cols];
            let mut img = Array2DRefMut::new(
                &mut storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    img[Coord2D::new(
                        RowIndex::new(row - 1),
                        ColIndex::new(col - 1),
                    )] = (10 * row + col).try_into().unwrap();
                }
            }
            let img = img.into();
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let mut buf = Cursor::new(vec![]);
                let packer =
                    Packer::new(&mut buf, BIT_ORDER, NUM_BITS, img, |_| {
                        ExtraPadding::new(NumBytes::new(num_padding_bytes))
                    });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 0, 12, 0],
        vec![11, 0, 12, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 0],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 21, 0, 22, 0],
        vec![11, 0, 12, 0, 0, 21, 0, 22, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 21, 0, 22, 0, 23, 0, 24, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 0, 21, 0, 22, 0, 23, 0, 24, 0, 0],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 21, 0, 22, 0, 23, 0, 24,
            0, 25, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 21, 0, 22, 0, 23, 0, 24,
            0, 25, 0, 26, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 26, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 21, 0, 22,
            0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 21, 0, 22,
            0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u12_enumeration_test() -> std::io::Result<()> {
    const BIT_ORDER: BitOrder = BitOrder::MSB16;
    type T = u16;
    const NUM_BITS: usize = 12;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![0; num_rows * num_cols];
            let mut img = Array2DRefMut::new(
                &mut storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    img[Coord2D::new(
                        RowIndex::new(row - 1),
                        ColIndex::new(col - 1),
                    )] = (10 * row + col).try_into().unwrap();
                }
            }
            let img = img.into();
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let mut buf = Cursor::new(vec![]);
                let packer =
                    Packer::new(&mut buf, BIT_ORDER, NUM_BITS, img, |_| {
                        ExtraPadding::new(NumBytes::new(num_padding_bytes))
                    });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![176, 0, 0, 0],
        vec![176, 0, 0, 0, 0],
        vec![176, 0, 0, 12],
        vec![176, 0, 0, 12, 0],
        vec![176, 0, 0, 12, 0, 208, 0, 0],
        vec![176, 0, 0, 12, 0, 208, 0, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 0, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 0, 16, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 0, 16, 0, 0, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 0, 16],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 0, 16, 0],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 18, 16],
        vec![176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 18, 16, 0],
        vec![176, 0, 0, 0, 80, 1, 0, 0],
        vec![176, 0, 0, 0, 0, 80, 1, 0, 0, 0],
        vec![176, 0, 0, 12, 80, 1, 0, 22],
        vec![176, 0, 0, 12, 0, 80, 1, 0, 22, 0],
        vec![176, 0, 0, 12, 0, 208, 0, 0, 80, 1, 1, 22, 0, 112, 0, 0],
        vec![
            176, 0, 0, 12, 0, 208, 0, 0, 0, 80, 1, 1, 22, 0, 112, 0, 0, 0,
        ],
        vec![176, 0, 0, 12, 14, 208, 0, 0, 80, 1, 1, 22, 24, 112, 0, 0],
        vec![
            176, 0, 0, 12, 14, 208, 0, 0, 0, 80, 1, 1, 22, 24, 112, 0, 0, 0,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 80, 1, 1, 22, 24, 112, 144, 1,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 0, 80, 1, 1, 22, 24, 112, 144, 1, 0,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 0, 16, 0, 0, 80, 1, 1, 22, 24, 112,
            144, 1, 0, 26, 0, 0,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 0, 16, 0, 0, 0, 80, 1, 1, 22, 24,
            112, 144, 1, 0, 26, 0, 0, 0,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 0, 16, 80, 1, 1, 22, 24,
            112, 144, 1, 1, 26, 0, 176,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 0, 16, 0, 80, 1, 1, 22, 24,
            112, 144, 1, 1, 26, 0, 176, 0,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 18, 16, 80, 1, 1, 22, 24,
            112, 144, 1, 1, 26, 28, 176,
        ],
        vec![
            176, 0, 0, 12, 14, 208, 240, 0, 1, 16, 18, 16, 0, 80, 1, 1, 22, 24,
            112, 144, 1, 1, 26, 28, 176, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
#[expect(clippy::too_many_lines)]
fn u32_enumeration_test() -> std::io::Result<()> {
    const BIT_ORDER: BitOrder = BitOrder::MSB16;
    type T = u32;
    const NUM_BITS: usize = 32;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![0; num_rows * num_cols];
            let mut img = Array2DRefMut::new(
                &mut storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    img[Coord2D::new(
                        RowIndex::new(row - 1),
                        ColIndex::new(col - 1),
                    )] = (10 * row + col).try_into().unwrap();
                }
            }
            let img = img.into();
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let mut buf = Cursor::new(vec![]);
                let packer =
                    Packer::new(&mut buf, BIT_ORDER, NUM_BITS, img, |_| {
                        ExtraPadding::new(NumBytes::new(num_padding_bytes))
                    });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 11, 0],
        vec![0, 0, 11, 0, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0,
        ],
        vec![0, 0, 11, 0, 0, 0, 21, 0],
        vec![0, 0, 11, 0, 0, 0, 0, 21, 0, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 21, 0, 0, 0, 22, 0],
        vec![0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0,
            0, 23, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0,
            0, 0, 23, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 21, 0, 0,
            0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 21, 0,
            0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
            25, 0, 0, 0, 26, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0,
            0, 25, 0, 0, 0, 26, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0,
            24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0,
            0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0,
        ],
        vec![
            0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0,
            0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0,
            0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}
