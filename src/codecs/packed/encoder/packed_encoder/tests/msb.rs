use crate::packed_encoder::ExtraPadding;
use crate::packed_encoder::NumBytes;
use crate::packed_encoder::Packer;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_std::array2dref::Array2DRef;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use std::io::Write as _;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::MSB;
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
    let bit_order = BitOrder::MSB;
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
    let bit_order = BitOrder::MSB;
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
                let bit_order = BitOrder::MSB;
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
        vec![11, 12, 0, 0],
        vec![11, 12, 0, 0, 0],
        vec![11, 12, 13, 0],
        vec![11, 12, 13, 0, 0],
        vec![11, 12, 13, 14],
        vec![11, 12, 13, 14, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 18],
        vec![11, 12, 13, 14, 15, 16, 17, 18, 0],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 12, 0, 0, 21, 22, 0, 0],
        vec![11, 12, 0, 0, 0, 21, 22, 0, 0, 0],
        vec![11, 12, 13, 0, 21, 22, 23, 0],
        vec![11, 12, 13, 0, 0, 21, 22, 23, 0, 0],
        vec![11, 12, 13, 14, 21, 22, 23, 24],
        vec![11, 12, 13, 14, 0, 21, 22, 23, 24, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0, 21, 22, 23, 24, 25, 0, 0, 0],
        vec![
            11, 12, 13, 14, 15, 0, 0, 0, 0, 21, 22, 23, 24, 25, 0, 0, 0, 0,
        ],
        vec![11, 12, 13, 14, 15, 16, 0, 0, 21, 22, 23, 24, 25, 26, 0, 0],
        vec![
            11, 12, 13, 14, 15, 16, 0, 0, 0, 21, 22, 23, 24, 25, 26, 0, 0, 0,
        ],
        vec![11, 12, 13, 14, 15, 16, 17, 0, 21, 22, 23, 24, 25, 26, 27, 0],
        vec![
            11, 12, 13, 14, 15, 16, 17, 0, 0, 21, 22, 23, 24, 25, 26, 27, 0, 0,
        ],
        vec![
            11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28,
        ],
        vec![
            11, 12, 13, 14, 15, 16, 17, 18, 0, 21, 22, 23, 24, 25, 26, 27, 28,
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
                let bit_order = BitOrder::MSB;
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
        vec![16, 0, 0, 0],
        vec![16, 0, 0, 0, 0],
        vec![18, 0, 0, 0],
        vec![18, 0, 0, 0, 0],
        vec![18, 48, 0, 0],
        vec![18, 48, 0, 0, 0],
        vec![18, 52, 0, 0],
        vec![18, 52, 0, 0, 0],
        vec![18, 52, 80, 0],
        vec![18, 52, 80, 0, 0],
        vec![18, 52, 86, 0],
        vec![18, 52, 86, 0, 0],
        vec![18, 52, 86, 112],
        vec![18, 52, 86, 112, 0],
        vec![16, 0, 0, 0, 32, 0, 0, 0],
        vec![16, 0, 0, 0, 0, 32, 0, 0, 0, 0],
        vec![18, 0, 0, 0, 52, 0, 0, 0],
        vec![18, 0, 0, 0, 0, 52, 0, 0, 0, 0],
        vec![18, 48, 0, 0, 69, 96, 0, 0],
        vec![18, 48, 0, 0, 0, 69, 96, 0, 0, 0],
        vec![18, 52, 0, 0, 86, 120, 0, 0],
        vec![18, 52, 0, 0, 0, 86, 120, 0, 0, 0],
        vec![18, 52, 80, 0, 103, 137, 160, 0],
        vec![18, 52, 80, 0, 0, 103, 137, 160, 0, 0],
        vec![18, 52, 86, 0, 120, 154, 188, 0],
        vec![18, 52, 86, 0, 0, 120, 154, 188, 0, 0],
        vec![18, 52, 86, 112, 137, 171, 205, 224],
        vec![18, 52, 86, 112, 0, 137, 171, 205, 224, 0],
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
                let bit_order = BitOrder::MSB;
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
        vec![0, 11, 0, 0],
        vec![0, 11, 0, 0, 0],
        vec![0, 11, 0, 12],
        vec![0, 11, 0, 12, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0],
        vec![0, 11, 0, 0, 0, 21, 0, 0],
        vec![0, 11, 0, 0, 0, 0, 21, 0, 0, 0],
        vec![0, 11, 0, 12, 0, 21, 0, 22],
        vec![0, 11, 0, 12, 0, 0, 21, 0, 22, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0, 0],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 21, 0, 22, 0, 23, 0, 24],
        vec![0, 11, 0, 12, 0, 13, 0, 14, 0, 0, 21, 0, 22, 0, 23, 0, 24, 0],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 0,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 0, 0,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 26,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 0, 21, 0, 22, 0, 23,
            0, 24, 0, 25, 0, 26, 0,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0, 0,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28,
        ],
        vec![
            0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 0, 21,
            0, 22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28, 0,
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
                let bit_order = BitOrder::MSB;
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
        vec![0, 176, 0, 0],
        vec![0, 176, 0, 0, 0],
        vec![0, 176, 12, 0],
        vec![0, 176, 12, 0, 0],
        vec![0, 176, 12, 0, 208, 0, 0, 0],
        vec![0, 176, 12, 0, 208, 0, 0, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 0, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 0, 0, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 0, 0],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 18],
        vec![0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 18, 0],
        vec![0, 176, 0, 0, 1, 80, 0, 0],
        vec![0, 176, 0, 0, 0, 1, 80, 0, 0, 0],
        vec![0, 176, 12, 0, 1, 80, 22, 0],
        vec![0, 176, 12, 0, 0, 1, 80, 22, 0, 0],
        vec![0, 176, 12, 0, 208, 0, 0, 0, 1, 80, 22, 1, 112, 0, 0, 0],
        vec![
            0, 176, 12, 0, 208, 0, 0, 0, 0, 1, 80, 22, 1, 112, 0, 0, 0, 0,
        ],
        vec![0, 176, 12, 0, 208, 14, 0, 0, 1, 80, 22, 1, 112, 24, 0, 0],
        vec![
            0, 176, 12, 0, 208, 14, 0, 0, 0, 1, 80, 22, 1, 112, 24, 0, 0, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 1, 80, 22, 1, 112, 24, 1, 144,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 0, 1, 80, 22, 1, 112, 24, 1, 144, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 0, 0, 0, 1, 80, 22, 1, 112, 24,
            1, 144, 26, 0, 0, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 0, 0, 0, 0, 1, 80, 22, 1, 112,
            24, 1, 144, 26, 0, 0, 0, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 0, 1, 80, 22, 1, 112,
            24, 1, 144, 26, 1, 176, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 0, 0, 1, 80, 22, 1, 112,
            24, 1, 144, 26, 1, 176, 0, 0,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 18, 1, 80, 22, 1, 112,
            24, 1, 144, 26, 1, 176, 28,
        ],
        vec![
            0, 176, 12, 0, 208, 14, 0, 240, 16, 1, 16, 18, 0, 1, 80, 22, 1,
            112, 24, 1, 144, 26, 1, 176, 28, 0,
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
                let bit_order = BitOrder::MSB;
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
        vec![0, 0, 0, 11, 0, 0, 0, 12],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0,
        ],
        vec![0, 0, 0, 11, 0, 0, 0, 21],
        vec![0, 0, 0, 11, 0, 0, 0, 0, 21, 0],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 21, 0, 0, 0, 22],
        vec![0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 21, 0, 0, 0, 22, 0,
            0, 0, 23,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 21, 0, 0, 0, 22,
            0, 0, 0, 23, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 21, 0,
            0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 21,
            0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0,
            0, 25, 0, 0, 0, 26,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0,
            0, 0, 25, 0, 0, 0, 26, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0,
            0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0,
            0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0,
            0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28,
        ],
        vec![
            0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0,
            0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0,
            0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0,
            0, 28, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}
