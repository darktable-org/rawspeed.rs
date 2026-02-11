use core::ops::Deref;

use rawspeed_metadata_xmltokendesparsifier::xmltokendesparsifier::{
    Token, TokenStream,
};

pub type Result<T> = core::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseStream<'a> {
    inner: TokenStream<'a>,
}

pub trait Parse<'a, 'b>: Sized {
    fn parse(input: &'b mut ParseStream<'a>) -> Result<Self>;
}

impl<'a, 'b, T> Parse<'a, 'b> for Option<T>
where
    T: for<'c> Parse<'a, 'c>,
{
    #[inline]
    fn parse(input: &'b mut ParseStream<'a>) -> Result<Self> {
        Ok(input.parse::<T>().ok())
    }
}

impl<'a> ParseStream<'a> {
    #[must_use]
    #[inline]
    pub fn new(buf: &'a str) -> Self {
        Self {
            inner: TokenStream::new(buf),
        }
    }

    #[inline]
    pub fn parse<T: for<'b> Parse<'a, 'b>>(&mut self) -> Result<T> {
        let mut fork = self.clone();
        match T::parse(&mut fork) {
            Ok(res) => {
                *self = fork;
                Ok(res)
            }
            Err(err) => Err(err),
        }
    }
}

#[inline]
pub fn parse_str<'a, T: for<'b> Parse<'a, 'b>>(str: &'a str) -> Result<T> {
    ParseStream::new(str).parse::<T>()
}

macro_rules! impl_matcher {
    ($ident:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $ident<'a> {
            buf: &'a str,
        }

        impl<'a> $ident<'a> {
            #[must_use]
            #[inline]
            const fn new(buf: &'a str) -> Self {
                Self { buf }
            }
        }

        impl<'a> Deref for $ident<'a> {
            type Target = &'a str;
            #[inline]
            fn deref(&self) -> &<Self as core::ops::Deref>::Target { &self.buf }
        }

        impl<'a, 'b> Parse<'a, 'b> for $ident<'a> {
            #[inline]
            fn parse(input: &'b mut ParseStream<'a>) -> Result<Self> {
                let token = input.inner.next();
                match token {
                    Some(Token::$ident(buf)) => Ok(Self::new(buf)),
                    None => Err(format!(
                        "While trying to match `{:?}`, encountered end of stream", stringify!($ident),
                    )),
                    Some(other) => Err(format!(
                        "While trying to match `{:?}`, but the following was encountered instead: `{:?}`",
                        stringify!($ident),
                        other
                    )),
                }
            }
        }
    };
}

impl_matcher!(ElementContentVerbatim);
impl_matcher!(Gt);
impl_matcher!(Lt);
impl_matcher!(ElementName);
impl_matcher!(ElementSlash);
impl_matcher!(ElementAttributeName);
impl_matcher!(ElementAttributeEq);
impl_matcher!(ElementAttributeValue);
impl_matcher!(Garbage);

#[cfg(test)]
mod tests;
