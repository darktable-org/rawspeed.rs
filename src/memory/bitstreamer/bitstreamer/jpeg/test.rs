use core::assert_eq;

use super::*;

#[test]
fn byte_enumeration_test() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![],
        vec![1, 0, 0, 0],
        vec![1, 2, 0, 0],
        vec![1, 2, 3, 0],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5, 0, 0, 0],
        vec![1, 2, 3, 4, 5, 6, 0, 0],
        vec![1, 2, 3, 4, 5, 6, 7, 0],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    ];
    for (num_bytes, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..num_bytes {
            assert_eq!(bs.get_bits(8)?, (1 + i).try_into().unwrap());
        }
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn nibble_enumeration_test() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![],
        vec![16, 0, 0, 0],
        vec![18, 0, 0, 0],
        vec![18, 48, 0, 0],
        vec![18, 52, 0, 0],
        vec![18, 52, 80, 0],
        vec![18, 52, 86, 0],
        vec![18, 52, 86, 112],
        vec![18, 52, 86, 120],
        vec![18, 52, 86, 120, 144, 0, 0, 0],
        vec![18, 52, 86, 120, 154, 0, 0, 0],
        vec![18, 52, 86, 120, 154, 176, 0, 0],
        vec![18, 52, 86, 120, 154, 188, 0, 0],
        vec![18, 52, 86, 120, 154, 188, 208, 0],
        vec![18, 52, 86, 120, 154, 188, 222, 0],
        vec![18, 52, 86, 120, 154, 188, 222, 240],
    ];
    for (num_nibbles, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..num_nibbles {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn bit_enumeration_test() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![128, 0, 0, 0],
        vec![64, 0, 0, 0],
        vec![32, 0, 0, 0],
        vec![16, 0, 0, 0],
        vec![8, 0, 0, 0],
        vec![4, 0, 0, 0],
        vec![2, 0, 0, 0],
        vec![1, 0, 0, 0],
        vec![0, 128, 0, 0],
        vec![0, 64, 0, 0],
        vec![0, 32, 0, 0],
        vec![0, 16, 0, 0],
        vec![0, 8, 0, 0],
        vec![0, 4, 0, 0],
        vec![0, 2, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 128, 0],
        vec![0, 0, 64, 0],
        vec![0, 0, 32, 0],
        vec![0, 0, 16, 0],
        vec![0, 0, 8, 0],
        vec![0, 0, 4, 0],
        vec![0, 0, 2, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 128],
        vec![0, 0, 0, 64],
        vec![0, 0, 0, 32],
        vec![0, 0, 0, 16],
        vec![0, 0, 0, 8],
        vec![0, 0, 0, 4],
        vec![0, 0, 0, 2],
        vec![0, 0, 0, 1],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for _i in 0..num_leading_zeros {
            assert_eq!(bs.get_bits(1)?, 0);
        }
        assert_eq!(bs.get_bits(1)?, 1);
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_test() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![0xFF, 0x00, 0, 0, 0],
        vec![127, 128, 0, 0],
        vec![63, 192, 0, 0],
        vec![31, 224, 0, 0],
        vec![15, 240, 0, 0],
        vec![7, 248, 0, 0],
        vec![3, 252, 0, 0],
        vec![1, 254, 0, 0],
        vec![0, 0xFF, 0x00, 0, 0],
        vec![0, 127, 128, 0],
        vec![0, 63, 192, 0],
        vec![0, 31, 224, 0],
        vec![0, 15, 240, 0],
        vec![0, 7, 248, 0],
        vec![0, 3, 252, 0],
        vec![0, 1, 254, 0],
        vec![0, 0, 0xFF, 0x00, 0],
        vec![0, 0, 127, 128],
        vec![0, 0, 63, 192],
        vec![0, 0, 31, 224],
        vec![0, 0, 15, 240],
        vec![0, 0, 7, 248],
        vec![0, 0, 3, 252],
        vec![0, 0, 1, 254],
        vec![0, 0, 0, 0xFF, 0x00],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for _i in 0..num_leading_zeros {
            assert_eq!(bs.get_bits(1)?, 0);
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_prefixed_by_enumerated_nibbles_test()
-> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![0xFF, 0x00, 0, 0, 0],
        vec![31, 240, 0, 0],
        vec![18, 0xFF, 0x00, 0, 0],
        vec![18, 63, 240, 0],
        vec![18, 52, 0xFF, 0x00, 0],
        vec![18, 52, 95, 240],
        vec![18, 52, 86, 0xFF, 0x00],
        vec![18, 52, 86, 127, 240, 0, 0, 0],
        vec![18, 52, 86, 120, 0xFF, 0x00, 0, 0, 0],
        vec![18, 52, 86, 120, 159, 240, 0, 0],
        vec![18, 52, 86, 120, 154, 0xFF, 0x00, 0, 0],
        vec![18, 52, 86, 120, 154, 191, 240, 0],
        vec![18, 52, 86, 120, 154, 188, 0xFF, 0x00, 0],
        vec![18, 52, 86, 120, 154, 188, 223, 240],
        vec![18, 52, 86, 120, 154, 188, 222, 0xFF, 0x00],
        vec![18, 52, 86, 120, 154, 188, 222, 0xFF, 0x00, 240, 0, 0, 0],
    ];
    for (num_leading_nibbles, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..num_leading_nibbles {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_through_enumerated_bytes_test()
-> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![
            0xFF, 0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 0xFF, 0x00, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 0xFF, 0x00, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 0xFF, 0x00, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 0xFF, 0x00, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 0xFF, 0x00, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 0xFF, 0x00, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 0xFF, 0x00, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 0xFF, 0x00, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0xFF, 0x00, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xFF, 0x00, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0xFF, 0x00, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xFF, 0x00, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0xFF, 0x00, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0xFF, 0x00, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0xFF, 0x00,
        ],
    ];
    for (oxff_pos, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..oxff_pos {
            assert_eq!(bs.get_bits(8)?, (1 + i).try_into().unwrap());
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
        for i in oxff_pos..15 {
            assert_eq!(bs.get_bits(8)?, (1 + i).try_into().unwrap());
        }
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_through_enumerated_nibbles_test()
-> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![0xFF, 0x00, 18, 52, 86, 120, 154, 188, 222],
        vec![31, 242, 52, 86, 120, 154, 188, 222],
        vec![18, 0xFF, 0x00, 52, 86, 120, 154, 188, 222],
        vec![18, 63, 244, 86, 120, 154, 188, 222],
        vec![18, 52, 0xFF, 0x00, 86, 120, 154, 188, 222],
        vec![18, 52, 95, 246, 120, 154, 188, 222],
        vec![18, 52, 86, 0xFF, 0x00, 120, 154, 188, 222],
        vec![18, 52, 86, 127, 248, 154, 188, 222],
        vec![18, 52, 86, 120, 0xFF, 0x00, 154, 188, 222],
        vec![18, 52, 86, 120, 159, 250, 188, 222],
        vec![18, 52, 86, 120, 154, 0xFF, 0x00, 188, 222],
        vec![18, 52, 86, 120, 154, 191, 252, 222],
        vec![18, 52, 86, 120, 154, 188, 0xFF, 0x00, 222],
        vec![18, 52, 86, 120, 154, 188, 223, 254],
        vec![18, 52, 86, 120, 154, 188, 222, 0xFF, 0x00],
    ];
    for (oxff_pos, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..oxff_pos {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
        for i in oxff_pos..14 {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_non_0x00_control_sequence_test()
-> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![
            0xFF, 0x01, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 0xFF, 0x01, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 0xFF, 0x01, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 0xFF, 0x01, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 0xFF, 0x01, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 0xFF, 0x01, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 0xFF, 0x01, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 0xFF, 0x01, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 0xFF, 0x01, 9, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0xFF, 0x01, 10, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xFF, 0x01, 11, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0xFF, 0x01, 12, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xFF, 0x01, 13, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0xFF, 0x01, 14, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0xFF, 0x01, 15,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0xFF, 0x01,
        ],
    ];
    for (oxff_pos, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerJPEG::new(input);
        for i in 0..oxff_pos {
            assert_eq!(bs.get_bits(8)?, (1 + i).try_into().unwrap());
        }
        while let Ok(bit) = bs.get_bits(1) {
            assert_eq!(bit, 0);
        }
    }
    Ok(())
}
