use crate::transform::split::transform;
use quote::ToTokens as _;
use quote::quote;

#[test]
fn split0_test() {
    let pragma = quote! { #[loop_split()] };
    for src in [
        quote! { for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break 'loop_label; } } },
    ] {
        let src = quote! { #pragma #src rest };
        let crate::Item::LoopSplitAttr(conf) =
            syn::parse2::<crate::Item>(src).unwrap()
        else {
            unreachable!()
        };

        let res = transform(conf);
        assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: while true {
                let mut r#iter = iter;
                for elt in r#iter { body; if should_break() { break 'loop_label; } }
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
fn split1_test() {
    let pragma = quote! { #[loop_split(first_chunk_len)] };
    for src in [
        quote! { for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break 'loop_label; } } },
    ] {
        let src = quote! { #pragma #src rest };
        let crate::Item::LoopSplitAttr(conf) =
            syn::parse2::<crate::Item>(src).unwrap()
        else {
            unreachable!()
        };

        let res = transform(conf);
        assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: while true {
                let mut r#iter = iter;
                for r#i in 0..first_chunk_len {
                    if let Some(elt) = r#iter.next() { body; if should_break() { break 'loop_label; } } else { break 'loop_label; }
                }
                for elt in r#iter { body; if should_break() { break 'loop_label; } }
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
fn split2_test() {
    let pragma = quote! { #[loop_split(first_chunk_len, second_chunk_len)] };
    for src in [
        quote! { for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break; } } },
        quote! { 'loop_label: for elt in iter { body; if should_break() { break 'loop_label; } } },
    ] {
        let src = quote! { #pragma #src rest };
        let crate::Item::LoopSplitAttr(conf) =
            syn::parse2::<crate::Item>(src).unwrap()
        else {
            unreachable!()
        };

        let res = transform(conf);
        assert_eq!(
        res.to_string(),
        quote! {
            'loop_label: while true {
                let mut r#iter = iter;
                for r#i in 0..first_chunk_len {
                    if let Some(elt) = r#iter.next() { body; if should_break() { break 'loop_label; } } else { break 'loop_label; }
                }
                for r#i in 0..second_chunk_len {
                    if let Some(elt) = r#iter.next() { body; if should_break() { break 'loop_label; } } else { break 'loop_label; }
                }
                for elt in r#iter { body; if should_break() { break 'loop_label; } }
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
    let pragma = quote! { #[loop_split(first_chunk_len)] };
    let body = quote! {
        for elt in other_iter { body; if should_break() { break; } };
        while other_iter { body; if should_break() { break; } };
        loop { body; if should_break() { break; } };
    };
    for src in [
        quote! {
            for elt in iter { #body if should_break() { break; } }
        },
        quote! {
            'loop_label: for elt in iter { #body if should_break() { break; } }
        },
        quote! {
            'loop_label: for elt in iter { #body if should_break() { break 'loop_label; } }
        },
    ] {
        let src = quote! { #pragma #src rest };
        let crate::Item::LoopSplitAttr(conf) =
            syn::parse2::<crate::Item>(src).unwrap()
        else {
            unreachable!()
        };

        let res = transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    for r#i in 0..first_chunk_len {
                        if let Some(elt) = r#iter.next() {
                            #body
                            if should_break() { break 'loop_label; }
                        } else {
                            break 'loop_label;
                        }
                    }
                    for elt in r#iter {
                        #body
                        if should_break() { break 'loop_label; }
                    }
                    break 'loop_label;
                }
                rest
            }
            .to_token_stream()
            .to_string()
        );
    }
}
