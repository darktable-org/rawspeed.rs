use crate::LoggingIter;
use core::cell::RefCell;
use rawspeed_utils_loop_xform::enable_loop_xforms;
use std::rc::Rc;

fn gen_unroll_output(uf: usize, r: core::ops::Range<usize>) -> Vec<String> {
    let mut vec = vec![];
    vec.push("Before macro".to_owned());
    vec.push(format!("Iter created, at {}", r.start).to_owned());
    let iterspace = r.clone().collect::<Vec<_>>();
    let mut chunks = iterspace[..].chunks_exact(uf);
    for chunk in chunks.by_ref() {
        for i in chunk {
            vec.push(format!("Iter next() called at pos = {i}"));
            vec.push(format!(
                "Iter next() called at pos = {i}, returning {i}, next is {}",
                i + 1
            ));
            vec.push(format!("IterVal({i}) created"));
        }
        for i in chunk {
            vec.push(format!("IterVal({i}) deref"));
            vec.push(format!("Loop body at i = {i}"));
            vec.push(format!("IterVal({i}) dropped"));
        }
    }
    for i in chunks.remainder() {
        vec.push(format!("Iter next() called at pos = {i}"));
        vec.push(format!(
            "Iter next() called at pos = {i}, returning {i}, next is {}",
            i + 1
        ));
        vec.push(format!("IterVal({i}) created"));
    }
    vec.push(format!("Iter next() called at pos = {}", r.end));
    vec.push(format!(
        "Iter next() called at pos = {}, returning None",
        r.end,
    ));
    for i in chunks.remainder() {
        vec.push(format!("IterVal({i}) deref"));
        vec.push(format!("Loop body at i = {i}"));
        vec.push(format!("IterVal({i}) dropped"));
    }
    vec.push(format!("Iter dropped, was at {}", r.end,));
    vec.push("After loop".to_owned());
    vec.push("After macro".to_owned());
    vec
}

macro_rules! gen_test {
    ($name:ident, $uf:expr, $len:expr) => {
        #[test]
        fn $name() {
            let log = Rc::new(RefCell::new(Vec::<String>::new()));

            log.borrow_mut().push("Before macro".to_owned());
            enable_loop_xforms!(
                #[loop_unroll(method(with_remainder), factor($uf))]
                for i in LoggingIter::new(Rc::clone(&log), 0..$len) {
                    let i = *i;
                    log.borrow_mut().push(format!("Loop body at i = {i}"));
                }
            );
            log.borrow_mut().push("After loop".to_owned());
            log.borrow_mut().push("After macro".to_owned());

            assert_eq!(log.borrow()[..], gen_unroll_output($uf, 0..$len));
        }
    };
}

gen_test!(unroll1_len0, 1, 0);
gen_test!(unroll1_len1, 1, 1);
gen_test!(unroll1_len2, 1, 2);
gen_test!(unroll1_len3, 1, 3);
gen_test!(unroll1_len4, 1, 4);
gen_test!(unroll1_len5, 1, 5);

gen_test!(unroll2_len0, 2, 0);
gen_test!(unroll2_len1, 2, 1);
gen_test!(unroll2_len2, 2, 2);
gen_test!(unroll2_len3, 2, 3);
gen_test!(unroll2_len4, 2, 4);
gen_test!(unroll2_len5, 2, 5);

gen_test!(unroll3_len0, 3, 0);
gen_test!(unroll3_len1, 3, 1);
gen_test!(unroll3_len2, 3, 2);
gen_test!(unroll3_len3, 3, 3);
gen_test!(unroll3_len4, 3, 4);
gen_test!(unroll3_len5, 3, 5);

#[test]
fn break0_unroll2_len2_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(with_remainder), factor(2))]
        for i in LoggingIter::new(Rc::clone(&log), 0..2) {
            let i = *i;
            log.borrow_mut().push(format!("Loop body at i = {i}"));
            if i == 0 {
                break;
            }
        }
    );
    log.borrow_mut().push("After loop".to_owned());
    log.borrow_mut().push("After macro".to_owned());

    assert_eq!(
        log.borrow()[..],
        [
            "Before macro",
            "Iter created, at 0",
            "Iter next() called at pos = 0",
            "Iter next() called at pos = 0, returning 0, next is 1",
            "IterVal(0) created",
            "Iter next() called at pos = 1",
            "Iter next() called at pos = 1, returning 1, next is 2",
            "IterVal(1) created",
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "IterVal(1) dropped",
            "Iter dropped, was at 2",
            "After loop",
            "After macro"
        ]
    );
}

#[test]
fn break0_unroll3_len2_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(with_remainder), factor(3))]
        for i in LoggingIter::new(Rc::clone(&log), 0..2) {
            let i = *i;
            log.borrow_mut().push(format!("Loop body at i = {i}"));
            if i == 0 {
                break;
            }
        }
    );
    log.borrow_mut().push("After loop".to_owned());
    log.borrow_mut().push("After macro".to_owned());

    assert_eq!(
        log.borrow()[..],
        [
            "Before macro",
            "Iter created, at 0",
            "Iter next() called at pos = 0",
            "Iter next() called at pos = 0, returning 0, next is 1",
            "IterVal(0) created",
            "Iter next() called at pos = 1",
            "Iter next() called at pos = 1, returning 1, next is 2",
            "IterVal(1) created",
            "Iter next() called at pos = 2",
            "Iter next() called at pos = 2, returning None",
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "IterVal(1) dropped",
            "Iter dropped, was at 2",
            "After loop",
            "After macro"
        ]
    );
}

#[test]
fn break0_unroll2_len3_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(with_remainder), factor(2))]
        for i in LoggingIter::new(Rc::clone(&log), 0..3) {
            let i = *i;
            log.borrow_mut().push(format!("Loop body at i = {i}"));
            if i == 0 {
                break;
            }
        }
    );
    log.borrow_mut().push("After loop".to_owned());
    log.borrow_mut().push("After macro".to_owned());

    assert_eq!(
        log.borrow()[..],
        [
            "Before macro",
            "Iter created, at 0",
            "Iter next() called at pos = 0",
            "Iter next() called at pos = 0, returning 0, next is 1",
            "IterVal(0) created",
            "Iter next() called at pos = 1",
            "Iter next() called at pos = 1, returning 1, next is 2",
            "IterVal(1) created",
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "IterVal(1) dropped",
            "Iter dropped, was at 2",
            "After loop",
            "After macro"
        ]
    );
}

#[test]
fn break0_unroll3_len3_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(with_remainder), factor(3))]
        for i in LoggingIter::new(Rc::clone(&log), 0..3) {
            let i = *i;
            log.borrow_mut().push(format!("Loop body at i = {i}"));
            if i == 0 {
                break;
            }
        }
    );
    log.borrow_mut().push("After loop".to_owned());
    log.borrow_mut().push("After macro".to_owned());

    assert_eq!(
        log.borrow()[..],
        [
            "Before macro",
            "Iter created, at 0",
            "Iter next() called at pos = 0",
            "Iter next() called at pos = 0, returning 0, next is 1",
            "IterVal(0) created",
            "Iter next() called at pos = 1",
            "Iter next() called at pos = 1, returning 1, next is 2",
            "IterVal(1) created",
            "Iter next() called at pos = 2",
            "Iter next() called at pos = 2, returning 2, next is 3",
            "IterVal(2) created",
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "IterVal(1) dropped",
            "IterVal(2) dropped",
            "Iter dropped, was at 3",
            "After loop",
            "After macro"
        ]
    );
}
