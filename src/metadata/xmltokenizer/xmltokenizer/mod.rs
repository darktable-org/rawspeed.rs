#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum TokenTy {
    TrailingGarbage = -2,
    TrailingWhitespace = -1,

    StartOfTokenStream = 0,

    ElementContentVerbatim,

    LeadingGarbage,
    LeadingWhitespace,

    Gt,
    WhitespaceAfterGt,

    Lt,
    ElementWhitespaceAfterLt,

    ElementLeadingSlash,
    ElementWhitespaceAfterLeadingSlash,

    ElementEndName,
    ElementWhitespaceAfterEndName,

    ElementStartName,
    ElementWhitespaceAfterStartName,

    ElementTrailingSlash,
    ElementWhitespaceAfterTrailingSlash,

    ElementAttributeName,
    ElementWhitespaceAfterAttributeName,

    ElementAttributeEq,
    ElementWhitespaceAfterAttributeEq,

    ElementAttributeValue,
    ElementWhitespaceAfterAttributeValue,
}

impl TokenTy {
    #[must_use]
    #[inline]
    const fn new(ch: char) -> TokenTy {
        match ch {
            '<' => TokenTy::Lt,
            '>' => TokenTy::Gt,
            _ => unreachable!(),
        }
    }

    #[must_use]
    #[inline]
    const fn with_whitespace(&self) -> TokenTy {
        #[expect(clippy::wildcard_enum_match_arm)]
        match *self {
            TokenTy::LeadingGarbage => TokenTy::LeadingWhitespace,
            TokenTy::TrailingGarbage => TokenTy::TrailingWhitespace,
            TokenTy::ElementContentVerbatim => TokenTy::WhitespaceAfterGt,
            TokenTy::Lt => TokenTy::ElementWhitespaceAfterLt,
            TokenTy::ElementLeadingSlash => {
                TokenTy::ElementWhitespaceAfterLeadingSlash
            }
            TokenTy::ElementEndName => TokenTy::ElementWhitespaceAfterEndName,
            TokenTy::ElementStartName => {
                TokenTy::ElementWhitespaceAfterStartName
            }
            TokenTy::ElementTrailingSlash => {
                TokenTy::ElementWhitespaceAfterTrailingSlash
            }
            TokenTy::ElementAttributeName => {
                TokenTy::ElementWhitespaceAfterAttributeName
            }
            TokenTy::ElementAttributeEq => {
                TokenTy::ElementWhitespaceAfterAttributeEq
            }
            TokenTy::ElementAttributeValue => {
                TokenTy::ElementWhitespaceAfterAttributeValue
            }
            _ => unreachable!(),
        }
    }

    #[must_use]
    #[inline]
    const fn basename(&self) -> TokenTy {
        #[expect(clippy::wildcard_enum_match_arm)]
        match *self {
            TokenTy::LeadingWhitespace => TokenTy::LeadingGarbage,
            TokenTy::TrailingWhitespace => TokenTy::TrailingGarbage,
            TokenTy::WhitespaceAfterGt => TokenTy::ElementContentVerbatim,
            TokenTy::ElementWhitespaceAfterLt => TokenTy::Lt,
            TokenTy::ElementWhitespaceAfterLeadingSlash => {
                TokenTy::ElementLeadingSlash
            }
            TokenTy::ElementWhitespaceAfterEndName => TokenTy::ElementEndName,
            TokenTy::ElementWhitespaceAfterStartName => {
                TokenTy::ElementStartName
            }
            TokenTy::ElementWhitespaceAfterTrailingSlash => {
                TokenTy::ElementTrailingSlash
            }
            TokenTy::ElementWhitespaceAfterAttributeName => {
                TokenTy::ElementAttributeName
            }
            TokenTy::ElementWhitespaceAfterAttributeEq => {
                TokenTy::ElementAttributeEq
            }
            TokenTy::ElementWhitespaceAfterAttributeValue => {
                TokenTy::ElementAttributeValue
            }
            t => t,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_whitespace(&self) -> bool {
        *self != self.basename()
    }
}

impl From<TokenTy> for char {
    #[inline]
    fn from(other: TokenTy) -> Self {
        #[expect(clippy::wildcard_enum_match_arm)]
        match other {
            TokenTy::Lt => '<',
            TokenTy::Gt => '>',
            TokenTy::ElementLeadingSlash | TokenTy::ElementTrailingSlash => '/',
            _ => unreachable!(),
        }
    }
}

#[must_use]
#[inline]
const fn is_xml_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' ')
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Token<'a> {
    pub tok: TokenTy,
    pub buf: &'a str,
}

impl<'a> Token<'a> {
    #[must_use]
    #[inline]
    const fn new(tok: TokenTy, buf: &'a str) -> Self {
        Self { tok, buf }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream<'a> {
    buf: &'a str,
    prev_tok: TokenTy,
}

impl<'a> TokenStream<'a> {
    #[must_use]
    #[inline]
    pub const fn new(buf: &'a str) -> Self {
        Self {
            buf,
            prev_tok: TokenTy::StartOfTokenStream,
        }
    }

    #[must_use]
    #[inline]
    #[expect(clippy::wildcard_enum_match_arm)]
    fn peek_lt_tok_impl(&self) -> (TokenTy, usize) {
        let next_tok = TokenTy::new('<');
        let next_char_is_tok = Some(next_tok.into()) == self.buf.chars().nth(0);
        match self.prev_tok {
            TokenTy::LeadingGarbage
            | TokenTy::LeadingWhitespace
            | TokenTy::ElementContentVerbatim
            | TokenTy::WhitespaceAfterGt => {
                assert!(next_char_is_tok);
                return (next_tok, 1);
            }
            TokenTy::StartOfTokenStream | TokenTy::Gt if next_char_is_tok => {
                return (next_tok, 1);
            }
            TokenTy::StartOfTokenStream | TokenTy::Gt => {}
            _ => unreachable!(),
        }

        match self.buf.find::<char>(next_tok.into()) {
            Some(0) => unreachable!(),
            Some(next_tok_pos) => {
                let tok = match self.prev_tok {
                    TokenTy::StartOfTokenStream => TokenTy::LeadingGarbage,
                    TokenTy::Gt => TokenTy::ElementContentVerbatim,
                    _ => unreachable!(),
                };
                (tok, next_tok_pos)
            }
            None => (TokenTy::TrailingGarbage, self.buf.len()),
        }
    }

    #[must_use]
    #[inline]
    #[expect(clippy::wildcard_enum_match_arm)]
    fn peek_lt_tok(&self) -> (TokenTy, usize) {
        let (tok, tok_end) = match self.prev_tok {
            TokenTy::StartOfTokenStream
            | TokenTy::ElementContentVerbatim
            | TokenTy::LeadingGarbage
            | TokenTy::LeadingWhitespace
            | TokenTy::Gt
            | TokenTy::WhitespaceAfterGt => self.peek_lt_tok_impl(),
            _ => unreachable!(),
        };

        match tok {
            TokenTy::Lt => return (tok, tok_end),
            TokenTy::TrailingGarbage if tok_end == 0 => {
                assert!(tok_end == self.buf.len());
                return (tok, tok_end);
            }
            TokenTy::ElementContentVerbatim
            | TokenTy::LeadingGarbage
            | TokenTy::TrailingGarbage
                if tok_end != 0 => {}
            _ => unreachable!(),
        }

        match (self.buf.find(|c: char| !is_xml_whitespace(c)), tok) {
            (Some(0), TokenTy::TrailingGarbage) => return (tok, tok_end),
            (Some(whitespace_end), TokenTy::ElementContentVerbatim)
                if whitespace_end != tok_end =>
            {
                return (tok, tok_end);
            }
            (Some(0), _) => unreachable!(),
            (Some(whitespace_end), _) if whitespace_end == tok_end => {}
            (None, TokenTy::TrailingGarbage) => {}
            (Some(_), _) => return (tok, tok_end),
            (None, _) => unreachable!(),
        }
        (tok.with_whitespace(), tok_end)
    }

    #[must_use]
    #[inline]
    #[expect(
        clippy::wildcard_enum_match_arm,
        clippy::too_many_lines,
        clippy::cognitive_complexity
    )]
    fn peek_in_element_non_whitespace(&self) -> (TokenTy, usize) {
        assert!(matches!(
            self.prev_tok,
            TokenTy::Lt
                | TokenTy::ElementWhitespaceAfterLt
                | TokenTy::ElementLeadingSlash
                | TokenTy::ElementWhitespaceAfterLeadingSlash
                | TokenTy::ElementEndName
                | TokenTy::ElementWhitespaceAfterEndName
                | TokenTy::ElementStartName
                | TokenTy::ElementWhitespaceAfterStartName
                | TokenTy::ElementTrailingSlash
                | TokenTy::ElementWhitespaceAfterTrailingSlash
                | TokenTy::ElementAttributeName
                | TokenTy::ElementWhitespaceAfterAttributeName
                | TokenTy::ElementAttributeEq
                | TokenTy::ElementWhitespaceAfterAttributeEq
                | TokenTy::ElementAttributeValue
                | TokenTy::ElementWhitespaceAfterAttributeValue
        ));
        assert!(Some(0) == self.buf.find(|c: char| !is_xml_whitespace(c)));

        match self.prev_tok {
            TokenTy::Lt | TokenTy::ElementWhitespaceAfterLt
                if self.buf.starts_with('/') =>
            {
                (TokenTy::ElementLeadingSlash, 1)
            }

            TokenTy::Lt
            | TokenTy::ElementWhitespaceAfterLt
            | TokenTy::ElementLeadingSlash
            | TokenTy::ElementWhitespaceAfterLeadingSlash => {
                match self.buf.find(|c: char| match c {
                    '/' | '>' => true,
                    _ => is_xml_whitespace(c),
                }) {
                    Some(0) | None => {
                        (TokenTy::TrailingGarbage, self.buf.len())
                    }
                    Some(tok_end) => match self.prev_tok {
                        TokenTy::Lt | TokenTy::ElementWhitespaceAfterLt => {
                            (TokenTy::ElementStartName, tok_end)
                        }
                        TokenTy::ElementLeadingSlash
                        | TokenTy::ElementWhitespaceAfterLeadingSlash => {
                            (TokenTy::ElementEndName, tok_end)
                        }
                        _ => unreachable!(),
                    },
                }
            }

            TokenTy::ElementEndName
            | TokenTy::ElementWhitespaceAfterEndName
            | TokenTy::ElementTrailingSlash
            | TokenTy::ElementWhitespaceAfterTrailingSlash => {
                match self.buf.chars().next() {
                    Some('>') => (TokenTy::Gt, 1),
                    _ => (TokenTy::TrailingGarbage, self.buf.len()),
                }
            }

            TokenTy::ElementStartName
            | TokenTy::ElementWhitespaceAfterStartName
            | TokenTy::ElementAttributeValue
            | TokenTy::ElementWhitespaceAfterAttributeValue => {
                match self.buf.chars().next() {
                    Some('/') => (TokenTy::ElementTrailingSlash, 1),
                    Some('>') => (TokenTy::Gt, 1),
                    None => (TokenTy::TrailingGarbage, self.buf.len()),
                    _ => {
                        match self.buf.find(|c: char| match c {
                            '=' => true,
                            _ => is_xml_whitespace(c),
                        }) {
                            Some(0) | None => {
                                (TokenTy::TrailingGarbage, self.buf.len())
                            }
                            Some(tok_end) => {
                                (TokenTy::ElementAttributeName, tok_end)
                            }
                        }
                    }
                }
            }

            TokenTy::ElementAttributeName
            | TokenTy::ElementWhitespaceAfterAttributeName => {
                match self.buf.chars().next() {
                    Some('=') => (TokenTy::ElementAttributeEq, 1),
                    _ => (TokenTy::TrailingGarbage, self.buf.len()),
                }
            }

            TokenTy::ElementAttributeEq
            | TokenTy::ElementWhitespaceAfterAttributeEq => {
                let quote = match self.buf.chars().next() {
                    Some(c) if (c == '"' || c == '\'') => c,
                    _ => {
                        return (TokenTy::TrailingGarbage, self.buf.len());
                    }
                };
                match self.buf.strip_prefix(quote).and_then(|s| s.find(quote)) {
                    None => (TokenTy::TrailingGarbage, self.buf.len()),
                    Some(tok_end) => {
                        (TokenTy::ElementAttributeValue, 2 + tok_end)
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    #[must_use]
    #[inline]
    #[expect(clippy::wildcard_enum_match_arm)]
    fn peek_in_element(&self) -> (TokenTy, usize) {
        match self.prev_tok {
            TokenTy::Lt
            | TokenTy::ElementLeadingSlash
            | TokenTy::ElementEndName
            | TokenTy::ElementStartName
            | TokenTy::ElementTrailingSlash
            | TokenTy::ElementAttributeName
            | TokenTy::ElementAttributeEq
            | TokenTy::ElementAttributeValue => {}

            TokenTy::ElementWhitespaceAfterLt
            | TokenTy::ElementWhitespaceAfterLeadingSlash
            | TokenTy::ElementWhitespaceAfterEndName
            | TokenTy::ElementWhitespaceAfterStartName
            | TokenTy::ElementWhitespaceAfterTrailingSlash
            | TokenTy::ElementWhitespaceAfterAttributeName
            | TokenTy::ElementWhitespaceAfterAttributeEq
            | TokenTy::ElementWhitespaceAfterAttributeValue => {
                return self.peek_in_element_non_whitespace();
            }

            _ => unreachable!(),
        }

        match self.buf.find(|c: char| !is_xml_whitespace(c)) {
            Some(0) => self.peek_in_element_non_whitespace(),
            Some(tok_end) => (self.prev_tok.with_whitespace(), tok_end),
            None => (TokenTy::TrailingGarbage, self.buf.len()),
        }
    }

    #[must_use]
    #[inline]
    fn peek_tok(&self) -> Option<(TokenTy, usize)> {
        let tok = match self.prev_tok {
            TokenTy::StartOfTokenStream
            | TokenTy::ElementContentVerbatim
            | TokenTy::LeadingGarbage
            | TokenTy::LeadingWhitespace
            | TokenTy::Gt
            | TokenTy::WhitespaceAfterGt => self.peek_lt_tok(),

            TokenTy::Lt
            | TokenTy::ElementWhitespaceAfterLt
            | TokenTy::ElementLeadingSlash
            | TokenTy::ElementWhitespaceAfterLeadingSlash
            | TokenTy::ElementEndName
            | TokenTy::ElementWhitespaceAfterEndName
            | TokenTy::ElementStartName
            | TokenTy::ElementWhitespaceAfterStartName
            | TokenTy::ElementTrailingSlash
            | TokenTy::ElementWhitespaceAfterTrailingSlash
            | TokenTy::ElementAttributeName
            | TokenTy::ElementWhitespaceAfterAttributeName
            | TokenTy::ElementAttributeEq
            | TokenTy::ElementWhitespaceAfterAttributeEq
            | TokenTy::ElementAttributeValue
            | TokenTy::ElementWhitespaceAfterAttributeValue => {
                self.peek_in_element()
            }

            TokenTy::TrailingGarbage | TokenTy::TrailingWhitespace => {
                assert!(self.buf.is_empty());
                return None;
            }
        };
        assert!(!self.prev_tok.is_whitespace() || !tok.0.is_whitespace());
        if let (TokenTy::TrailingGarbage, tok_end) = tok {
            assert!(tok_end == self.buf.len());
            if self.buf.is_empty() {
                return None;
            }
        }
        Some(tok)
    }
}

#[expect(clippy::missing_trait_methods)]
impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (tok, tok_end) = self.peek_tok()?;
        let (tok_str, rest) = self.buf.split_at(tok_end);
        assert!(!tok_str.is_empty());
        self.buf = rest;
        self.prev_tok = tok;
        Some(Token::new(tok, tok_str))
    }
}

#[cfg(test)]
#[allow(clippy::allow_attributes, clippy::large_stack_frames)]
mod tests;
