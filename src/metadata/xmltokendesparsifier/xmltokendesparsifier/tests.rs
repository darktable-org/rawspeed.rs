use super::*;

#[test]
fn without_whitespaces_test() {
    let inputs: Vec<&str> = vec![
        " prefix <outer attr0=\" va'l 0 \" attr1=' va\"l 1 '> inner </outer><something> suffix ",
        " prefix < outer attr0 = \" va'l 0 \" attr1 = ' va\"l 1 ' > inner < / outer > < something > suffix ",
    ];
    let expected: Vec<Option<Token<'static>>> = vec![
        Some(Token::Garbage(" prefix ")),
        Some(Token::Lt("<")),
        Some(Token::ElementName("outer")),
        Some(Token::ElementAttributeName("attr0")),
        Some(Token::ElementAttributeEq("=")),
        Some(Token::ElementAttributeValue(" va'l 0 ")),
        Some(Token::ElementAttributeName("attr1")),
        Some(Token::ElementAttributeEq("=")),
        Some(Token::ElementAttributeValue(" va\"l 1 ")),
        Some(Token::Gt(">")),
        Some(Token::ElementContentVerbatim(" inner ")),
        Some(Token::Lt("<")),
        Some(Token::ElementSlash("/")),
        Some(Token::ElementName("outer")),
        Some(Token::Gt(">")),
        Some(Token::Lt("<")),
        Some(Token::ElementName("something")),
        Some(Token::Gt(">")),
        Some(Token::Garbage(" suffix ")),
        None,
    ];
    for input in inputs {
        let mut res = vec![];
        let mut iter = TokenStream::new(input);
        loop {
            let iter_val = iter.next();
            res.push(iter_val);
            if iter_val.is_none() {
                break;
            }
        }
        assert_eq!(res, expected);
    }
}

#[test]
fn initial_state_test() {
    let input = " <outer>";
    let expected = "TokenStream { inner: TokenStream { buf: \"<outer>\", prev_tok: LeadingWhitespace } }";
    let iter = TokenStream::new(input);
    let res = format!("{iter:?}");
    assert_eq!(res, expected);
}

#[test]
fn autoskip_test() {
    let input = "<outer> <something>";
    let mut iter = TokenStream::new(input);
    assert_eq!(
        format!("{iter:?}"),
        "TokenStream { inner: TokenStream { buf: \"<outer> <something>\", prev_tok: StartOfTokenStream } }"
    );
    iter.next();
    assert_eq!(
        format!("{iter:?}"),
        "TokenStream { inner: TokenStream { buf: \"outer> <something>\", prev_tok: Lt } }"
    );
    iter.next();
    assert_eq!(
        format!("{iter:?}"),
        "TokenStream { inner: TokenStream { buf: \"> <something>\", prev_tok: ElementStartName } }"
    );
    iter.next();
    assert_eq!(
        format!("{iter:?}"),
        "TokenStream { inner: TokenStream { buf: \"<something>\", prev_tok: WhitespaceAfterGt } }"
    );
}
