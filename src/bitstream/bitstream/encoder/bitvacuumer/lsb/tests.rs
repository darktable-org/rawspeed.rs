use super::*;

use std::io::Write as _;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let _vac = BitVacuumerLSB::new(&mut buf);
}

#[test]
fn arr_ctor_test() {
    use std::io::Cursor;
    let mut buf = [0_u8; 1024];
    let mut buf = Cursor::new(buf.as_mut());
    let _vac = BitVacuumerLSB::new(&mut buf);
}

#[test]
fn drop_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerLSB::new(&mut buf);
    drop(vac);
    buf.flush()?;
    assert!(&buf.get_ref().is_empty());
    Ok(())
}

#[test]
fn flush_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerLSB::new(&mut buf);
    vac.flush()?;
    buf.flush()?;
    assert!(&buf.get_ref().is_empty());
    Ok(())
}

#[test]
#[should_panic(
    expected = "Unrecoverable Error: trying to drop non-empty BitVacuumer. Did you forget to call `flush()`?"
)]
fn dropping_unflushed_vac_byte() {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let mut vac = BitVacuumerLSB::new(&mut buf);
    vac.put(0, 1).unwrap();
    drop(vac);
}

#[test]
fn flush_arr_overflow_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = [0_u8; 0];
    let mut buf = Cursor::new(buf.as_mut());
    let mut vac = BitVacuumerLSB::new(&mut buf);
    vac.put(0, 1)?;
    assert!(vac.flush().is_err());
    Ok(())
}

#[test]
fn byte_enumeration_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_bytes in 0..=8 {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        for i in 0..num_bytes {
            vac.put(1 + i, 8)?;
        }
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn nibble_enumeration_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_nibbles in 0..16 {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        for i in 0..num_nibbles {
            let nibble = 1 + i;
            assert!(nibble <= 0xF);
            vac.put(nibble, 4)?;
        }
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn bit_enumeration_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_zeros in 0..32 {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(1, 1)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_zeros in 0..=(32 - 8) {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(0xFF, 8)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_prefixed_by_enumerated_nibbles_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_nibbles in 0..16 {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerLSB::new(&mut buf);
        for i in 0..num_leading_nibbles {
            let nibble = 1 + i;
            assert!(nibble <= 0xF);
            vac.put(nibble, 4)?;
        }
        vac.put(0xFF, 8)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}
