use super::*;

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn read_overflow_test() {
    let buf = [0_u8; 0];
    let mut bs = ByteStreamer::new(&buf, Endianness::Little);
    bs.read::<u8>();
}

#[test]
fn u8_enumeration_test() {
    type T = u8;
    let input: Vec<u8> = vec![0, 0, 255, 128, 127];
    let expected: Vec<T> =
        vec![0, u8::MIN, u8::MAX, i8::MIN as u8, i8::MAX as u8];
    for endianness in [Endianness::Little, Endianness::Big] {
        let mut vac = ByteStreamer::new(&input, endianness);
        for val in &expected {
            assert_eq!(vac.read::<T>(), *val);
        }
    }
}

#[test]
fn u16_enumeration_le_test() {
    type T = u16;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 255, 0, 128, 0, 127, 0, 0, 0, 255, 255, 0, 128, 255, 127,
    ];
    let expected: Vec<T> = vec![
        0,
        u16::from(u8::MIN),
        u16::from(u8::MAX),
        u16::from(i8::MIN as u8),
        u16::from(i8::MAX as u8),
        u16::MIN,
        u16::MAX,
        i16::MIN as u16,
        i16::MAX as u16,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn u16_enumeration_be_test() {
    type T = u16;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 255, 0, 128, 0, 127, 0, 0, 255, 255, 128, 0, 127, 255,
    ];
    let expected: Vec<T> = vec![
        0,
        u16::from(u8::MIN),
        u16::from(u8::MAX),
        u16::from(i8::MIN as u8),
        u16::from(i8::MAX as u8),
        u16::MIN,
        u16::MAX,
        i16::MIN as u16,
        i16::MAX as u16,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn u32_enumeration_le_test() {
    type T = u32;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 128, 0, 0, 0, 127, 0, 0, 0, 0, 0,
        0, 0, 255, 255, 0, 0, 0, 128, 0, 0, 255, 127, 0, 0, 0, 0, 0, 0, 255,
        255, 255, 255, 0, 0, 0, 128, 255, 255, 255, 127,
    ];
    let expected: Vec<T> = vec![
        0,
        u32::from(u8::MIN),
        u32::from(u8::MAX),
        u32::from(i8::MIN as u8),
        u32::from(i8::MAX as u8),
        u32::from(u16::MIN),
        u32::from(u16::MAX),
        u32::from(i16::MIN as u16),
        u32::from(i16::MAX as u16),
        u32::MIN,
        u32::MAX,
        i32::MIN as u32,
        i32::MAX as u32,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn u32_enumeration_be_test() {
    type T = u32;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 128, 0, 0, 0, 127, 0, 0,
        0, 0, 0, 0, 255, 255, 0, 0, 128, 0, 0, 0, 127, 255, 0, 0, 0, 0, 255,
        255, 255, 255, 128, 0, 0, 0, 127, 255, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0,
        u32::from(u8::MIN),
        u32::from(u8::MAX),
        u32::from(i8::MIN as u8),
        u32::from(i8::MAX as u8),
        u32::from(u16::MIN),
        u32::from(u16::MAX),
        u32::from(i16::MIN as u16),
        u32::from(i16::MAX as u16),
        u32::MIN,
        u32::MAX,
        i32::MIN as u32,
        i32::MAX as u32,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn u64_enumeration_le_test() {
    type T = u64;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0,
        0, 128, 0, 0, 0, 0, 0, 0, 0, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 255,
        127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 0,
        0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 255, 255, 255, 127, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
        0, 0, 0, 0, 128, 255, 255, 255, 255, 255, 255, 255, 127,
    ];
    let expected: Vec<T> = vec![
        0,
        u64::from(u8::MIN),
        u64::from(u8::MAX),
        u64::from(i8::MIN as u8),
        u64::from(i8::MAX as u8),
        u64::from(u16::MIN),
        u64::from(u16::MAX),
        u64::from(i16::MIN as u16),
        u64::from(i16::MAX as u16),
        u64::from(u32::MIN),
        u64::from(u32::MAX),
        u64::from(i32::MIN as u32),
        u64::from(i32::MAX as u32),
        u64::MIN,
        u64::MAX,
        i64::MIN as u64,
        i64::MAX as u64,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn u64_enumeration_be_test() {
    type T = u64;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 127, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0,
        0, 0, 0, 127, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255,
        255, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 127, 255, 255, 255, 0, 0, 0,
        0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0,
        0, 0, 0, 127, 255, 255, 255, 255, 255, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0,
        u64::from(u8::MIN),
        u64::from(u8::MAX),
        u64::from(i8::MIN as u8),
        u64::from(i8::MAX as u8),
        u64::from(u16::MIN),
        u64::from(u16::MAX),
        u64::from(i16::MIN as u16),
        u64::from(i16::MAX as u16),
        u64::from(u32::MIN),
        u64::from(u32::MAX),
        u64::from(i32::MIN as u32),
        u64::from(i32::MAX as u32),
        u64::MIN,
        u64::MAX,
        i64::MIN as u64,
        i64::MAX as u64,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i8_enumeration_test() {
    type T = i8;
    let input: Vec<u8> = vec![0, 128, 127];
    let expected: Vec<T> = vec![0, i8::MIN, i8::MAX];
    for endianness in [Endianness::Little, Endianness::Big] {
        let mut vac = ByteStreamer::new(&input, endianness);
        for val in &expected {
            assert_eq!(vac.read::<T>(), *val);
        }
    }
}

#[test]
fn i16_enumeration_le_test() {
    type T = i16;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> =
        vec![0, 0, 0, 0, 255, 0, 128, 255, 127, 0, 0, 128, 255, 127];
    let expected: Vec<T> = vec![
        0,
        i16::from(u8::MIN),
        i16::from(u8::MAX),
        i16::from(i8::MIN),
        i16::from(i8::MAX),
        i16::MIN,
        i16::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i16_enumeration_be_test() {
    type T = i16;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> =
        vec![0, 0, 0, 0, 0, 255, 255, 128, 0, 127, 128, 0, 127, 255];
    let expected: Vec<T> = vec![
        0,
        i16::from(u8::MIN),
        i16::from(u8::MAX),
        i16::from(i8::MIN),
        i16::from(i8::MAX),
        i16::MIN,
        i16::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i32_enumeration_le_test() {
    type T = i32;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0,
        0, 0, 0, 0, 255, 255, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 0, 0,
        128, 255, 255, 255, 127,
    ];
    let expected: Vec<T> = vec![
        0,
        i32::from(u8::MIN),
        i32::from(u8::MAX),
        i32::from(i8::MIN),
        i32::from(i8::MAX),
        i32::from(u16::MIN),
        i32::from(u16::MAX),
        i32::from(i16::MIN),
        i32::from(i16::MAX),
        i32::MIN,
        i32::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i32_enumeration_be_test() {
    type T = i32;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 128, 0, 0, 0, 127,
        0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 128, 0, 0, 0, 127, 255, 128, 0,
        0, 0, 127, 255, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0,
        i32::from(u8::MIN),
        i32::from(u8::MAX),
        i32::from(i8::MIN),
        i32::from(i8::MAX),
        i32::from(u16::MIN),
        i32::from(u16::MAX),
        i32::from(i16::MIN),
        i32::from(i16::MAX),
        i32::MIN,
        i32::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i64_enumeration_le_test() {
    type T = i64;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0,
        0, 128, 255, 255, 255, 255, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255,
        255, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
        255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 255, 255, 255,
        255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 255,
        255, 255, 255, 127,
    ];
    let expected: Vec<T> = vec![
        0,
        i64::from(u8::MIN),
        i64::from(u8::MAX),
        i64::from(i8::MIN),
        i64::from(i8::MAX),
        i64::from(u16::MIN),
        i64::from(u16::MAX),
        i64::from(i16::MIN),
        i64::from(i16::MAX),
        i64::from(u32::MIN),
        i64::from(u32::MAX),
        i64::from(i32::MIN),
        i64::from(i32::MAX),
        i64::MIN,
        i64::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn i64_enumeration_be_test() {
    type T = i64;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 127,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
        255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 127, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0, 0,
        0, 0, 127, 255, 255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 127, 255, 255, 255,
        255, 255, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0,
        i64::from(u8::MIN),
        i64::from(u8::MAX),
        i64::from(i8::MIN),
        i64::from(i8::MAX),
        i64::from(u16::MIN),
        i64::from(u16::MAX),
        i64::from(i16::MIN),
        i64::from(i16::MAX),
        i64::from(u32::MIN),
        i64::from(u32::MAX),
        i64::from(i32::MIN),
        i64::from(i32::MAX),
        i64::MIN,
        i64::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn f32_enumeration_le_test() {
    type T = f32;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 67, 0, 0, 0, 195, 0, 0, 254, 66, 0,
        0, 0, 0, 0, 255, 127, 71, 0, 0, 0, 199, 0, 254, 255, 70, 255, 255, 127,
        255, 255, 255, 127, 127,
    ];
    let expected: Vec<T> = vec![
        0.0,
        f32::from(u8::MIN),
        f32::from(u8::MAX),
        f32::from(i8::MIN),
        f32::from(i8::MAX),
        f32::from(u16::MIN),
        f32::from(u16::MAX),
        f32::from(i16::MIN),
        f32::from(i16::MAX),
        f32::MIN,
        f32::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn f32_enumeration_be_test() {
    type T = f32;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 67, 127, 0, 0, 195, 0, 0, 0, 66, 254, 0, 0, 0,
        0, 0, 0, 71, 127, 255, 0, 199, 0, 0, 0, 70, 255, 254, 0, 255, 127, 255,
        255, 127, 127, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0.0,
        f32::from(u8::MIN),
        f32::from(u8::MAX),
        f32::from(i8::MIN),
        f32::from(i8::MAX),
        f32::from(u16::MIN),
        f32::from(u16::MAX),
        f32::from(i16::MIN),
        f32::from(i16::MAX),
        f32::MIN,
        f32::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn f64_enumeration_le_test() {
    type T = f64;
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224,
        111, 64, 0, 0, 0, 0, 0, 0, 96, 192, 0, 0, 0, 0, 0, 192, 95, 64, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 255, 239, 64, 0, 0, 0, 0, 0, 0, 224,
        192, 0, 0, 0, 0, 192, 255, 223, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224,
        255, 255, 255, 239, 65, 0, 0, 0, 0, 0, 0, 224, 193, 0, 0, 192, 255,
        255, 255, 223, 65, 255, 255, 255, 255, 255, 255, 239, 255, 255, 255,
        255, 255, 255, 255, 239, 127,
    ];
    let expected: Vec<T> = vec![
        0.0,
        f64::from(u8::MIN),
        f64::from(u8::MAX),
        f64::from(i8::MIN),
        f64::from(i8::MAX),
        f64::from(u16::MIN),
        f64::from(u16::MAX),
        f64::from(i16::MIN),
        f64::from(i16::MAX),
        f64::from(u32::MIN),
        f64::from(u32::MAX),
        f64::from(i32::MIN),
        f64::from(i32::MAX),
        f64::MIN,
        f64::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn f64_enumeration_be_test() {
    type T = f64;
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 111, 224, 0, 0, 0,
        0, 0, 192, 96, 0, 0, 0, 0, 0, 0, 64, 95, 192, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 64, 239, 255, 224, 0, 0, 0, 0, 192, 224, 0, 0, 0, 0, 0,
        0, 64, 223, 255, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 239, 255,
        255, 255, 224, 0, 0, 193, 224, 0, 0, 0, 0, 0, 0, 65, 223, 255, 255,
        255, 192, 0, 0, 255, 239, 255, 255, 255, 255, 255, 255, 127, 239, 255,
        255, 255, 255, 255, 255,
    ];
    let expected: Vec<T> = vec![
        0.0,
        f64::from(u8::MIN),
        f64::from(u8::MAX),
        f64::from(i8::MIN),
        f64::from(i8::MAX),
        f64::from(u16::MIN),
        f64::from(u16::MAX),
        f64::from(i16::MIN),
        f64::from(i16::MAX),
        f64::from(u32::MIN),
        f64::from(u32::MAX),
        f64::from(i32::MIN),
        f64::from(i32::MAX),
        f64::MIN,
        f64::MAX,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    for val in &expected {
        assert_eq!(vac.read::<T>(), *val);
    }
}

#[test]
fn everything_le_test() {
    const ENDIANNESS: Endianness = Endianness::Little;
    let input: Vec<u8> = vec![
        42, 42, 0, 42, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 66, 0, 0, 0,
        0, 0, 0, 69, 64,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    assert_eq!(vac.read::<u8>(), 42_u8);
    assert_eq!(vac.read::<u16>(), 42_u16);
    assert_eq!(vac.read::<u32>(), 42_u32);
    assert_eq!(vac.read::<u64>(), 42_u64);
    assert_eq!(vac.read::<f32>(), 42.0);
    assert_eq!(vac.read::<f64>(), 42.0);
}

#[test]
fn everything_be_test() {
    const ENDIANNESS: Endianness = Endianness::Big;
    let input: Vec<u8> = vec![
        42, 0, 42, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 42, 66, 40, 0, 0, 64, 69,
        0, 0, 0, 0, 0, 0,
    ];
    let mut vac = ByteStreamer::new(&input, ENDIANNESS);
    assert_eq!(vac.read::<u8>(), 42_u8);
    assert_eq!(vac.read::<u16>(), 42_u16);
    assert_eq!(vac.read::<u32>(), 42_u32);
    assert_eq!(vac.read::<u64>(), 42_u64);
    assert_eq!(vac.read::<f32>(), 42.0);
    assert_eq!(vac.read::<f64>(), 42.0);
}
