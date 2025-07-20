use crate::packed_decoder::Unpacker;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use rawspeed_std_ndslice::array2dref::Array2DRef;
use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

#[test]
fn u8_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::MSB32;
    type T = u8;
    const NUM_BITS: usize = 8;
    let inputs: Vec<Vec<u8>> = vec![
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

    let mut inputs_ = &*inputs;
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            for num_padding_bytes in 0..=1 {
                let (input, remaining) = inputs_.split_first().unwrap();
                inputs_ = remaining;

                assert!(input.len().is_multiple_of(num_rows));
                let pitch = RowPitch::new(input.len() / num_rows);
                let bytes = Array2DRef::new(
                    input,
                    RowLength::new(*pitch - num_padding_bytes),
                    pitch,
                );

                let mut storage: Vec<T> = vec![0; num_rows * num_cols];
                let mut img = Array2DRefMut::new(
                    &mut storage,
                    RowLength::new(num_cols),
                    RowPitch::new(num_cols),
                );

                let unpacker =
                    Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
                unpacker.unpack();

                for row in 1..=num_rows {
                    for col in 1..=num_cols {
                        assert_eq!(
                            img[Coord2D::new(
                                RowIndex::new(row - 1),
                                ColIndex::new(col - 1)
                            )],
                            (10 * row + col).try_into().unwrap()
                        );
                    }
                }
            }
        }
    }
    assert!(inputs_.is_empty());
}

#[test]
fn u4_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::MSB32;
    type T = u8;
    const NUM_BITS: usize = 4;
    let inputs: Vec<Vec<u8>> = vec![
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

    let mut inputs_ = &*inputs;
    for num_rows in 1..=2 {
        for num_cols in 1..=7 {
            for num_padding_bytes in 0..=1 {
                let (input, remaining) = inputs_.split_first().unwrap();
                inputs_ = remaining;

                assert!(input.len().is_multiple_of(num_rows));
                let pitch = RowPitch::new(input.len() / num_rows);
                let bytes = Array2DRef::new(
                    input,
                    RowLength::new(*pitch - num_padding_bytes),
                    pitch,
                );

                let mut storage: Vec<T> = vec![0; num_rows * num_cols];
                let mut img = Array2DRefMut::new(
                    &mut storage,
                    RowLength::new(num_cols),
                    RowPitch::new(num_cols),
                );

                let unpacker =
                    Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
                unpacker.unpack();

                for row in 0..num_rows {
                    for col in 0..num_cols {
                        assert_eq!(
                            img[Coord2D::new(
                                RowIndex::new(row),
                                ColIndex::new(col)
                            )],
                            (1 + num_cols * row + col).try_into().unwrap()
                        );
                    }
                }
            }
        }
    }
    assert!(inputs_.is_empty());
}

#[test]
fn u16_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::MSB32;
    type T = u16;
    const NUM_BITS: usize = 16;
    let inputs: Vec<Vec<u8>> = vec![
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

    let mut inputs_ = &*inputs;
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            for num_padding_bytes in 0..=1 {
                let (input, remaining) = inputs_.split_first().unwrap();
                inputs_ = remaining;

                assert!(input.len().is_multiple_of(num_rows));
                let pitch = RowPitch::new(input.len() / num_rows);
                let bytes = Array2DRef::new(
                    input,
                    RowLength::new(*pitch - num_padding_bytes),
                    pitch,
                );

                let mut storage: Vec<T> = vec![0; num_rows * num_cols];
                let mut img = Array2DRefMut::new(
                    &mut storage,
                    RowLength::new(num_cols),
                    RowPitch::new(num_cols),
                );

                let unpacker =
                    Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
                unpacker.unpack();

                for row in 1..=num_rows {
                    for col in 1..=num_cols {
                        assert_eq!(
                            img[Coord2D::new(
                                RowIndex::new(row - 1),
                                ColIndex::new(col - 1)
                            )],
                            (10 * row + col).try_into().unwrap()
                        );
                    }
                }
            }
        }
    }
    assert!(inputs_.is_empty());
}

#[test]
fn u12_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::MSB32;
    type T = u16;
    const NUM_BITS: usize = 12;
    let inputs: Vec<Vec<u8>> = vec![
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

    let mut inputs_ = &*inputs;
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            for num_padding_bytes in 0..=1 {
                let (input, remaining) = inputs_.split_first().unwrap();
                inputs_ = remaining;

                assert!(input.len().is_multiple_of(num_rows));
                let pitch = RowPitch::new(input.len() / num_rows);
                let bytes = Array2DRef::new(
                    input,
                    RowLength::new(*pitch - num_padding_bytes),
                    pitch,
                );

                let mut storage: Vec<T> = vec![0; num_rows * num_cols];
                let mut img = Array2DRefMut::new(
                    &mut storage,
                    RowLength::new(num_cols),
                    RowPitch::new(num_cols),
                );

                let unpacker =
                    Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
                unpacker.unpack();

                for row in 1..=num_rows {
                    for col in 1..=num_cols {
                        assert_eq!(
                            img[Coord2D::new(
                                RowIndex::new(row - 1),
                                ColIndex::new(col - 1)
                            )],
                            (10 * row + col).try_into().unwrap()
                        );
                    }
                }
            }
        }
    }
    assert!(inputs_.is_empty());
}

#[test]
#[expect(clippy::too_many_lines)]
fn u32_enumeration_test() {
    const BIT_ORDER: BitOrder = BitOrder::MSB32;
    type T = u32;
    const NUM_BITS: usize = 32;
    let inputs: Vec<Vec<u8>> = vec![
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

    let mut inputs_ = &*inputs;
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            for num_padding_bytes in 0..=1 {
                let (input, remaining) = inputs_.split_first().unwrap();
                inputs_ = remaining;

                assert!(input.len().is_multiple_of(num_rows));
                let pitch = RowPitch::new(input.len() / num_rows);
                let bytes = Array2DRef::new(
                    input,
                    RowLength::new(*pitch - num_padding_bytes),
                    pitch,
                );

                let mut storage: Vec<T> = vec![0; num_rows * num_cols];
                let mut img = Array2DRefMut::new(
                    &mut storage,
                    RowLength::new(num_cols),
                    RowPitch::new(num_cols),
                );

                let unpacker =
                    Unpacker::new(bytes, BIT_ORDER, NUM_BITS, &mut img);
                unpacker.unpack();

                for row in 1..=num_rows {
                    for col in 1..=num_cols {
                        assert_eq!(
                            img[Coord2D::new(
                                RowIndex::new(row - 1),
                                ColIndex::new(col - 1)
                            )],
                            (10 * row + col).try_into().unwrap()
                        );
                    }
                }
            }
        }
    }
    assert!(inputs_.is_empty());
}
