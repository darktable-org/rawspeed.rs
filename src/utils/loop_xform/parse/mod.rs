use crate::{
    Loop, LoopPeelParams, LoopSplitParams, LoopUnrollParams, LoopXFormParams,
};

use super::Item;
use super::UnrollMethod;
use super::kw;
use syn::Result;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::{Attribute, ExprForLoop};
use syn::{LitInt, Token, punctuated::Punctuated};

impl Parse for UnrollMethod {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::runtime) {
            input.parse::<kw::runtime>()?;
            Ok(UnrollMethod::Runtime)
        } else if lookahead.peek(kw::with_remainder) {
            input.parse::<kw::with_remainder>()?;
            Ok(UnrollMethod::WithRemainder)
        } else {
            Err(lookahead.error())
        }
    }
}

fn parse_method(
    meta: &syn::meta::ParseNestedMeta<'_>,
    unroll_method: &mut Option<UnrollMethod>,
) -> Result<()> {
    assert!(meta.path.is_ident("method"));

    if unroll_method.is_some() {
        return Err(
            meta.error("only a single unroll method shall be specified")
        );
    }

    let content;
    parenthesized!(content in meta.input);
    let head = content.fork();
    match content.parse::<UnrollMethod>() {
        Ok(m) => *unroll_method = Some(m),
        Err(_) => {
            return Err(head.error("expected valid unroll method"));
        }
    }
    if !content.is_empty() {
        return Err(syn::Error::new_spanned(
            content.parse::<proc_macro2::TokenStream>()?,
            "unexpected garbage in unroll method argument",
        ));
    }
    Ok(())
}

fn parse_factor(
    meta: &syn::meta::ParseNestedMeta<'_>,
    unroll_factor: &mut Option<usize>,
) -> Result<()> {
    assert!(meta.path.is_ident("factor"));

    if unroll_factor.is_some() {
        return Err(
            meta.error("only a single unroll factor shall be specified")
        );
    }
    let content;
    parenthesized!(content in meta.input);
    let lit: LitInt = content.parse()?;
    if !lit.suffix().is_empty() {
        return Err(syn::Error::new_spanned(
            lit,
            "unroll factor should not have any suffix",
        ));
    }
    if !content.is_empty() {
        return Err(syn::Error::new_spanned(
            content.parse::<proc_macro2::TokenStream>()?,
            "unexpected garbage in unroll factor argument",
        ));
    }
    let n: usize = lit.base10_parse()?;
    if n < 1 {
        return Err(meta.error("Unroll factor can not be zero"));
    }
    *unroll_factor = Some(n);
    Ok(())
}

fn parse_unroll_attr(attr: &Attribute) -> Option<Result<LoopUnrollParams>> {
    if !attr.path().is_ident("loop_unroll") {
        return None;
    }

    let mut unroll_method: Option<UnrollMethod> = None;
    let mut unroll_factor: Option<usize> = None;
    if let Err(err) = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("method") {
            return parse_method(&meta, &mut unroll_method);
        }
        if meta.path.is_ident("factor") {
            return parse_factor(&meta, &mut unroll_factor);
        }
        Err(meta.error(
            "unrecognized parameter, expected `method(...)` and `factor(..)`",
        ))
    }) {
        return Some(Err(err));
    }

    let Some(unroll_method) = unroll_method else {
        return Some(Err(syn::Error::new_spanned(
            attr,
            "The attribute must specify unroll `method`",
        )));
    };

    let Some(unroll_factor) = unroll_factor else {
        return Some(Err(syn::Error::new_spanned(
            attr,
            "The attribute must specify `factor`",
        )));
    };

    Some(Ok(LoopUnrollParams {
        unroll_method,
        unroll_factor,
    }))
}

struct PeelCount(usize);
impl syn::parse::Parse for PeelCount {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let Ok(lit) = input.parse::<LitInt>() else {
            return Err(syn::Error::new_spanned(
                input.parse::<proc_macro2::TokenStream>()?,
                "The attribute must specify peel count",
            ));
        };
        if !lit.suffix().is_empty() {
            return Err(syn::Error::new_spanned(
                lit,
                "Peel count should not have any suffix",
            ));
        }
        if !input.is_empty() {
            return Err(syn::Error::new_spanned(
                input.parse::<proc_macro2::TokenStream>()?,
                "unexpected garbage in peel count argument",
            ));
        }
        let n: usize = lit.base10_parse()?;
        if n < 1 {
            return Err(input.error("Peel count can not be zero"));
        }
        Ok(Self(n))
    }
}

fn parse_peel_attr(attr: &Attribute) -> Option<Result<LoopPeelParams>> {
    if !attr.path().is_ident("loop_peel") {
        return None;
    }

    match attr.parse_args::<PeelCount>() {
        Ok(PeelCount(peel_count)) => Some(Ok(LoopPeelParams { peel_count })),
        Err(err) => Some(Err(err)),
    }
}

struct SplitCounts(Punctuated<proc_macro2::TokenTree, Token![,]>);
impl syn::parse::Parse for SplitCounts {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}

fn parse_split_attr(attr: &Attribute) -> Option<Result<LoopSplitParams>> {
    if !attr.path().is_ident("loop_split") {
        return None;
    }

    match attr.parse_args::<SplitCounts>() {
        Ok(SplitCounts(split_lens)) => Some(Ok(LoopSplitParams { split_lens })),
        Err(err) => Some(Err(err)),
    }
}

fn parse_attr(attr: &Attribute) -> Result<LoopXFormParams> {
    if let Some(p) = parse_unroll_attr(attr) {
        return Ok(p?.into());
    }

    if let Some(p) = parse_peel_attr(attr) {
        return Ok(p?.into());
    }

    if let Some(p) = parse_split_attr(attr) {
        return Ok(p?.into());
    }

    Err(syn::Error::new_spanned(
        attr,
        "`loop_unroll` or `loop_peel` attribute expected",
    ))
}

impl Parse for Loop {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let origin = input.fork();
        match input.parse::<syn::Expr>() {
            Ok(syn::Expr::ForLoop(expr_for_loop)) => Ok(expr_for_loop.into()),
            Ok(syn::Expr::Loop(expr_loop)) => Ok(expr_loop.into()),
            Ok(syn::Expr::While(expr_while)) => Ok(expr_while.into()),
            _ => Err(origin.error("expected some kind of loop")),
        }
    }
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let params = if let Some(attr) = attrs.first() {
            if let Some(ea) = attrs.get(1) {
                return Err(syn::Error::new_spanned(
                    ea,
                    "There should only be a single attribute",
                ));
            }

            parse_attr(attr)?
        } else {
            return Err(syn::Error::new_spanned(
                input.parse::<proc_macro2::TokenStream>()?,
                "There must be an attribute",
            ));
        };

        let the_loop = input.parse()?;
        let remainder = input.parse()?;
        assert!(input.is_empty());

        Ok(Self::new(params, the_loop, remainder))
    }
}

#[cfg(test)]
mod tests;
