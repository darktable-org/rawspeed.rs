use crate::{Loop, LoopPeelParams, LoopXFormConf, transform::peel::transform};

use quote::ToTokens as _;
use quote::quote;

#[test]
fn peel1_test() {
    for src in [
        quote! { for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break 'loop_label; } },
    ] {
        let conf = LoopXFormConf {
            params: LoopPeelParams { peel_count: 1 },
            the_loop: syn::parse2::<Loop>(src).unwrap(),
            rest_of_tokenstream: quote! { rest },
        };

        let res = transform(conf);
        assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: while true {
                let mut r#iter = iter;
                if let Some(elt) = r#iter.next() { body; break 'loop_label; } else { break 'loop_label; }
                for elt in r#iter { body; break 'loop_label; }
                break 'loop_label;
            }
            rest
        }
        .to_token_stream()
        .to_string()
    );
    }
}

#[test]
fn peel2_test() {
    for src in [
        quote! { for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break 'loop_label; } },
    ] {
        let conf = LoopXFormConf {
            params: LoopPeelParams { peel_count: 2 },
            the_loop: syn::parse2::<Loop>(src).unwrap(),
            rest_of_tokenstream: quote! { rest },
        };

        let res = transform(conf);
        assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: while true {
                let mut r#iter = iter;
                if let Some(elt) = r#iter.next() { body; break 'loop_label; } else { break 'loop_label; }
                if let Some(elt) = r#iter.next() { body; break 'loop_label; } else { break 'loop_label; }
                for elt in r#iter { body; break 'loop_label; }
                break 'loop_label;
            }
            rest
        }
        .to_token_stream()
        .to_string()
    );
    }
}

#[test]
fn peel1_with_nested_loop_test() {
    let body = quote! {
        for elt in other_iter { body; break; };
        while other_iter { body; break; };
        loop { body; break; };
    };
    for src in [
        quote! {
            for elt in iter {
                #body
                break;
            }
        },
        quote! {
            'loop_label: for elt in iter {
                #body
                break;
            }
        },
        quote! {
            'loop_label: for elt in iter {
                #body
                break 'loop_label;
            }
        },
    ] {
        let conf = LoopXFormConf {
            params: LoopPeelParams { peel_count: 1 },
            the_loop: syn::parse2::<Loop>(src).unwrap(),
            rest_of_tokenstream: quote! { rest },
        };

        let res = transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    if let Some(elt) = r#iter.next() { #body break 'loop_label; } else { break 'loop_label; }
                    for elt in r#iter { #body break 'loop_label; }
                    break 'loop_label;
                }
                rest
            }
            .to_token_stream()
            .to_string()
        );
    }
}
