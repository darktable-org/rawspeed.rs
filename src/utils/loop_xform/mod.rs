mod kw {
    syn::custom_keyword!(runtime);
    syn::custom_keyword!(with_remainder);
}

#[derive(PartialEq, Eq, Debug)]
enum UnrollMethod {
    Runtime,
    WithRemainder,
}

enum Item {
    LoopUnrollAttr(LoopUnrollConf),
}

struct LoopUnrollConf {
    pub unroll_method: UnrollMethod,
    pub unroll_factor: usize,
    pub for_loop: syn::ExprForLoop,
    pub rest_of_tokenstream: proc_macro2::TokenStream,
}

#[proc_macro]
pub fn enable_loop_xforms(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as Item);

    match input {
        Item::LoopUnrollAttr(c) => {
            #[cfg(clippy)]
            {
                use quote::ToTokens as _;
                return c.for_loop.to_token_stream().into();
            }
            #[cfg_attr(clippy, allow(unreachable_code))]
            transform::perform_loop_unroll(c).into()
        }
    }
}

mod parse;
mod transform;
