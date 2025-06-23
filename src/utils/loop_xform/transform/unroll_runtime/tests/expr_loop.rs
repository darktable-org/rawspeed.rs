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
        the_loop: syn::parse2::<Loop>(quote! { 'loop_label: loop { body } })
            .unwrap(),
        rest_of_tokenstream: quote! { rest },
    };

    let res = transform(&conf);
    assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: loop {
                { body }
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
        the_loop: syn::parse2::<Loop>(quote! { 'loop_label: loop { body } })
            .unwrap(),
        rest_of_tokenstream: quote! { rest },
    };

    let res = transform(&conf);
    assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: loop {
                { body }
                { body }
            }
            rest
        }
        .to_token_stream()
        .to_string()
    );
}
