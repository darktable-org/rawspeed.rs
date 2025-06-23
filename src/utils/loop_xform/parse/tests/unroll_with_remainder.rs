use crate::Item;
use crate::UnrollMethod;
use quote::ToTokens as _;
use quote::quote;

#[test]
fn good_test() {
    let tokens = quote! {
        #[loop_unroll(method(with_remainder), factor(42))]
        'loop_label: for elt in iter { body } rest
    };
    let Item::LoopUnrollAttr(attr) = syn::parse2::<Item>(tokens).unwrap()
    else {
        unreachable!();
    };
    assert_eq!(attr.params.unroll_method, UnrollMethod::WithRemainder);
    assert_eq!(attr.params.unroll_factor, 42);
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
