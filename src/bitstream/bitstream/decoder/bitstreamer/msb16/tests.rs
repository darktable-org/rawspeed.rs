use core::assert_eq;

use super::*;

#[test]
fn byte_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: u32 = 8;
    let inputs: Vec<Vec<u8>> = vec![
        vec![0, 1, 0, 0],
        vec![2, 1, 0, 0],
        vec![2, 1, 0, 3],
        vec![2, 1, 4, 3],
        vec![2, 1, 4, 3, 0, 5, 0, 0],
        vec![2, 1, 4, 3, 6, 5, 0, 0],
        vec![2, 1, 4, 3, 6, 5, 0, 7],
        vec![2, 1, 4, 3, 6, 5, 8, 7],
    ];
    for (num_bytes, input) in inputs.iter().enumerate() {
        let mut bs =
            BitStreamerMSB16::new(input.as_slice().try_into().unwrap());
        for i in 0..=num_bytes {
            bs.fill(NUM_BITS)?;
            assert_eq!(
                bs.peek_bits_no_fill(NUM_BITS),
                (1 + i).try_into().unwrap()
            );
            bs.skip_bits_no_fill(NUM_BITS);
        }
        while let Ok(()) = bs.fill(1) {
            assert_eq!(bs.peek_bits_no_fill(1), 0);
            bs.skip_bits_no_fill(1);
        }
    }
    Ok(())
}

#[test]
fn nibble_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: u32 = 4;
    let inputs: Vec<Vec<u8>> = vec![
        vec![0, 16, 0, 0],
        vec![0, 18, 0, 0],
        vec![48, 18, 0, 0],
        vec![52, 18, 0, 0],
        vec![52, 18, 0, 80],
        vec![52, 18, 0, 86],
        vec![52, 18, 112, 86],
        vec![52, 18, 120, 86],
        vec![52, 18, 120, 86, 0, 144, 0, 0],
        vec![52, 18, 120, 86, 0, 154, 0, 0],
        vec![52, 18, 120, 86, 176, 154, 0, 0],
        vec![52, 18, 120, 86, 188, 154, 0, 0],
        vec![52, 18, 120, 86, 188, 154, 0, 208],
        vec![52, 18, 120, 86, 188, 154, 0, 222],
        vec![52, 18, 120, 86, 188, 154, 240, 222],
    ];
    for (num_nibbles, input) in inputs.iter().enumerate() {
        let mut bs =
            BitStreamerMSB16::new(input.as_slice().try_into().unwrap());
        for i in 0..=num_nibbles {
            bs.fill(NUM_BITS)?;
            assert_eq!(
                bs.peek_bits_no_fill(NUM_BITS),
                (1 + i).try_into().unwrap()
            );
            bs.skip_bits_no_fill(NUM_BITS);
        }
        while let Ok(()) = bs.fill(1) {
            assert_eq!(bs.peek_bits_no_fill(1), 0);
            bs.skip_bits_no_fill(1);
        }
    }
    Ok(())
}

#[test]
fn bit_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: u32 = 1;
    let inputs: Vec<Vec<u8>> = vec![
        vec![0, 128, 0, 0],
        vec![0, 64, 0, 0],
        vec![0, 32, 0, 0],
        vec![0, 16, 0, 0],
        vec![0, 8, 0, 0],
        vec![0, 4, 0, 0],
        vec![0, 2, 0, 0],
        vec![0, 1, 0, 0],
        vec![128, 0, 0, 0],
        vec![64, 0, 0, 0],
        vec![32, 0, 0, 0],
        vec![16, 0, 0, 0],
        vec![8, 0, 0, 0],
        vec![4, 0, 0, 0],
        vec![2, 0, 0, 0],
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 128],
        vec![0, 0, 0, 64],
        vec![0, 0, 0, 32],
        vec![0, 0, 0, 16],
        vec![0, 0, 0, 8],
        vec![0, 0, 0, 4],
        vec![0, 0, 0, 2],
        vec![0, 0, 0, 1],
        vec![0, 0, 128, 0],
        vec![0, 0, 64, 0],
        vec![0, 0, 32, 0],
        vec![0, 0, 16, 0],
        vec![0, 0, 8, 0],
        vec![0, 0, 4, 0],
        vec![0, 0, 2, 0],
        vec![0, 0, 1, 0],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs =
            BitStreamerMSB16::new(input.as_slice().try_into().unwrap());
        for _i in 0..num_leading_zeros {
            bs.fill(NUM_BITS)?;
            assert_eq!(bs.peek_bits_no_fill(NUM_BITS), 0);
            bs.skip_bits_no_fill(NUM_BITS);
        }
        bs.fill(1)?;
        assert_eq!(bs.peek_bits_no_fill(1), 1);
        bs.skip_bits_no_fill(1);
        while let Ok(()) = bs.fill(1) {
            assert_eq!(bs.peek_bits_no_fill(1), 0);
            bs.skip_bits_no_fill(1);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_test() -> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: u32 = 1;
    let inputs: Vec<Vec<u8>> = vec![
        vec![0, 255, 0, 0],
        vec![128, 127, 0, 0],
        vec![192, 63, 0, 0],
        vec![224, 31, 0, 0],
        vec![240, 15, 0, 0],
        vec![248, 7, 0, 0],
        vec![252, 3, 0, 0],
        vec![254, 1, 0, 0],
        vec![255, 0, 0, 0],
        vec![127, 0, 0, 128],
        vec![63, 0, 0, 192],
        vec![31, 0, 0, 224],
        vec![15, 0, 0, 240],
        vec![7, 0, 0, 248],
        vec![3, 0, 0, 252],
        vec![1, 0, 0, 254],
        vec![0, 0, 0, 255],
        vec![0, 0, 128, 127],
        vec![0, 0, 192, 63],
        vec![0, 0, 224, 31],
        vec![0, 0, 240, 15],
        vec![0, 0, 248, 7],
        vec![0, 0, 252, 3],
        vec![0, 0, 254, 1],
        vec![0, 0, 255, 0],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs =
            BitStreamerMSB16::new(input.as_slice().try_into().unwrap());
        for _i in 0..num_leading_zeros {
            bs.fill(NUM_BITS)?;
            assert_eq!(bs.peek_bits_no_fill(NUM_BITS), 0);
            bs.skip_bits_no_fill(NUM_BITS);
        }
        bs.fill(8)?;
        assert_eq!(bs.peek_bits_no_fill(8), 0xFF);
        bs.skip_bits_no_fill(8);
        while let Ok(()) = bs.fill(1) {
            assert_eq!(bs.peek_bits_no_fill(1), 0);
            bs.skip_bits_no_fill(1);
        }
    }
    Ok(())
}

#[test]
fn sliding_0xff_prefixed_by_enumerated_nibbles_test()
-> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: u32 = 4;
    let inputs: Vec<Vec<u8>> = vec![
        vec![0, 255, 0, 0],
        vec![240, 31, 0, 0],
        vec![255, 18, 0, 0],
        vec![63, 18, 0, 240],
        vec![52, 18, 0, 255],
        vec![52, 18, 240, 95],
        vec![52, 18, 255, 86],
        vec![52, 18, 127, 86, 0, 240, 0, 0],
        vec![52, 18, 120, 86, 0, 255, 0, 0],
        vec![52, 18, 120, 86, 240, 159, 0, 0],
        vec![52, 18, 120, 86, 255, 154, 0, 0],
        vec![52, 18, 120, 86, 191, 154, 0, 240],
        vec![52, 18, 120, 86, 188, 154, 0, 255],
        vec![52, 18, 120, 86, 188, 154, 240, 223],
        vec![52, 18, 120, 86, 188, 154, 255, 222],
        vec![52, 18, 120, 86, 188, 154, 255, 222, 0, 240, 0, 0],
    ];
    for (num_leading_nibbles, input) in inputs.iter().enumerate() {
        let mut bs =
            BitStreamerMSB16::new(input.as_slice().try_into().unwrap());
        for i in 0..num_leading_nibbles {
            bs.fill(NUM_BITS)?;
            assert_eq!(
                bs.peek_bits_no_fill(NUM_BITS),
                (1 + i).try_into().unwrap()
            );
            bs.skip_bits_no_fill(NUM_BITS);
        }
        bs.fill(8)?;
        assert_eq!(bs.peek_bits_no_fill(8), 0xFF);
        bs.skip_bits_no_fill(8);
        while let Ok(()) = bs.fill(1) {
            assert_eq!(bs.peek_bits_no_fill(1), 0);
            bs.skip_bits_no_fill(1);
        }
    }
    Ok(())
}
