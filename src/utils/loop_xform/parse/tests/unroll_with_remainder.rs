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
    match syn::parse2::<Item>(tokens).unwrap() {
        Item::LoopUnrollAttr(loop_unroll_conf) => {
            assert_eq!(
                loop_unroll_conf.unroll_method,
                UnrollMethod::WithRemainder
            );
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
