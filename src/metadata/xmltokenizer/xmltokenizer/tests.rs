use super::*;

#[test]
#[expect(clippy::too_many_lines)]
fn tokenization_basic_test() {
    let inputs: Vec<&str> = vec![
        "", " ", "<", " <", "< ", " < ", ">", " >", "> ", " > ", "<>", " <>",
        "< >", "<> ", " < >", " <> ", " < > ",
    ];
    let expected: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> = vec![
        vec![(
            TokenStream {
                buf: "",
                prev_tok: TokenTy::StartOfTokenStream,
            },
            None,
        )],
        vec![
            (
                TokenStream {
                    buf: " ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::TrailingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingWhitespace,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::Lt,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " <",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "<",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::Lt,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " < ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "< ",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " >",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " >",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "> ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "> ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " > ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " > ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<>",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " <>",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "<>",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< >",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " >",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<> ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "> ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "> ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " < >",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "< >",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " >",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " <> ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "<> ",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "> ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "> ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " < > ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "< > ",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " > ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "> ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
    ];
    let mut res: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> =
        vec![];
    for input in inputs {
        let mut output = vec![];
        let mut iter = TokenStream::new(input);
        let mut reconstructed = String::new();
        loop {
            let iter_clone = iter.clone();
            let iter_val = iter.next();
            output.push((iter_clone, iter_val));
            match iter_val {
                Some(tok) => reconstructed.push_str(tok.buf),
                None => break,
            }
        }
        assert_eq!(reconstructed, input);
        res.push(output);
    }
    assert_eq!(res, expected);
}

#[test]
#[expect(clippy::too_many_lines)]
fn tokenization_slashes_test() {
    let inputs: Vec<&str> = vec![
        "",
        "<inner> suffix ",
        "<inner> data verbatim <garbage",
        "< inner > suffix ",
        "< inner > data verbatim < garbage",
        "</inner> suffix ",
        "</inner> data verbatim </garbage",
        "< / inner > suffix ",
        "< / inner > data verbatim < / garbage",
        "<inner/> suffix ",
        "<inner/> data verbatim <garbage/",
        "< inner / > suffix ",
        "< inner / > data verbatim < garbage /",
        "</inner/> suffix ",
        "</inner/> data verbatim </garbage/",
        "< / inner / > suffix ",
        "< / inner / > data verbatim < / garbage / ",
    ];
    let expected: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> = vec![
        vec![(
            TokenStream {
                buf: "",
                prev_tok: TokenTy::StartOfTokenStream,
            },
            None,
        )],
        vec![
            (
                TokenStream {
                    buf: "<inner> suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "inner> suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<inner> data verbatim <garbage",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "inner> data verbatim <garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim <garbage",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim <garbage",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "<garbage",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< inner > suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " inner > suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " > suffix ",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< inner > data verbatim < garbage",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " inner > data verbatim < garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner > data verbatim < garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " > data verbatim < garbage",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim < garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim < garbage",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "< garbage",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "</inner> suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/inner> suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "inner> suffix ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "</inner> data verbatim </garbage",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/inner> data verbatim </garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "inner> data verbatim </garbage",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim </garbage",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim </garbage",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "</garbage",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< / inner > suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / inner > suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ inner > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " inner > suffix ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " > suffix ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< / inner > data verbatim < / garbage",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / inner > data verbatim < / garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ inner > data verbatim < / garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " inner > data verbatim < / garbage",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner > data verbatim < / garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " > data verbatim < / garbage",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim < / garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim < / garbage",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "< / garbage",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / garbage",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " garbage",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<inner/> suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "inner/> suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "/> suffix ",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "<inner/> data verbatim <garbage/",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "inner/> data verbatim <garbage/",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "/> data verbatim <garbage/",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim <garbage/",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim <garbage/",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "<garbage/",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage/",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: "/",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< inner / > suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " inner / > suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner / > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " / > suffix ",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " > suffix ",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterTrailingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< inner / > data verbatim < garbage /",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " inner / > data verbatim < garbage /",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner / > data verbatim < garbage /",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " / > data verbatim < garbage /",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ > data verbatim < garbage /",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " > data verbatim < garbage /",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterTrailingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> data verbatim < garbage /",
                    prev_tok: TokenTy::ElementWhitespaceAfterTrailingSlash,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " data verbatim < garbage /",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " data verbatim ",
                }),
            ),
            (
                TokenStream {
                    buf: "< garbage /",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " garbage /",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "garbage /",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "garbage",
                }),
            ),
            (
                TokenStream {
                    buf: " /",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementTrailingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::ElementTrailingSlash,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "</inner/> suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/inner/> suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "inner/> suffix ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "/> suffix ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "/> suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "</inner/> data verbatim </garbage/",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/inner/> data verbatim </garbage/",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "inner/> data verbatim </garbage/",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "/> data verbatim </garbage/",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "/> data verbatim </garbage/",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< / inner / > suffix ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / inner / > suffix ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ inner / > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " inner / > suffix ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner / > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " / > suffix ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ > suffix ",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "/ > suffix ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: "< / inner / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / inner / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ inner / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " inner / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "inner / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: " / > data verbatim < / garbage / ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ > data verbatim < / garbage / ",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: "/ > data verbatim < / garbage / ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
    ];
    let mut res: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> =
        vec![];
    for input in inputs {
        let mut output = vec![];
        let mut iter = TokenStream::new(input);
        let mut reconstructed = String::new();
        loop {
            let iter_clone = iter.clone();
            let iter_val = iter.next();
            output.push((iter_clone, iter_val));
            match iter_val {
                Some(tok) => reconstructed.push_str(tok.buf),
                None => break,
            }
        }
        assert_eq!(reconstructed, input);
        res.push(output);
    }
    assert_eq!(res, expected);
}

#[test]
#[expect(clippy::too_many_lines)]
fn tokenization_attributes_test() {
    let inputs: Vec<&str> = vec![
        "< outermost attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
    ];
    let expected: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> = vec![
        vec![
            (
                TokenStream {
                    buf: "< outermost attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " outermost attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outermost attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "outermost",
                }),
            ),
            (
                TokenStream {
                    buf: " attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "attr0 = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeName,
                    buf: "attr0",
                }),
            ),
            (
                TokenStream {
                    buf: " = \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "= \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeEq,
                    buf: "=",
                }),
            ),
            (
                TokenStream {
                    buf: " \" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "\" val0 \" attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeValue,
                    buf: "\" val0 \"",
                }),
            ),
            (
                TokenStream {
                    buf: " attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "attr1 = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeName,
                    buf: "attr1",
                }),
            ),
            (
                TokenStream {
                    buf: " = ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "= ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeEq,
                    buf: "=",
                }),
            ),
            (
                TokenStream {
                    buf: " ' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "' val1 ' attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeValue,
                    buf: "' val1 '",
                }),
            ),
            (
                TokenStream {
                    buf: " attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "attr2 = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeName,
                    buf: "attr2",
                }),
            ),
            (
                TokenStream {
                    buf: " = \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "= \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeEq,
                    buf: "=",
                }),
            ),
            (
                TokenStream {
                    buf: " \" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "\" val'2 \" ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeValue,
                    buf: "\" val'2 \"",
                }),
            ),
            (
                TokenStream {
                    buf: " ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "ttr3 = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeName,
                    buf: "ttr3",
                }),
            ),
            (
                TokenStream {
                    buf: " = ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "= ' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeName,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeEq,
                    buf: "=",
                }),
            ),
            (
                TokenStream {
                    buf: " ' val\"3 ' >",
                    prev_tok: TokenTy::ElementAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "' val\"3 ' >",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeEq,
                },
                Some(Token {
                    tok: TokenTy::ElementAttributeValue,
                    buf: "' val\"3 '",
                }),
            ),
            (
                TokenStream {
                    buf: " >",
                    prev_tok: TokenTy::ElementAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::ElementWhitespaceAfterAttributeValue,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::Gt,
                },
                None,
            ),
        ],
    ];
    let mut res: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> =
        vec![];
    for input in inputs {
        let mut output = vec![];
        let mut iter = TokenStream::new(input);
        let mut reconstructed = String::new();
        loop {
            let iter_clone = iter.clone();
            let iter_val = iter.next();
            output.push((iter_clone, iter_val));
            match iter_val {
                Some(tok) => reconstructed.push_str(tok.buf),
                None => break,
            }
        }
        assert_eq!(reconstructed, input);
        res.push(output);
    }
    assert_eq!(res, expected);
}

#[test]
#[expect(clippy::too_many_lines)]
fn tokenization_verbatim_test() {
    let inputs: Vec<&str> = vec![
        "<outer></outer>",
        " < outer > < / outer > ",
        " prefix < outer >inner< / outer > suffix",
        " prefix < outer > inner < / outer > suffix",
    ];
    let expected: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> = vec![
        vec![
            (
                TokenStream {
                    buf: "<outer></outer>",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "outer></outer>",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: "></outer>",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "</outer>",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: "/outer>",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: "outer>",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: ">",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::Gt,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " < outer > < / outer > ",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "< outer > < / outer > ",
                    prev_tok: TokenTy::LeadingWhitespace,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " outer > < / outer > ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer > < / outer > ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " > < / outer > ",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> < / outer > ",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " < / outer > ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::WhitespaceAfterGt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "< / outer > ",
                    prev_tok: TokenTy::WhitespaceAfterGt,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / outer > ",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ outer > ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " outer > ",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer > ",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " > ",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> ",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " ",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingWhitespace,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingWhitespace,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " prefix < outer >inner< / outer > suffix",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingGarbage,
                    buf: " prefix ",
                }),
            ),
            (
                TokenStream {
                    buf: "< outer >inner< / outer > suffix",
                    prev_tok: TokenTy::LeadingGarbage,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " outer >inner< / outer > suffix",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer >inner< / outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " >inner< / outer > suffix",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: ">inner< / outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: "inner< / outer > suffix",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: "inner",
                }),
            ),
            (
                TokenStream {
                    buf: "< / outer > suffix",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / outer > suffix",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " outer > suffix",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " > suffix",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
        vec![
            (
                TokenStream {
                    buf: " prefix < outer > inner < / outer > suffix",
                    prev_tok: TokenTy::StartOfTokenStream,
                },
                Some(Token {
                    tok: TokenTy::LeadingGarbage,
                    buf: " prefix ",
                }),
            ),
            (
                TokenStream {
                    buf: "< outer > inner < / outer > suffix",
                    prev_tok: TokenTy::LeadingGarbage,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " outer > inner < / outer > suffix",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer > inner < / outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementStartName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " > inner < / outer > suffix",
                    prev_tok: TokenTy::ElementStartName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterStartName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> inner < / outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterStartName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " inner < / outer > suffix",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::ElementContentVerbatim,
                    buf: " inner ",
                }),
            ),
            (
                TokenStream {
                    buf: "< / outer > suffix",
                    prev_tok: TokenTy::ElementContentVerbatim,
                },
                Some(Token {
                    tok: TokenTy::Lt,
                    buf: "<",
                }),
            ),
            (
                TokenStream {
                    buf: " / outer > suffix",
                    prev_tok: TokenTy::Lt,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLt,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "/ outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLt,
                },
                Some(Token {
                    tok: TokenTy::ElementLeadingSlash,
                    buf: "/",
                }),
            ),
            (
                TokenStream {
                    buf: " outer > suffix",
                    prev_tok: TokenTy::ElementLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "outer > suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterLeadingSlash,
                },
                Some(Token {
                    tok: TokenTy::ElementEndName,
                    buf: "outer",
                }),
            ),
            (
                TokenStream {
                    buf: " > suffix",
                    prev_tok: TokenTy::ElementEndName,
                },
                Some(Token {
                    tok: TokenTy::ElementWhitespaceAfterEndName,
                    buf: " ",
                }),
            ),
            (
                TokenStream {
                    buf: "> suffix",
                    prev_tok: TokenTy::ElementWhitespaceAfterEndName,
                },
                Some(Token {
                    tok: TokenTy::Gt,
                    buf: ">",
                }),
            ),
            (
                TokenStream {
                    buf: " suffix",
                    prev_tok: TokenTy::Gt,
                },
                Some(Token {
                    tok: TokenTy::TrailingGarbage,
                    buf: " suffix",
                }),
            ),
            (
                TokenStream {
                    buf: "",
                    prev_tok: TokenTy::TrailingGarbage,
                },
                None,
            ),
        ],
    ];
    let mut res: Vec<Vec<(TokenStream<'static>, Option<Token<'static>>)>> =
        vec![];
    for input in inputs {
        let mut output = vec![];
        let mut iter = TokenStream::new(input);
        let mut reconstructed = String::new();
        loop {
            let iter_clone = iter.clone();
            let iter_val = iter.next();
            output.push((iter_clone, iter_val));
            match iter_val {
                Some(tok) => reconstructed.push_str(tok.buf),
                None => break,
            }
        }
        assert_eq!(reconstructed, input);
        res.push(output);
    }
    assert_eq!(res, expected);
}
