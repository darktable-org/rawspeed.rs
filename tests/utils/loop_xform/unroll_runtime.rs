use crate::LoggingIter;
use crate::gen_native_output;
use core::cell::RefCell;
use rawspeed_utils_loop_xform::enable_loop_xforms;
use std::rc::Rc;

macro_rules! gen_test {
    ($name:ident, $uf:expr, $len:expr) => {
        #[test]
        fn $name() {
            let log = Rc::new(RefCell::new(Vec::<String>::new()));

            log.borrow_mut().push("Before macro".to_owned());
            enable_loop_xforms!(
                #[loop_unroll(method(runtime), factor($uf))]
                for i in LoggingIter::new(Rc::clone(&log), 0..$len) {
                    let i = *i;
                    log.borrow_mut().push(format!("Loop body at i = {i}"));
                }
            );
            log.borrow_mut().push("After loop".to_owned());
            log.borrow_mut().push("After macro".to_owned());

            assert_eq!(log.borrow()[..], gen_native_output(0..$len));
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
fn break0_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(runtime), factor(16))]
        for i in LoggingIter::new(Rc::clone(&log), 0..16) {
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
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "Iter dropped, was at 1",
            "After loop",
            "After macro"
        ]
    );
}

#[test]
fn break_label0_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    enable_loop_xforms!(
        #[loop_unroll(method(runtime), factor(16))]
        'my_loop: for i in LoggingIter::new(Rc::clone(&log), 0..16) {
            let i = *i;
            log.borrow_mut().push(format!("Loop body at i = {i}"));
            if i == 0 {
                break 'my_loop;
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
            "IterVal(0) deref",
            "Loop body at i = 0",
            "IterVal(0) dropped",
            "Iter dropped, was at 1",
            "After loop",
            "After macro"
        ]
    );
}
