use quote::ToTokens as _;
use quote::quote;

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn t0_test() {
    let tokens = quote! {
        for i in e {}
    };
    let mut for_loop = syn::parse2::<syn::ExprForLoop>(tokens).unwrap();
    super::LabelUnlabelledBreaks::visit(&mut for_loop);
}

#[test]
fn test() {
    let body_verbatim = quote! {
        break 'loop_label;
        for a in b {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
        'other_loop_label: for a in b {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
        while c {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
        'other_loop_label: while c {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
        loop {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
        'other_loop_label: loop {
            break;
            break 'loop_label;
            break 'other_loop_label;
        }
    };
    let tokens = quote! {
        'loop_label: for i in e {
            break;
            #body_verbatim
        }
    };
    let mut for_loop = syn::parse2::<syn::ExprForLoop>(tokens).unwrap();
    super::LabelUnlabelledBreaks::visit(&mut for_loop);

    assert_eq!(
        quote! { #for_loop }.to_string(),
        quote! {
            'loop_label: for i in e {
                break 'loop_label;
                #body_verbatim
            }
        }
        .to_token_stream()
        .to_string()
    );
}
