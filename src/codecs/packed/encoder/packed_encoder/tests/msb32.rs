use crate::packed_encoder::ExtraPadding;
use crate::packed_encoder::NumBytes;
use crate::packed_encoder::Packer;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use std::io::Write as _;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::MSB32;
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
    let bit_order = BitOrder::MSB32;
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
    let bit_order = BitOrder::MSB32;
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
    type T = u8;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::MSB32;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 11],
        vec![0, 0, 0, 11, 0],
        vec![0, 0, 12, 11],
        vec![0, 0, 12, 11, 0],
        vec![0, 13, 12, 11],
        vec![0, 13, 12, 11, 0],
        vec![14, 13, 12, 11],
        vec![14, 13, 12, 11, 0],
        vec![14, 13, 12, 11, 0, 0, 0, 15],
        vec![14, 13, 12, 11, 0, 0, 0, 15, 0],
        vec![14, 13, 12, 11, 0, 0, 16, 15],
        vec![14, 13, 12, 11, 0, 0, 16, 15, 0],
        vec![14, 13, 12, 11, 0, 17, 16, 15],
        vec![14, 13, 12, 11, 0, 17, 16, 15, 0],
        vec![14, 13, 12, 11, 18, 17, 16, 15],
        vec![14, 13, 12, 11, 18, 17, 16, 15, 0],
        vec![0, 0, 0, 11, 0, 0, 0, 21],
        vec![0, 0, 0, 11, 0, 0, 0, 0, 21, 0],
        vec![0, 0, 12, 11, 0, 0, 22, 21],
        vec![0, 0, 12, 11, 0, 0, 0, 22, 21, 0],
        vec![0, 13, 12, 11, 0, 23, 22, 21],
        vec![0, 13, 12, 11, 0, 0, 23, 22, 21, 0],
        vec![14, 13, 12, 11, 24, 23, 22, 21],
        vec![14, 13, 12, 11, 0, 24, 23, 22, 21, 0],
        vec![14, 13, 12, 11, 0, 0, 0, 15, 24, 23, 22, 21, 0, 0, 0, 25],
        vec![
            14, 13, 12, 11, 0, 0, 0, 15, 0, 24, 23, 22, 21, 0, 0, 0, 25, 0,
        ],
        vec![14, 13, 12, 11, 0, 0, 16, 15, 24, 23, 22, 21, 0, 0, 26, 25],
        vec![
            14, 13, 12, 11, 0, 0, 16, 15, 0, 24, 23, 22, 21, 0, 0, 26, 25, 0,
        ],
        vec![14, 13, 12, 11, 0, 17, 16, 15, 24, 23, 22, 21, 0, 27, 26, 25],
        vec![
            14, 13, 12, 11, 0, 17, 16, 15, 0, 24, 23, 22, 21, 0, 27, 26, 25, 0,
        ],
        vec![
            14, 13, 12, 11, 18, 17, 16, 15, 24, 23, 22, 21, 28, 27, 26, 25,
        ],
        vec![
            14, 13, 12, 11, 18, 17, 16, 15, 0, 24, 23, 22, 21, 28, 27, 26, 25,
            0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u4_enumeration_test() -> std::io::Result<()> {
    type T = u8;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=7 {
            let mut storage: Vec<T> = vec![];
            for e in 1..=(num_rows * num_cols) {
                assert!(e <= 0xF);
                storage.push(e.try_into().unwrap());
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::MSB32;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(&mut buf, bit_order, 4, img, |_| {
                    ExtraPadding::new(NumBytes::new(num_padding_bytes))
                });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 16],
        vec![0, 0, 0, 16, 0],
        vec![0, 0, 0, 18],
        vec![0, 0, 0, 18, 0],
        vec![0, 0, 48, 18],
        vec![0, 0, 48, 18, 0],
        vec![0, 0, 52, 18],
        vec![0, 0, 52, 18, 0],
        vec![0, 80, 52, 18],
        vec![0, 80, 52, 18, 0],
        vec![0, 86, 52, 18],
        vec![0, 86, 52, 18, 0],
        vec![112, 86, 52, 18],
        vec![112, 86, 52, 18, 0],
        vec![0, 0, 0, 16, 0, 0, 0, 32],
        vec![0, 0, 0, 16, 0, 0, 0, 0, 32, 0],
        vec![0, 0, 0, 18, 0, 0, 0, 52],
        vec![0, 0, 0, 18, 0, 0, 0, 0, 52, 0],
        vec![0, 0, 48, 18, 0, 0, 96, 69],
        vec![0, 0, 48, 18, 0, 0, 0, 96, 69, 0],
        vec![0, 0, 52, 18, 0, 0, 120, 86],
        vec![0, 0, 52, 18, 0, 0, 0, 120, 86, 0],
        vec![0, 80, 52, 18, 0, 160, 137, 103],
        vec![0, 80, 52, 18, 0, 0, 160, 137, 103, 0],
        vec![0, 86, 52, 18, 0, 188, 154, 120],
        vec![0, 86, 52, 18, 0, 0, 188, 154, 120, 0],
        vec![112, 86, 52, 18, 224, 205, 171, 137],
        vec![112, 86, 52, 18, 0, 224, 205, 171, 137, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u16_enumeration_test() -> std::io::Result<()> {
    type T = u16;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::MSB32;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 11, 0],
        vec![0, 0, 11, 0, 0],
        vec![12, 0, 11, 0],
        vec![12, 0, 11, 0, 0],
        vec![12, 0, 11, 0, 0, 0, 13, 0],
        vec![12, 0, 11, 0, 0, 0, 13, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 0, 0, 15, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 0, 0, 15, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0, 0, 17, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0, 0, 17, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 18, 0, 17, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 18, 0, 17, 0, 0],
        vec![0, 0, 11, 0, 0, 0, 21, 0],
        vec![0, 0, 11, 0, 0, 0, 0, 21, 0, 0],
        vec![12, 0, 11, 0, 22, 0, 21, 0],
        vec![12, 0, 11, 0, 0, 22, 0, 21, 0, 0],
        vec![12, 0, 11, 0, 0, 0, 13, 0, 22, 0, 21, 0, 0, 0, 23, 0],
        vec![12, 0, 11, 0, 0, 0, 13, 0, 0, 22, 0, 21, 0, 0, 0, 23, 0, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 22, 0, 21, 0, 24, 0, 23, 0],
        vec![12, 0, 11, 0, 14, 0, 13, 0, 0, 22, 0, 21, 0, 24, 0, 23, 0, 0],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 0, 0, 15, 0, 22, 0, 21, 0, 24, 0, 23,
            0, 0, 0, 25, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 0, 0, 15, 0, 0, 22, 0, 21, 0, 24, 0,
            23, 0, 0, 0, 25, 0, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 22, 0, 21, 0, 24, 0, 23,
            0, 26, 0, 25, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0, 22, 0, 21, 0, 24, 0,
            23, 0, 26, 0, 25, 0, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0, 0, 17, 0, 22, 0, 21,
            0, 24, 0, 23, 0, 26, 0, 25, 0, 0, 0, 27, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 0, 0, 17, 0, 0, 22, 0,
            21, 0, 24, 0, 23, 0, 26, 0, 25, 0, 0, 0, 27, 0, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 18, 0, 17, 0, 22, 0, 21,
            0, 24, 0, 23, 0, 26, 0, 25, 0, 28, 0, 27, 0,
        ],
        vec![
            12, 0, 11, 0, 14, 0, 13, 0, 16, 0, 15, 0, 18, 0, 17, 0, 0, 22, 0,
            21, 0, 24, 0, 23, 0, 26, 0, 25, 0, 28, 0, 27, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u12_enumeration_test() -> std::io::Result<()> {
    type T = u16;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::MSB32;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(&mut buf, bit_order, 12, img, |_| {
                    ExtraPadding::new(NumBytes::new(num_padding_bytes))
                });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 176, 0],
        vec![0, 0, 176, 0, 0],
        vec![0, 12, 176, 0],
        vec![0, 12, 176, 0, 0],
        vec![0, 12, 176, 0, 0, 0, 0, 208],
        vec![0, 12, 176, 0, 0, 0, 0, 208, 0],
        vec![0, 12, 176, 0, 0, 0, 14, 208],
        vec![0, 12, 176, 0, 0, 0, 14, 208, 0],
        vec![0, 12, 176, 0, 240, 0, 14, 208],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 0],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 0, 0, 0, 16],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 0, 0, 0, 16, 0],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 0, 16, 1, 16],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 0, 16, 1, 16, 0],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 18, 16, 1, 16],
        vec![0, 12, 176, 0, 240, 0, 14, 208, 18, 16, 1, 16, 0],
        vec![0, 0, 176, 0, 0, 0, 80, 1],
        vec![0, 0, 176, 0, 0, 0, 0, 80, 1, 0],
        vec![0, 12, 176, 0, 0, 22, 80, 1],
        vec![0, 12, 176, 0, 0, 0, 22, 80, 1, 0],
        vec![0, 12, 176, 0, 0, 0, 0, 208, 1, 22, 80, 1, 0, 0, 0, 112],
        vec![
            0, 12, 176, 0, 0, 0, 0, 208, 0, 1, 22, 80, 1, 0, 0, 0, 112, 0,
        ],
        vec![0, 12, 176, 0, 0, 0, 14, 208, 1, 22, 80, 1, 0, 0, 24, 112],
        vec![
            0, 12, 176, 0, 0, 0, 14, 208, 0, 1, 22, 80, 1, 0, 0, 24, 112, 0,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 1, 22, 80, 1, 144, 1, 24, 112,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 0, 1, 22, 80, 1, 144, 1, 24, 112, 0,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 0, 0, 0, 16, 1, 22, 80, 1, 144, 1,
            24, 112, 0, 0, 0, 26,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 0, 0, 0, 16, 0, 1, 22, 80, 1, 144,
            1, 24, 112, 0, 0, 0, 26, 0,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 0, 16, 1, 16, 1, 22, 80, 1, 144, 1,
            24, 112, 0, 176, 1, 26,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 0, 16, 1, 16, 0, 1, 22, 80, 1, 144,
            1, 24, 112, 0, 176, 1, 26, 0,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 18, 16, 1, 16, 1, 22, 80, 1, 144,
            1, 24, 112, 28, 176, 1, 26,
        ],
        vec![
            0, 12, 176, 0, 240, 0, 14, 208, 18, 16, 1, 16, 0, 1, 22, 80, 1,
            144, 1, 24, 112, 28, 176, 1, 26, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
#[expect(clippy::too_many_lines)]
fn u32_enumeration_test() -> std::io::Result<()> {
    type T = u32;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::MSB32;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0,
        ],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 0],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 21, 0, 0, 0,
            22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 21, 0, 0, 0,
            22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0,
            21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
            25, 0, 0, 0, 26, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
            25, 0, 0, 0, 26, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0,
            24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0,
            24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0, 0, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}
