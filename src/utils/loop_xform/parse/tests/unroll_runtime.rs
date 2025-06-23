use crate::Item;
use crate::UnrollMethod;
use quote::ToTokens as _;
use quote::quote;

#[test]
#[should_panic(expected = "There must be an attribute")]
fn t0_test() {
    let tokens = quote! {};
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "`loop_unroll` attribute expected")]
fn t1_test() {
    let tokens = quote! {
        #[attr]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(
    expected = "expected attribute arguments in parentheses: #[loop_unroll(...)"
)]
fn t2_test() {
    let tokens = quote! {
        #[loop_unroll]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "The attribute must specify unroll `method`")]
fn t3_test() {
    let tokens = quote! {
        #[loop_unroll()]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unexpected end of input, expected parentheses")]
fn t4_test() {
    let tokens = quote! {
        #[loop_unroll(method)]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(
    expected = "unexpected end of input, expected valid unroll method"
)]
fn t5_test() {
    let tokens = quote! {
        #[loop_unroll(method())]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "expected valid unroll method")]
fn t6_test() {
    let tokens = quote! {
        #[loop_unroll(method(run1time))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "The attribute must specify `factor`")]
fn t7_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "expected `,")]
fn t8_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime) factor)]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unexpected end of input, expected parentheses")]
fn t9_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor)]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unexpected end of input, expected integer literal")]
fn t10_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor())]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unexpected end of input, expected `for`")]
fn t11_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(42))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unroll factor should not have any suffix")]
fn t12_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(42u16))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "Unroll factor can not be zero")]
fn t13_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(0))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "unexpected end of input, expected `for`")]
fn t14_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(1))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
#[should_panic(expected = "There should only be a single attribute")]
fn t15_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(1))]
        #[loop_unroll(method(runtime), factor(1))]
    };
    match syn::parse2::<Item>(tokens) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[test]
fn good_test() {
    let tokens = quote! {
        #[loop_unroll(method(runtime), factor(42))]
        'loop_label: for elt in iter { body } rest
    };
    match syn::parse2::<Item>(tokens).unwrap() {
        Item::LoopUnrollAttr(loop_unroll_conf) => {
            assert_eq!(loop_unroll_conf.unroll_method, UnrollMethod::Runtime);
            assert_eq!(loop_unroll_conf.unroll_factor, 42);
            assert_eq!(
                loop_unroll_conf
                    .for_loop
                    .label
                    .to_token_stream()
                    .to_string(),
                ("'loop_label :")
            );
            assert_eq!(
                loop_unroll_conf.for_loop.pat.to_token_stream().to_string(),
                ("elt")
            );
            assert_eq!(
                loop_unroll_conf.for_loop.expr.to_token_stream().to_string(),
                ("iter")
            );
            assert_eq!(
                loop_unroll_conf.for_loop.body.to_token_stream().to_string(),
                ("{ body }")
            );
            assert_eq!(
                loop_unroll_conf.rest_of_tokenstream.to_string(),
                "rest"
            );
        }
    }
}
