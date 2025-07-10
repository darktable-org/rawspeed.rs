use crate::LoggingIter;
use crate::gen_native_output;
use core::cell::RefCell;
use std::rc::Rc;

macro_rules! gen_native_test {
    ($name:ident, $len:expr) => {
        #[test]
        fn $name() {
            let log = Rc::new(RefCell::new(Vec::<String>::new()));

            log.borrow_mut().push("Before macro".to_owned());
            for i in LoggingIter::new(Rc::clone(&log), 0..$len) {
                let i = *i;
                log.borrow_mut().push(format!("Loop body at i = {i}"));
            }
            log.borrow_mut().push("After loop".to_owned());
            log.borrow_mut().push("After macro".to_owned());

            assert_eq!(log.borrow()[..], gen_native_output(0..$len));
        }
    };
}

gen_native_test!(baseline_len0, 0);
gen_native_test!(baseline_len1, 1);
gen_native_test!(baseline_len2, 2);
gen_native_test!(baseline_len3, 3);
gen_native_test!(baseline_len4, 4);
gen_native_test!(baseline_len5, 5);

#[test]
fn break0_test() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    log.borrow_mut().push("Before macro".to_owned());
    for i in LoggingIter::new(Rc::clone(&log), 0..16) {
        let i = *i;
        log.borrow_mut().push(format!("Loop body at i = {i}"));
        if i == 0 {
            break;
        }
    }
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
    'my_loop: for i in LoggingIter::new(Rc::clone(&log), 0..16) {
        let i = *i;
        log.borrow_mut().push(format!("Loop body at i = {i}"));
        if i == 0 {
            break 'my_loop;
        }
    }
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
