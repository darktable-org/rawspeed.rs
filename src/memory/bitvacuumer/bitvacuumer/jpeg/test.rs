use std::io::Write as _;

use super::*;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let mut buf: Cursor<_> = Cursor::new(vec![]);
    let _vac = BitVacuumerJPEG::new(&mut buf);
}

#[test]
fn arr_ctor_test() {
    use std::io::Cursor;
    let mut buf = [0u8; 1024];
    let mut buf = Cursor::new(buf.as_mut());
    let _vac = BitVacuumerJPEG::new(&mut buf);
}

#[test]
fn drop_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerJPEG::new(&mut buf);
    drop(vac);
    buf.flush()?;
    assert!(&buf.get_ref().is_empty());
    Ok(())
}

#[test]
fn flush_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerJPEG::new(&mut buf);
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
    let mut vac = BitVacuumerJPEG::new(&mut buf);
    vac.put(0, 1).expect("unexpected panic");
    drop(vac);
}

#[test]
fn flush_arr_overflow_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = [0u8; 0];
    let mut buf = Cursor::new(buf.as_mut());
    let mut vac = BitVacuumerJPEG::new(&mut buf);
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
        let mut vac = BitVacuumerJPEG::new(&mut buf);
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
        let mut vac = BitVacuumerJPEG::new(&mut buf);
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn bit_enumeration_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_zeros in 0..32 {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerJPEG::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(1, 1)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_zeros in 0..=(32 - 8) {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerJPEG::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(0xFF, 8)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![255, 0, 0, 0, 0],
        vec![127, 128, 0, 0],
        vec![63, 192, 0, 0],
        vec![31, 224, 0, 0],
        vec![15, 240, 0, 0],
        vec![7, 248, 0, 0],
        vec![3, 252, 0, 0],
        vec![1, 254, 0, 0],
        vec![0, 255, 0, 0, 0],
        vec![0, 127, 128, 0],
        vec![0, 63, 192, 0],
        vec![0, 31, 224, 0],
        vec![0, 15, 240, 0],
        vec![0, 7, 248, 0],
        vec![0, 3, 252, 0],
        vec![0, 1, 254, 0],
        vec![0, 0, 255, 0, 0],
        vec![0, 0, 127, 128],
        vec![0, 0, 63, 192],
        vec![0, 0, 31, 224],
        vec![0, 0, 15, 240],
        vec![0, 0, 7, 248],
        vec![0, 0, 3, 252],
        vec![0, 0, 1, 254],
        vec![0, 0, 0, 255, 0],
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
        let mut vac = BitVacuumerJPEG::new(&mut buf);
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
        vec![255, 0, 0, 0, 0],
        vec![31, 240, 0, 0],
        vec![18, 255, 0, 0, 0],
        vec![18, 63, 240, 0],
        vec![18, 52, 255, 0, 0],
        vec![18, 52, 95, 240],
        vec![18, 52, 86, 255, 0],
        vec![18, 52, 86, 127, 240, 0, 0, 0],
        vec![18, 52, 86, 120, 255, 0, 0, 0, 0],
        vec![18, 52, 86, 120, 159, 240, 0, 0],
        vec![18, 52, 86, 120, 154, 255, 0, 0, 0],
        vec![18, 52, 86, 120, 154, 191, 240, 0],
        vec![18, 52, 86, 120, 154, 188, 255, 0, 0],
        vec![18, 52, 86, 120, 154, 188, 223, 240],
        vec![18, 52, 86, 120, 154, 188, 222, 255, 0],
        vec![18, 52, 86, 120, 154, 188, 222, 255, 0, 240, 0, 0, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_through_enumerated_bytes_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    let bytes = (0x1u8..16).collect::<Vec<u8>>();
    for oxff_pos in 0..=bytes.len() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerJPEG::new(&mut buf);
        let (left, right) = bytes.split_at(oxff_pos);
        for byte in left {
            vac.put((*byte).into(), 8)?;
        }
        vac.put(0xFF, 8)?;
        for byte in right {
            vac.put((*byte).into(), 8)?;
        }
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 255, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 255, 0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 255, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 255, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 255, 0, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 255, 0, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 255, 0, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 255, 0, 9, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 0, 10, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255, 0, 11, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 0, 12, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 255, 0, 13, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 255, 0, 14, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 255, 0, 15],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 255, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_through_enumerated_nibbles_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    let nibbles = (0x1u8..0xF).collect::<Vec<u8>>();
    for oxff_pos in 0..=nibbles.len() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerJPEG::new(&mut buf);
        let (left, right) = nibbles.split_at(oxff_pos);
        for nibble in left {
            assert!(*nibble <= 0xF);
            vac.put((*nibble).into(), 4)?;
        }
        vac.put(0xFF, 8)?;
        for nibble in right {
            assert!(*nibble <= 0xF);
            vac.put((*nibble).into(), 4)?;
        }
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![255, 0, 18, 52, 86, 120, 154, 188, 222],
        vec![31, 242, 52, 86, 120, 154, 188, 222],
        vec![18, 255, 0, 52, 86, 120, 154, 188, 222],
        vec![18, 63, 244, 86, 120, 154, 188, 222],
        vec![18, 52, 255, 0, 86, 120, 154, 188, 222],
        vec![18, 52, 95, 246, 120, 154, 188, 222],
        vec![18, 52, 86, 255, 0, 120, 154, 188, 222],
        vec![18, 52, 86, 127, 248, 154, 188, 222],
        vec![18, 52, 86, 120, 255, 0, 154, 188, 222],
        vec![18, 52, 86, 120, 159, 250, 188, 222],
        vec![18, 52, 86, 120, 154, 255, 0, 188, 222],
        vec![18, 52, 86, 120, 154, 191, 252, 222],
        vec![18, 52, 86, 120, 154, 188, 255, 0, 222],
        vec![18, 52, 86, 120, 154, 188, 223, 254],
        vec![18, 52, 86, 120, 154, 188, 222, 255, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}
