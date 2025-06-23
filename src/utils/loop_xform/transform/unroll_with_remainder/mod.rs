use super::super::LoopUnrollConf;
use quote::{ToTokens as _, quote};
use syn::{Label, Lifetime};

#[allow(clippy::single_call_fn)]
pub fn transform(ast: LoopUnrollConf) -> proc_macro2::TokenStream {
    let mut for_loop = ast.for_loop;

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

    super::utils::loop_break_labeller::LabelUnlabelledBreaks::visit(
        &mut for_loop,
    );

    let label_inner =
        Lifetime::new("'label_inner", proc_macro2::Span::mixed_site());

    let iter = syn::Ident::new_raw("iter", proc_macro2::Span::mixed_site());

    let mut iter_evals = vec![];
    for i in 0..ast.unroll_factor {
        iter_evals.push(syn::Ident::new_raw(
            &format!("iter_{}_of_{}", i + 1, ast.unroll_factor).to_owned(),
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
}

fn builder(s: &Pieces) -> proc_macro2::TokenStream {
    let label_outer = &s.label_outer;
    let pat = &s.pat;
    let expr = &s.expr;
    let body_stmts = &s.body_stmts;
    let remainder = &s.remainder;
    let label_inner = &s.label_inner;
    let iter = &s.iter;

    let mut prologue = quote! {};
    for curr_pat in s.iter_evals.iter().rev() {
        quote! {
            let mut #curr_pat = None;
        }
        .to_tokens(&mut prologue);
    }

    let mut new_body = quote! {};
    for curr_pat in &s.iter_evals {
        quote! {
            {
                let #pat = #curr_pat.take().unwrap();
                #(#body_stmts)*
            }
        }
        .to_tokens(&mut new_body);
    }

    for curr_pat in s.iter_evals.iter().rev() {
        new_body = quote! {
            #curr_pat = #iter.next();
            if #curr_pat.is_some() {
                #new_body
            } else {
                break #label_inner;
            }
        };
    }

    let mut epilogue = quote! {};
    for curr_pat in s.iter_evals.iter().rev().skip(1).rev() {
        quote! {
            if let Some(#pat) = #curr_pat.take() {
                #(#body_stmts)*
            } else {
                break #label_outer;
            }
        }
        .to_tokens(&mut epilogue);
    }

    quote! {
        #label_outer: while true {
            let mut #iter = #expr;
            #prologue
            #label_inner: while true {
                #new_body
            }
            #epilogue
            break #label_outer;
        }
        #remainder
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod tests;
