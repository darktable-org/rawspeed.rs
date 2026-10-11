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
#[should_panic(expected = "attribute expected")]
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
    expected = "expected attribute arguments in parentheses: #[loop_split(...)]"
)]
fn t2_test() {
    let tokens = quote! {
        #[loop_split]
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
    expected = "unexpected end of input, expected some kind of loop"
)]
fn t3_test() {
    let tokens = quote! {
        #[loop_split(42)]
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
fn t4_test() {
    let tokens = quote! {
        #[loop_split(1)]
        #[loop_split(1)]
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
        #[loop_split(foo, bar, )]
        'loop_label: for elt in iter { body } rest
    };
    let Item::LoopSplitAttr(attr) = syn::parse2::<Item>(tokens).unwrap() else {
        unreachable!()
    };
    assert_eq!(
        attr.params
            .split_lens
            .into_iter()
            .map(|l| l.to_token_stream().to_string())
            .collect::<Vec<_>>(),
        ["foo", "bar"]
    );
    let crate::Loop::ExprForLoop(for_loop) = attr.the_loop else {
        unreachable!()
    };
    assert_eq!(
        for_loop.label.to_token_stream().to_string(),
        ("'loop_label :")
    );
    assert_eq!(for_loop.pat.to_token_stream().to_string(), ("elt"));
    assert_eq!(for_loop.expr.to_token_stream().to_string(), ("iter"));
    assert_eq!(for_loop.body.to_token_stream().to_string(), ("{ body }"));
    assert_eq!(attr.rest_of_tokenstream.to_string(), "rest");
}
