use quote::quote;
use syn::{Label, Lifetime};

use crate::{LoopPeelParams, LoopXFormConf};
// use syn::{ExprForLoop, token::Loop};

fn transform_ExprForLoop(
    params: LoopPeelParams,
    mut expr_for_loop: syn::ExprForLoop,
    rest_of_tokenstream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let label = match expr_for_loop.label {
        Some(l) => l.name,
        None => Lifetime::new("'loop_label", proc_macro2::Span::mixed_site()),
    };

    expr_for_loop.label = Some(Label {
        name: label.clone(),
        colon_token: syn::token::Colon {
            spans: [proc_macro2::Span::mixed_site()],
        },
    });

    super::utils::loop_break_labeller::LabelUnlabelledBreaks::visit_expr_for_loop(
        &mut expr_for_loop,
    );

    expr_for_loop.label = None;

    let pat = expr_for_loop.pat.as_ref();
    let expr = expr_for_loop.expr.as_ref();
    let body_stmts = expr_for_loop.body.stmts.as_slice();

    let iter = syn::Ident::new_raw("iter", proc_macro2::Span::mixed_site());

    let prelude = core::iter::repeat_n(
        quote! {
            if let Some(#pat) = #iter.next() {
                #(#body_stmts)*
            } else {
                break #label;
            }
        },
        params.peel_count,
    );

    quote! {
        #label: while true {
            let mut #iter = #expr;
            #(#prelude)*
            for #pat in #iter {
                #(#body_stmts)*
            }
            break #label;
        }
        #rest_of_tokenstream
    }
}

fn transform_ExprWhile(
    params: LoopPeelParams,
    mut expr_while: syn::ExprWhile,
    rest_of_tokenstream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let label = match expr_while.label {
        Some(l) => l.name,
        None => Lifetime::new("'loop_label", proc_macro2::Span::mixed_site()),
    };

    expr_while.label = Some(Label {
        name: label.clone(),
        colon_token: syn::token::Colon {
            spans: [proc_macro2::Span::mixed_site()],
        },
    });

    super::utils::loop_break_labeller::LabelUnlabelledBreaks::visit_expr_while(
        &mut expr_while,
    );

    expr_while.label = None;

    let cond = &*expr_while.cond;
    let body_stmts = expr_while.body.stmts.as_slice();

    let prelude = core::iter::repeat_n(
        quote! {
            if #cond {
                #(#body_stmts)*
            } else {
                break #label;
            }
        },
        params.peel_count,
    );

    quote! {
        #label: while true {
            #(#prelude)*
            while #cond {
                #(#body_stmts)*
            }
            break #label;
        }
        #rest_of_tokenstream
    }
}

fn transform_ExprLoop(
    params: LoopPeelParams,
    mut expr_loop: syn::ExprLoop,
    rest_of_tokenstream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let label = match expr_loop.label {
        Some(l) => l.name,
        None => Lifetime::new("'loop_label", proc_macro2::Span::mixed_site()),
    };

    expr_loop.label = Some(Label {
        name: label.clone(),
        colon_token: syn::token::Colon {
            spans: [proc_macro2::Span::mixed_site()],
        },
    });

    super::utils::loop_break_labeller::LabelUnlabelledBreaks::visit_expr_loop(
        &mut expr_loop,
    );

    expr_loop.label = None;

    let body_stmts = expr_loop.body.stmts.as_slice();

    let prelude = core::iter::repeat_n(
        quote! {
            {
                #(#body_stmts)*
            }
        },
        params.peel_count,
    );

    quote! {
        #label: while true {
            #(#prelude)*
            loop {
                #(#body_stmts)*
            }
            break #label;
        }
        #rest_of_tokenstream
    }
}

pub fn transform(
    ast: LoopXFormConf<LoopPeelParams>,
) -> proc_macro2::TokenStream {
    let LoopXFormConf {
        params,
        the_loop,
        rest_of_tokenstream,
    } = ast;
    match the_loop {
        crate::Loop::ExprForLoop(expr_for_loop) => {
            transform_ExprForLoop(params, expr_for_loop, rest_of_tokenstream)
        }
        crate::Loop::ExprWhile(expr_while) => {
            transform_ExprWhile(params, expr_while, rest_of_tokenstream)
        }
        crate::Loop::ExprLoop(expr_loop) => {
            transform_ExprLoop(params, expr_loop, rest_of_tokenstream)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests;
