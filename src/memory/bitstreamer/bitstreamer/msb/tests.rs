use core::assert_eq;

use super::*;

#[test]
fn byte_enumeration_test() -> Result<(), Box<dyn core::error::Error>> {
    const NUM_BITS: usize = 8;
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
        let mut bs = BitStreamerMSB::new(input);
        for i in 0..num_bytes {
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
    const NUM_BITS: usize = 4;
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
        let mut bs = BitStreamerMSB::new(input);
        for i in 0..num_nibbles {
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
    const NUM_BITS: usize = 1;
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
        let mut bs = BitStreamerMSB::new(input);
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
    const NUM_BITS: usize = 1;
    let inputs: Vec<Vec<u8>> = vec![
        vec![255, 0, 0, 0],
        vec![127, 128, 0, 0],
        vec![63, 192, 0, 0],
        vec![31, 224, 0, 0],
        vec![15, 240, 0, 0],
        vec![7, 248, 0, 0],
        vec![3, 252, 0, 0],
        vec![1, 254, 0, 0],
        vec![0, 255, 0, 0],
        vec![0, 127, 128, 0],
        vec![0, 63, 192, 0],
        vec![0, 31, 224, 0],
        vec![0, 15, 240, 0],
        vec![0, 7, 248, 0],
        vec![0, 3, 252, 0],
        vec![0, 1, 254, 0],
        vec![0, 0, 255, 0],
        vec![0, 0, 127, 128],
        vec![0, 0, 63, 192],
        vec![0, 0, 31, 224],
        vec![0, 0, 15, 240],
        vec![0, 0, 7, 248],
        vec![0, 0, 3, 252],
        vec![0, 0, 1, 254],
        vec![0, 0, 0, 255],
    ];
    for (num_leading_zeros, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerMSB::new(input);
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
    const NUM_BITS: usize = 4;
    let inputs: Vec<Vec<u8>> = vec![
        vec![255, 0, 0, 0],
        vec![31, 240, 0, 0],
        vec![18, 255, 0, 0],
        vec![18, 63, 240, 0],
        vec![18, 52, 255, 0],
        vec![18, 52, 95, 240],
        vec![18, 52, 86, 255],
        vec![18, 52, 86, 127, 240, 0, 0, 0],
        vec![18, 52, 86, 120, 255, 0, 0, 0],
        vec![18, 52, 86, 120, 159, 240, 0, 0],
        vec![18, 52, 86, 120, 154, 255, 0, 0],
        vec![18, 52, 86, 120, 154, 191, 240, 0],
        vec![18, 52, 86, 120, 154, 188, 255, 0],
        vec![18, 52, 86, 120, 154, 188, 223, 240],
        vec![18, 52, 86, 120, 154, 188, 222, 255],
        vec![18, 52, 86, 120, 154, 188, 222, 255, 240, 0, 0, 0],
    ];
    for (num_leading_nibbles, input) in inputs.iter().enumerate() {
        let mut bs = BitStreamerMSB::new(input);
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
