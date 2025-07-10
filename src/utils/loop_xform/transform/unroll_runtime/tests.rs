use super::super::UnrollMethod;
use super::LoopUnrollConf;
use quote::ToTokens as _;
use quote::quote;
use syn::ExprForLoop;

#[test]
fn unroll1_test() {
    let conf = LoopUnrollConf {
        unroll_method: UnrollMethod::Runtime,
        for_loop: syn::parse2::<ExprForLoop>(
            quote! { 'loop_label: for elt in iter { body } },
        )
        .unwrap(),
        unroll_factor: 1,
        rest_of_tokenstream: quote! { rest },
    };

    let res = super::transform(&conf);
    assert_eq!(
        res.to_string(),
        quote! {
            {
                let mut r#iter = iter;
                'loop_label : while true {
                    if let Some(elt) = r#iter.next() { body } else { break; }
                }
            }
            rest
        }
        .to_token_stream()
        .to_string()
    );
}

#[test]
fn unroll2_test() {
    let conf = LoopUnrollConf {
        unroll_method: UnrollMethod::Runtime,
        for_loop: syn::parse2::<ExprForLoop>(
            quote! { 'loop_label: for elt in iter { body } },
        )
        .unwrap(),
        unroll_factor: 2,
        rest_of_tokenstream: quote! { rest },
    };

    let res = super::transform(&conf);
    assert_eq!(
        res.to_string(),
        quote! {
            {
                let mut r#iter = iter;
                'loop_label : while true {
                    if let Some(elt) = r#iter.next() { body } else { break; }
                    if let Some(elt) = r#iter.next() { body } else { break; }
                }
            }
            rest
        }
        .to_token_stream()
        .to_string()
    );
}
