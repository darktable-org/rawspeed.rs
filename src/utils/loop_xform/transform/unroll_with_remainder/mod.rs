use crate::{LoopUnrollParams, LoopXFormConf};
use quote::{ToTokens as _, quote};
use syn::{Expr, Label, Lifetime};

pub fn transform(
    ast: LoopXFormConf<LoopUnrollParams>,
) -> proc_macro2::TokenStream {
    let crate::Loop::ExprForLoop(mut for_loop) = ast.the_loop else {
        return syn::Error::new_spanned(
            ast.the_loop.to_token_stream(),
            "Loop unroll with remainder only supported expr-for-loops",
        )
        .to_compile_error();
    };

    let label_outer = match for_loop.label {
        Some(l) => l.name.clone(),
        None => Lifetime::new("'loop_label", proc_macro2::Span::mixed_site()),
    };

    for_loop.label = Some(Label {
        name: label_outer.clone(),
        colon_token: syn::token::Colon {
            spans: [proc_macro2::Span::mixed_site()],
        },
    });

    super::utils::loop_break_labeller::LabelUnlabelledBreaks::visit_expr_for_loop(
        &mut for_loop,
    );

    let label_inner =
        Lifetime::new("'label_inner", proc_macro2::Span::mixed_site());

    let iter = syn::Ident::new_raw("iter", proc_macro2::Span::mixed_site());

    let mut iter_evals = vec![];
    let mut iter_elts = vec![];
    for i in 0..ast.params.unroll_factor {
        let suffix = format!("{}_of_{}", i + 1, ast.params.unroll_factor);
        iter_evals.push(syn::Ident::new_raw(
            &format!("iter_{suffix}"),
            proc_macro2::Span::mixed_site(),
        ));
        iter_elts.push(syn::Ident::new_raw(
            &format!("elt_{suffix}"),
            proc_macro2::Span::mixed_site(),
        ));
    }

    let p = Pieces {
        label_outer,
        pat: for_loop.pat,
        expr: for_loop.expr,
        body_stmts: for_loop.body.stmts,
        remainder: ast.rest_of_tokenstream,
        label_inner,
        iter,
        iter_evals,
        iter_elts,
    };
    builder(&p)
}

struct Pieces {
    label_outer: Lifetime,
    pat: Box<syn::Pat>,
    expr: Box<syn::Expr>,
    body_stmts: Vec<syn::Stmt>,
    remainder: proc_macro2::TokenStream,
    label_inner: Lifetime,
    iter: syn::Ident,
    iter_evals: Vec<syn::Ident>,
    iter_elts: Vec<syn::Ident>,
}

fn builder(s: &Pieces) -> proc_macro2::TokenStream {
    let label_outer = &s.label_outer;
    let pat = s.pat.as_ref();
    let expr = s.expr.as_ref();
    let body_stmts = s.body_stmts.as_slice();
    let remainder = &s.remainder;
    let label_inner = &s.label_inner;
    let iter = &s.iter;

    let prologue = s.iter_evals.iter().rev().map(|curr_pat| {
        quote! {
            let mut #curr_pat = None;
        }
    });

    let unrolled_iter_init = s.iter_evals.iter().map(Some).scan(None, |prev, curr| {
        let out = (*prev, curr);
        *prev = curr;
        Some(out)
    }).map(|(prev_iter, cur_iter)| {
        match (prev_iter, cur_iter) {
            (None, Some(cur_iter)) => {
                quote! {
                    #cur_iter = #iter.next();
                }
            }
            (Some(prev_iter), Some(cur_iter)) => {
                quote! {
                    #cur_iter = if #prev_iter.is_some() { #iter.next() } else { None };
                }
            }
            _ => unreachable!(),
        }
    });

    let unrolled_iter_cond =
        s.iter_evals.iter().zip(s.iter_elts.iter()).rev().map(
            |(curr_iter, curr_elt)| {
                quote! {
                    let Some(#curr_elt) = #curr_iter.take()
                }
            },
        );

    let unrolled_body = s.iter_elts.iter().map(|curr_elt| {
        quote! {
            {
                let #pat = #curr_elt;
                #(#body_stmts)*
            }
        }
    });

    let epilogue = s.iter_evals.iter().rev().skip(1).rev().map(|curr_pat| {
        quote! {
            if let Some(#pat) = #curr_pat.take() {
                #(#body_stmts)*
            } else {
                break #label_outer;
            }
        }
    });

    quote! {
        #label_outer: while true {
            let mut #iter = #expr;
            #(#prologue)*
            #label_inner: while true {
                #(#unrolled_iter_init)*
                if #(#unrolled_iter_cond)&&* {
                    #(#unrolled_body)*
                } else {
                    break #label_inner;
                }
            }
            #(#epilogue)*
            break #label_outer;
        }
        #remainder
    }
}

#[cfg(test)]
mod tests;
