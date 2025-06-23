macro_rules! gen_test {
    ($name:ident, $peel_count:expr) => {
        mod $name {
            #[test]
            fn expr_for_loop() {
                for len in 0..5 {
                    for break_after in [None].iter().copied().chain((0..=len).into_iter().map(Some)) {
                        let log = std::rc::Rc::new(core::cell::RefCell::new(Vec::<String>::new()));

                        log.borrow_mut().push("Before macro".to_owned());
                        rawspeed_utils_loop_xform::enable_loop_xforms!(
                            #[loop_peel($peel_count)]
                            for i in crate::LoggingIter::new(std::rc::Rc::clone(&log), 0..len) {
                                let i = *i;
                                log.borrow_mut().push(format!("Loop body at i = {i}"));
                                if Some(i) == break_after {
                                    break;
                                }
                            }
                        );
                        log.borrow_mut().push("After loop".to_owned());
                        log.borrow_mut().push("After macro".to_owned());

                        assert_eq!(log.borrow()[..], crate::gen_native_output(0..len, break_after));
                    }
                }
            }
            #[test]
            fn expr_while() {
                for len in 0..5 {
                    for break_after in [None].iter().copied().chain((0..=len).into_iter().map(Some)) {
                        let log = std::rc::Rc::new(core::cell::RefCell::new(Vec::<String>::new()));

                        log.borrow_mut().push("Before macro".to_owned());
                        let mut iter = crate::LoggingIter::new(std::rc::Rc::clone(&log), 0..len);
                        rawspeed_utils_loop_xform::enable_loop_xforms!(
                            #[loop_peel($peel_count)]
                            while let Some(i) = iter.next() {
                                let i = *i;
                                log.borrow_mut().push(format!("Loop body at i = {i}"));
                                    if Some(i) == break_after {
                                        break;
                                    }
                            };
                        );
                        drop(iter);
                        log.borrow_mut().push("After loop".to_owned());
                        log.borrow_mut().push("After macro".to_owned());

                        assert_eq!(log.borrow()[..], crate::gen_native_output(0..len, break_after));
                    }
                }
            }
            #[test]
            fn expr_loop() {
                for len in 0..5 {
                    for break_after in [None].iter().copied().chain((0..=len).into_iter().map(Some)) {
                        let log = std::rc::Rc::new(core::cell::RefCell::new(Vec::<String>::new()));

                        log.borrow_mut().push("Before macro".to_owned());
                        let mut iter = crate::LoggingIter::new(std::rc::Rc::clone(&log), 0..len);
                        rawspeed_utils_loop_xform::enable_loop_xforms!(
                            #[loop_peel($peel_count)]
                            loop {
                                let Some(i) = iter.next() else { break; };
                                let i = *i;
                                log.borrow_mut().push(format!("Loop body at i = {i}"));
                                    if Some(i) == break_after {
                                        break;
                                    }
                            };
                        );
                        drop(iter);
                        log.borrow_mut().push("After loop".to_owned());
                        log.borrow_mut().push("After macro".to_owned());

                        assert_eq!(log.borrow()[..], crate::gen_native_output(0..len, break_after));
                    }
                }
            }
        }
    }
}

gen_test!(peel1, 1);
gen_test!(peel2, 2);
gen_test!(peel3, 3);
gen_test!(peel4, 4);
