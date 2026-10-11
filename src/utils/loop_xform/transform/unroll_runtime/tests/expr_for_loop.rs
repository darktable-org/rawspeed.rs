use crate::{
    Loop, LoopUnrollParams, LoopXFormConf, UnrollMethod,
    transform::unroll_runtime::transform,
};

use quote::ToTokens as _;
use quote::quote;

#[test]
fn unroll1_test() {
    let conf = LoopXFormConf {
        params: LoopUnrollParams {
            unroll_method: UnrollMethod::Runtime,
            unroll_factor: 1,
        },
        the_loop: syn::parse2::<Loop>(
            quote! { 'loop_label: for elt in iter { body } },
        )
        .unwrap(),
        rest_of_tokenstream: quote! { rest },
    };

    let res = transform(&conf);
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
    let conf = LoopXFormConf {
        params: LoopUnrollParams {
            unroll_method: UnrollMethod::Runtime,
            unroll_factor: 2,
        },
        the_loop: syn::parse2::<Loop>(
            quote! { 'loop_label: for elt in iter { body } },
        )
        .unwrap(),
        rest_of_tokenstream: quote! { rest },
    };

    let res = transform(&conf);
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
