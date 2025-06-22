use super::*;

use core::result::Result::Ok;
use std::io::Write;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let mut buf: Cursor<_> = Cursor::new(vec![]);
    let _vac = BitVacuumerMSB16::new(&mut buf);
}

#[test]
fn arr_ctor_test() {
    use std::io::Cursor;
    let mut buf = [0u8; 1024];
    let mut buf = Cursor::new(buf.as_mut());
    let _vac = BitVacuumerMSB16::new(&mut buf);
}

#[test]
fn drop_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerMSB16::new(&mut buf);
    drop(vac);
    buf.flush()?;
    assert!(&buf.get_ref().is_empty());
    Ok(())
}

#[test]
fn flush_empty_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = Cursor::new(vec![]);
    let vac = BitVacuumerMSB16::new(&mut buf);
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
    let mut vac = BitVacuumerMSB16::new(&mut buf);
    vac.put(0, 1).expect("unexpected panic");
    drop(vac);
}

#[test]
fn flush_arr_overflow_test() -> std::io::Result<()> {
    use std::io::Cursor;
    let mut buf = [0u8; 0];
    let mut buf = Cursor::new(buf.as_mut());
    let mut vac = BitVacuumerMSB16::new(&mut buf);
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
        let mut vac = BitVacuumerMSB16::new(&mut buf);
        for i in 0..num_bytes {
            vac.put(1 + i, 8)?;
        }
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![],
        vec![0, 1, 0, 0],
        vec![2, 1, 0, 0],
        vec![2, 1, 0, 3],
        vec![2, 1, 4, 3],
        vec![2, 1, 4, 3, 0, 5, 0, 0],
        vec![2, 1, 4, 3, 6, 5, 0, 0],
        vec![2, 1, 4, 3, 6, 5, 0, 7],
        vec![2, 1, 4, 3, 6, 5, 8, 7],
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
        let mut vac = BitVacuumerMSB16::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(1, 1)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn sliding_0xff_test() -> std::io::Result<()> {
    let mut res: Vec<Vec<u8>> = vec![];
    for num_leading_zeros in 0..=(32 - 8) {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![]);
        let mut vac = BitVacuumerMSB16::new(&mut buf);
        for _i in 0..num_leading_zeros {
            vac.put(0, 1)?;
        }
        vac.put(0xFF, 8)?;
        vac.flush()?;
        buf.flush()?;
        res.push(buf.get_ref().clone());
    }
    let expected: Vec<Vec<u8>> = vec![
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
    assert_eq!(res, expected);
    Ok(())
}
