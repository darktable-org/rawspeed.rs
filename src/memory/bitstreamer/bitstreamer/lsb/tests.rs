use core::assert_eq;

use super::*;

#[test]
fn byte_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
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
        let mut bs = BitStreamerLSB::new(input);
        for i in 0..num_bytes {
            assert_eq!(bs.get_bits(8)?, (1 + i).try_into().unwrap());
        }
    }
    Ok(())
}

#[test]
fn nibble_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![],
        vec![1, 0, 0, 0],
        vec![33, 0, 0, 0],
        vec![33, 3, 0, 0],
        vec![33, 67, 0, 0],
        vec![33, 67, 5, 0],
        vec![33, 67, 101, 0],
        vec![33, 67, 101, 7],
        vec![33, 67, 101, 135],
        vec![33, 67, 101, 135, 9, 0, 0, 0],
        vec![33, 67, 101, 135, 169, 0, 0, 0],
        vec![33, 67, 101, 135, 169, 11, 0, 0],
        vec![33, 67, 101, 135, 169, 203, 0, 0],
        vec![33, 67, 101, 135, 169, 203, 13, 0],
        vec![33, 67, 101, 135, 169, 203, 237, 0],
        vec![33, 67, 101, 135, 169, 203, 237, 15],
    ];
    for (num_nibbles, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerLSB::new(input);
        for i in 0..num_nibbles {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
    }
    Ok(())
}

#[test]
fn bit_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![1, 0, 0, 0],
        vec![2, 0, 0, 0],
        vec![4, 0, 0, 0],
        vec![8, 0, 0, 0],
        vec![16, 0, 0, 0],
        vec![32, 0, 0, 0],
        vec![64, 0, 0, 0],
        vec![128, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 2, 0, 0],
        vec![0, 4, 0, 0],
        vec![0, 8, 0, 0],
        vec![0, 16, 0, 0],
        vec![0, 32, 0, 0],
        vec![0, 64, 0, 0],
        vec![0, 128, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 2, 0],
        vec![0, 0, 4, 0],
        vec![0, 0, 8, 0],
        vec![0, 0, 16, 0],
        vec![0, 0, 32, 0],
        vec![0, 0, 64, 0],
        vec![0, 0, 128, 0],
        vec![0, 0, 0, 1],
        vec![0, 0, 0, 2],
        vec![0, 0, 0, 4],
        vec![0, 0, 0, 8],
        vec![0, 0, 0, 16],
        vec![0, 0, 0, 32],
        vec![0, 0, 0, 64],
        vec![0, 0, 0, 128],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerLSB::new(input);
        for _i in 0..num_leading_zeros {
            assert_eq!(bs.get_bits(1)?, 0);
        }
        assert_eq!(bs.get_bits(1)?, 1);
    }
    Ok(())
}

#[test]
fn sliding_0xff_test() -> Result<(), Box<dyn core::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![255, 0, 0, 0],
        vec![254, 1, 0, 0],
        vec![252, 3, 0, 0],
        vec![248, 7, 0, 0],
        vec![240, 15, 0, 0],
        vec![224, 31, 0, 0],
        vec![192, 63, 0, 0],
        vec![128, 127, 0, 0],
        vec![0, 255, 0, 0],
        vec![0, 254, 1, 0],
        vec![0, 252, 3, 0],
        vec![0, 248, 7, 0],
        vec![0, 240, 15, 0],
        vec![0, 224, 31, 0],
        vec![0, 192, 63, 0],
        vec![0, 128, 127, 0],
        vec![0, 0, 255, 0],
        vec![0, 0, 254, 1],
        vec![0, 0, 252, 3],
        vec![0, 0, 248, 7],
        vec![0, 0, 240, 15],
        vec![0, 0, 224, 31],
        vec![0, 0, 192, 63],
        vec![0, 0, 128, 127],
        vec![0, 0, 0, 255],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerLSB::new(input);
        for _i in 0..num_leading_zeros {
            assert_eq!(bs.get_bits(1)?, 0);
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
    }
    Ok(())
}

#[test]
fn sliding_0xff_prefixed_by_enumerated_nibbles_test()
-> Result<(), Box<dyn core::error::Error>> {
    let inputs: Vec<Vec<u8>> = vec![
        vec![255, 0, 0, 0],
        vec![241, 15, 0, 0],
        vec![33, 255, 0, 0],
        vec![33, 243, 15, 0],
        vec![33, 67, 255, 0],
        vec![33, 67, 245, 15],
        vec![33, 67, 101, 255],
        vec![33, 67, 101, 247, 15, 0, 0, 0],
        vec![33, 67, 101, 135, 255, 0, 0, 0],
        vec![33, 67, 101, 135, 249, 15, 0, 0],
        vec![33, 67, 101, 135, 169, 255, 0, 0],
        vec![33, 67, 101, 135, 169, 251, 15, 0],
        vec![33, 67, 101, 135, 169, 203, 255, 0],
        vec![33, 67, 101, 135, 169, 203, 253, 15],
        vec![33, 67, 101, 135, 169, 203, 237, 255],
        vec![33, 67, 101, 135, 169, 203, 237, 255, 15, 0, 0, 0],
    ];
    for (num_leading_nibbles, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerLSB::new(input);
        for i in 0..num_leading_nibbles {
            assert_eq!(bs.get_bits(4)?, (1 + i).try_into().unwrap());
        }
        assert_eq!(bs.get_bits(8)?, 0xFF);
    }
    Ok(())
}
