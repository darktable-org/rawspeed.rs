use super::LoopUnrollConf;
use crate::UnrollMethod;
use quote::ToTokens as _;
use quote::quote;
use syn::ExprForLoop;

#[test]
fn unroll1_test() {
    for src in [
        quote! { for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break 'loop_label; } },
    ] {
        let conf = LoopUnrollConf {
            unroll_method: UnrollMethod::WithRemainder,
            for_loop: syn::parse2::<ExprForLoop>(src).unwrap(),
            unroll_factor: 1,
            rest_of_tokenstream: quote! { rest },
        };

        let res = super::transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    let mut r#iter_1_of_1 = None;
                    'label_inner: while true {
                        r#iter_1_of_1 = r#iter.next();
                        if r#iter_1_of_1.is_some() {
                            {
                                let elt = r#iter_1_of_1.take().unwrap();
                                body;
                                break 'loop_label;
                            }
                        } else {
                            break 'label_inner;
                        }
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

#[test]
fn unroll2_test() {
    for src in [
        quote! { for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break 'loop_label; } },
    ] {
        let conf = LoopUnrollConf {
            unroll_method: UnrollMethod::WithRemainder,
            for_loop: syn::parse2::<ExprForLoop>(src).unwrap(),
            unroll_factor: 2,
            rest_of_tokenstream: quote! { rest },
        };

        let res = super::transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    let mut r#iter_2_of_2 = None;
                    let mut r#iter_1_of_2 = None;
                    'label_inner: while true {
                        r#iter_1_of_2 = r#iter.next();
                        if r#iter_1_of_2.is_some() {
                            r#iter_2_of_2 = r#iter.next();
                            if r#iter_2_of_2.is_some() {
                                {
                                    let elt = r#iter_1_of_2.take().unwrap();
                                    body;
                                    break 'loop_label;
                                }
                                {
                                    let elt = r#iter_2_of_2.take().unwrap();
                                    body;
                                    break 'loop_label;
                                }
                            } else {
                                break 'label_inner;
                            }
                        } else {
                            break 'label_inner;
                        }
                    }
                    if let Some(elt) = r#iter_1_of_2.take() {
                        body;
                        break 'loop_label;
                    } else {
                        break 'loop_label;
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

#[test]
fn unroll3_test() {
    for src in [
        quote! { for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break; } },
        quote! { 'loop_label: for elt in iter { body; break 'loop_label; } },
    ] {
        let conf = LoopUnrollConf {
            unroll_method: UnrollMethod::WithRemainder,
            for_loop: syn::parse2::<ExprForLoop>(src).unwrap(),
            unroll_factor: 3,
            rest_of_tokenstream: quote! { rest },
        };

        let res = super::transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    let mut r#iter_3_of_3 = None;
                    let mut r#iter_2_of_3 = None;
                    let mut r#iter_1_of_3 = None;
                    'label_inner: while true {
                        r#iter_1_of_3 = r#iter.next();
                        if r#iter_1_of_3.is_some() {
                            r#iter_2_of_3 = r#iter.next();
                            if r#iter_2_of_3.is_some() {
                                r#iter_3_of_3 = r#iter.next();
                                if r#iter_3_of_3.is_some() {
                                    {
                                        let elt = r#iter_1_of_3.take().unwrap();
                                        body;
                                        break 'loop_label;
                                    }
                                    {
                                        let elt = r#iter_2_of_3.take().unwrap();
                                        body;
                                        break 'loop_label;
                                    }
                                    {
                                        let elt = r#iter_3_of_3.take().unwrap();
                                        body;
                                        break 'loop_label;
                                    }
                                } else {
                                    break 'label_inner;
                                }
                            } else {
                                break 'label_inner;
                            }
                        } else {
                            break 'label_inner;
                        }
                    }
                    if let Some(elt) = r#iter_1_of_3.take() {
                        body;
                        break 'loop_label;
                    } else {
                        break 'loop_label;
                    }
                    if let Some(elt) = r#iter_2_of_3.take() {
                        body;
                        break 'loop_label;
                    } else {
                        break 'loop_label;
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

#[test]
fn unroll1_with_nested_loop_test() {
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
        } },
        quote! {
        'loop_label: for elt in iter {
            #body
            break;
        } },
        quote! {
        'loop_label: for elt in iter {
            #body
            break 'loop_label;
        } },
    ] {
        let conf = LoopUnrollConf {
            unroll_method: UnrollMethod::WithRemainder,
            for_loop: syn::parse2::<ExprForLoop>(src).unwrap(),
            unroll_factor: 1,
            rest_of_tokenstream: quote! { rest },
        };

        let res = super::transform(conf);
        assert_eq!(
            res.to_string(),
            quote! {
                'loop_label: while true {
                    let mut r#iter = iter;
                    let mut r#iter_1_of_1 = None;
                    'label_inner: while true {
                        r#iter_1_of_1 = r#iter.next();
                        if r#iter_1_of_1.is_some() {
                            {
                                let elt = r#iter_1_of_1.take().unwrap();
                                #body
                                break 'loop_label;
                            }
                        } else {
                            break 'label_inner;
                        }
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
