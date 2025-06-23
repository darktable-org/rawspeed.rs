use super::super::LoopUnrollConf;
use quote::quote;

#[allow(clippy::single_call_fn)]
pub fn transform(ast: &LoopUnrollConf) -> proc_macro2::TokenStream {
    let label = &ast.for_loop.label;
    let pat = &ast.for_loop.pat;
    let expr = &ast.for_loop.expr;
    let body_stmts = &ast.for_loop.body.stmts;
    let remainder = &ast.rest_of_tokenstream;

    let iter = syn::Ident::new_raw("iter", proc_macro2::Span::mixed_site());

    let new_body = core::iter::repeat_with(|| {
        quote! {
            if let Some(#pat) = #iter.next() {
                #(#body_stmts)*
            } else {
                break;
            }
        }
    })
    .take(ast.unroll_factor);

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

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod tests;
