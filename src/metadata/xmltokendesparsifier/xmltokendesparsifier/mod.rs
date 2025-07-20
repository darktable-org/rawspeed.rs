use rawspeed_metadata_xmltokenizer::xmltokenizer;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Token<'a> {
    ElementContentVerbatim(&'a str),
    Gt(&'a str),
    Lt(&'a str),
    ElementName(&'a str),
    ElementSlash(&'a str),
    ElementAttributeName(&'a str),
    ElementAttributeEq(&'a str),
    ElementAttributeValue(&'a str),
    Garbage(&'a str),
}

impl<'a> Token<'a> {
    const fn new(tok: xmltokenizer::TokenTy, buf: &'a str) -> Self {
        #[expect(clippy::wildcard_enum_match_arm)]
        match tok {
            xmltokenizer::TokenTy::LeadingGarbage
            | xmltokenizer::TokenTy::TrailingGarbage => Token::Garbage(buf),
            xmltokenizer::TokenTy::ElementContentVerbatim => {
                Token::ElementContentVerbatim(buf)
            }
            xmltokenizer::TokenTy::ElementStartName
            | xmltokenizer::TokenTy::ElementEndName => Token::ElementName(buf),
            xmltokenizer::TokenTy::ElementAttributeName => {
                Token::ElementAttributeName(buf)
            }
            xmltokenizer::TokenTy::ElementAttributeValue => {
                Token::ElementAttributeValue(buf)
            }
            xmltokenizer::TokenTy::Lt => {
                assert!(buf.len() == 1);
                Token::Lt(buf)
            }
            xmltokenizer::TokenTy::Gt => {
                assert!(buf.len() == 1);
                Token::Gt(buf)
            }
            xmltokenizer::TokenTy::ElementLeadingSlash
            | xmltokenizer::TokenTy::ElementTrailingSlash => {
                assert!(buf.len() == 1);
                Token::ElementSlash(buf)
            }
            xmltokenizer::TokenTy::ElementAttributeEq => {
                assert!(buf.len() == 1);
                Token::ElementAttributeEq(buf)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream<'a> {
    inner: xmltokenizer::TokenStream<'a>,
}

impl<'a> TokenStream<'a> {
    #[must_use]
    #[inline]
    pub fn new(buf: &'a str) -> Self {
        let mut this = Self {
            inner: xmltokenizer::TokenStream::new(buf),
        };
        this.advance_to_nonwhitespace_tok();
        this
    }

    #[inline]
    fn advance_to_nonwhitespace_tok(&mut self) {
        loop {
            let mut fork = self.clone();
            match fork.inner.next() {
                None => return,
                Some(tok) if !tok.tok.is_whitespace() => return,
                Some(_) => *self = fork,
            }
        }
    }
}

#[expect(clippy::missing_trait_methods)]
impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.inner.next()?;
        assert!(!tok.tok.is_whitespace());
        self.advance_to_nonwhitespace_tok();
        let (tok, mut tok_buf) = (tok.tok, tok.buf);
        assert!(!tok_buf.is_empty());
        if tok == xmltokenizer::TokenTy::ElementAttributeValue {
            tok_buf = match tok_buf.get(1..tok_buf.len() - 1) {
                Some(tok_buf) => tok_buf,
                None => unreachable!(),
            }
        }
        Some(Token::new(tok, tok_buf))
    }
}

#[cfg(test)]
mod tests;
