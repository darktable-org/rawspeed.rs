use quote::ToTokens;

mod kw {
    syn::custom_keyword!(runtime);
    syn::custom_keyword!(with_remainder);
}

enum Loop {
    ExprForLoop(syn::ExprForLoop),
    ExprWhile(syn::ExprWhile),
    ExprLoop(syn::ExprLoop),
}

impl From<syn::ExprForLoop> for Loop {
    fn from(v: syn::ExprForLoop) -> Self {
        Self::ExprForLoop(v)
    }
}

impl From<syn::ExprWhile> for Loop {
    fn from(v: syn::ExprWhile) -> Self {
        Self::ExprWhile(v)
    }
}

impl From<syn::ExprLoop> for Loop {
    fn from(v: syn::ExprLoop) -> Self {
        Self::ExprLoop(v)
    }
}

impl ToTokens for Loop {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::ExprForLoop(expr_for_loop) => expr_for_loop.to_tokens(tokens),
            Self::ExprWhile(expr_while) => expr_while.to_tokens(tokens),
            Self::ExprLoop(expr_loop) => expr_loop.to_tokens(tokens),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum UnrollMethod {
    Runtime,
    WithRemainder,
}

struct LoopUnrollParams {
    pub unroll_method: UnrollMethod,
    pub unroll_factor: usize,
}

struct LoopPeelParams {
    pub peel_count: usize,
}

struct LoopSplitParams {
    pub split_lens:
        syn::punctuated::Punctuated<proc_macro2::TokenTree, syn::Token![,]>,
}

enum LoopXFormParams {
    LoopUnroll(LoopUnrollParams),
    LoopPeel(LoopPeelParams),
    LoopSplit(LoopSplitParams),
}

impl From<LoopUnrollParams> for LoopXFormParams {
    fn from(value: LoopUnrollParams) -> Self {
        Self::LoopUnroll(value)
    }
}

impl From<LoopPeelParams> for LoopXFormParams {
    fn from(value: LoopPeelParams) -> Self {
        Self::LoopPeel(value)
    }
}

impl From<LoopSplitParams> for LoopXFormParams {
    fn from(value: LoopSplitParams) -> Self {
        Self::LoopSplit(value)
    }
}

struct LoopXFormConf<Params> {
    pub params: Params,
    pub the_loop: Loop,
    pub rest_of_tokenstream: proc_macro2::TokenStream,
}

enum Item {
    LoopUnrollAttr(LoopXFormConf<LoopUnrollParams>),
    LoopPeelAttr(LoopXFormConf<LoopPeelParams>),
    LoopSplitAttr(LoopXFormConf<LoopSplitParams>),
}

impl Item {
    fn new(
        params: LoopXFormParams,
        the_loop: Loop,
        rest_of_tokenstream: proc_macro2::TokenStream,
    ) -> Self {
        match params {
            LoopXFormParams::LoopUnroll(params) => {
                Item::LoopUnrollAttr(LoopXFormConf {
                    params,
                    the_loop,
                    rest_of_tokenstream,
                })
            }
            LoopXFormParams::LoopPeel(params) => {
                Item::LoopPeelAttr(LoopXFormConf {
                    params,
                    the_loop,
                    rest_of_tokenstream,
                })
            }
            LoopXFormParams::LoopSplit(params) => {
                Item::LoopSplitAttr(LoopXFormConf {
                    params,
                    the_loop,
                    rest_of_tokenstream,
                })
            }
        }
    }

    const fn the_loop(&self) -> &Loop {
        match self {
            Self::LoopUnrollAttr(loop_xform_conf) => &loop_xform_conf.the_loop,
            Self::LoopPeelAttr(loop_xform_conf) => &loop_xform_conf.the_loop,
            Self::LoopSplitAttr(loop_xform_conf) => &loop_xform_conf.the_loop,
        }
    }
}

#[proc_macro]
#[inline(never)]
pub fn enable_loop_xforms(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as Item);
    if cfg!(clippy) {
        use quote::ToTokens as _;
        return input.the_loop().to_token_stream().into();
    }
    match input {
        Item::LoopUnrollAttr(c) => transform::perform_loop_unroll(c).into(),
        Item::LoopPeelAttr(c) => transform::perform_loop_peel(c).into(),
        Item::LoopSplitAttr(c) => transform::perform_loop_split(c).into(),
        _ => unreachable!(),
    }
}

mod parse;
mod transform;
