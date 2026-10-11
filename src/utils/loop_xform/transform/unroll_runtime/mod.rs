use quote::quote;

use crate::{LoopUnrollParams, LoopXFormConf};
// use syn::{ExprForLoop, token::Loop};

fn transform_ExprForLoop(
    ast: &LoopXFormConf<LoopUnrollParams>,
    expr_for_loop: &syn::ExprForLoop,
) -> proc_macro2::TokenStream {
    let label = &expr_for_loop.label;
    let pat = expr_for_loop.pat.as_ref();
    let expr = expr_for_loop.expr.as_ref();
    let body_stmts = expr_for_loop.body.stmts.as_slice();
    let remainder = &ast.rest_of_tokenstream;

    let iter = syn::Ident::new_raw("iter", proc_macro2::Span::mixed_site());

    let new_body = core::iter::repeat_n(
        quote! {
            if let Some(#pat) = #iter.next() {
                #(#body_stmts)*
            } else {
                break;
            }
        },
        ast.params.unroll_factor,
    );

    quote! {
        {
            let mut #iter = #expr;
            #label while true {
                #(#new_body)*
            }
        }
        #remainder
    }
}

fn transform_ExprWhile(
    ast: &LoopXFormConf<LoopUnrollParams>,
    expr_while: &syn::ExprWhile,
) -> proc_macro2::TokenStream {
    let label = &expr_while.label;
    let cond = expr_while.cond.as_ref();
    let body_stmts = expr_while.body.stmts.as_slice();
    let remainder = &ast.rest_of_tokenstream;

    let new_body = core::iter::repeat_n(
        quote! {
            if #cond {
                #(#body_stmts)*
            } else {
                break;
            }
        },
        ast.params.unroll_factor,
    );

    quote! {
        #label while true {
            #(#new_body)*
        }
        #remainder
    }
}

fn transform_ExprLoop(
    ast: &LoopXFormConf<LoopUnrollParams>,
    expr_loop: &syn::ExprLoop,
) -> proc_macro2::TokenStream {
    let label = &expr_loop.label;
    let body_stmts = expr_loop.body.stmts.as_slice();
    let remainder = &ast.rest_of_tokenstream;

    let new_body = core::iter::repeat_n(
        quote! {
            {
                #(#body_stmts)*
            }
        },
        ast.params.unroll_factor,
    );

    quote! {
        #label loop {
            #(#new_body)*
        }
        #remainder
    }
}

pub fn transform(
    ast: &LoopXFormConf<LoopUnrollParams>,
) -> proc_macro2::TokenStream {
    match &ast.the_loop {
        crate::Loop::ExprForLoop(expr_for_loop) => {
            transform_ExprForLoop(ast, expr_for_loop)
        }
        crate::Loop::ExprWhile(expr_while) => {
            transform_ExprWhile(ast, expr_while)
        }
        crate::Loop::ExprLoop(expr_loop) => transform_ExprLoop(ast, expr_loop),
    }
}

#[cfg(test)]
mod tests;
