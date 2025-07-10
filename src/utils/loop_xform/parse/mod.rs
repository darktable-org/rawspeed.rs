use super::Item;
use super::LoopUnrollConf;
use super::UnrollMethod;
use super::kw;
use syn::Attribute;
use syn::LitInt;
use syn::Result;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;

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

#[allow(clippy::single_call_fn)]
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

#[allow(clippy::single_call_fn)]
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

#[allow(clippy::single_call_fn)]
fn parse_attr(attr: &Attribute) -> Result<(UnrollMethod, usize)> {
    if !attr.path().is_ident("loop_unroll") {
        return Err(syn::Error::new_spanned(
            attr,
            "`loop_unroll` attribute expected",
        ));
    }

    let mut unroll_method: Option<UnrollMethod> = None;
    let mut unroll_factor: Option<usize> = None;
    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("method") {
            return parse_method(&meta, &mut unroll_method);
        }
        if meta.path.is_ident("factor") {
            return parse_factor(&meta, &mut unroll_factor);
        }
        Err(meta.error(
            "unrecognized parameter, expected `method(...)` and `factor(..)`",
        ))
    })?;

    let Some(unroll_method) = unroll_method else {
        return Err(syn::Error::new_spanned(
            attr,
            "The attribute must specify unroll `method`",
        ));
    };

    let Some(unroll_factor) = unroll_factor else {
        return Err(syn::Error::new_spanned(
            attr,
            "The attribute must specify `factor`",
        ));
    };

    Ok((unroll_method, unroll_factor))
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let (unroll_method, unroll_factor);
        if let Some(attr) = attrs.first() {
            if let Some(ea) = attrs.get(1) {
                return Err(syn::Error::new_spanned(
                    ea,
                    "There should only be a single attribute",
                ));
            }

            (unroll_method, unroll_factor) = parse_attr(attr)?;
        } else {
            return Err(syn::Error::new_spanned(
                input.parse::<proc_macro2::TokenStream>()?,
                "There must be an attribute",
            ));
        }

        let for_loop = input.parse::<syn::ExprForLoop>()?;
        let remainder: proc_macro2::TokenStream = input.parse()?;
        assert!(input.is_empty());

        Ok(Item::LoopUnrollAttr(LoopUnrollConf {
            unroll_method,
            unroll_factor,
            for_loop,
            rest_of_tokenstream: remainder,
        }))
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod tests;
