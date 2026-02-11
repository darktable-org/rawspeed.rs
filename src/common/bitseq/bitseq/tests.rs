use crate::bitseq::{BitLen, BitSeq};

#[test]
#[should_panic(expected = "assertion failed: *len <= T::BITWIDTH")]
fn len_cap() {
    let b = BitLen::new(9);
    let _ = BitSeq::<u8>::new(b, 0);
}

#[test]
#[expect(clippy::cognitive_complexity)]
fn new_zext_tests() {
    macro_rules! test {
        ($expr:literal as $bit:literal-bit) => {
            let b = BitLen::new($bit);
            let r = BitSeq::<u8>::new(b, $expr).unwrap();
            assert_eq!(r.len(), b);
            assert_eq!(r.zext(), u8::from($expr));
        };
    }
    test!(0 as 0-bit);
    test!(0b0 as 1-bit);
    test!(0b1 as 1-bit);
    test!(0b00 as 2-bit);
    test!(0b01 as 2-bit);
    test!(0b10 as 2-bit);
    test!(0b11 as 2-bit);
}

#[test]
fn new_with_bad_len_tests() {
    macro_rules! test {
        ($expr:literal as $bit:literal-bit) => {
            let b = BitLen::new($bit);
            let r = BitSeq::<u8>::new(b, $expr);
            assert!(r.is_none());
        };
    }
    test!(1 as 0-bit);
    test!(0b10 as 1-bit);
}
