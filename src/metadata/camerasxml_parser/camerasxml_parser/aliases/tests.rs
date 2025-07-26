use super::super::BodyStr;
use super::super::Str;
use super::super::id_attr::Id;
use super::Aliases;
use super::IndividualAliases;
use super::alias::Alias;
use super::xmlparser;

type T<'a> = Aliases<'a>;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T<'static>, String> {
    Result::Err(str.to_owned())
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<Aliases",
        "<Aliases ",
        "<NotAliases>",
        "<Aliases>",
        "<Aliases>
            <Alias>Foo</Alias>",
        "<Aliases>
            <Alias>Foo</Alias>
        <",
        "<Aliases>
            <Alias>Foo</Alias>
        </",
        "<Aliases>
            <Alias>Foo</Alias>
        </Aliases",
        "<Aliases>
            <Alias>Foo</Alias>
        </Aliases ",
        "<Aliases>
            <Alias>Foo</Alias>
        </NotAliases ",
        "<Aliases>
            <Alias>Foo</Alias>
        </Aliases>",
        "<Aliases>
            <Alias>Foo</Alias>
            <Alias>bar</Alias>
            <Alias id=\"Baz\">Qux</Alias>
        </Aliases>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![
        (
            "",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<Aliases",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Aliases\")`",
            ),
        ),
        (
            "<Aliases ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotAliases>",
            Err(
                "Error while parsing element, expected `\"Aliases\"`, but instead found: `\"NotAliases\"`",
            ),
        ),
        (
            "<Aliases>",
            Err("unexpected end of input, expected `Alias`"),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        </Aliases",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Aliases\")`",
            ),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        </Aliases ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        </NotAliases ",
            Err(
                "Error while parsing element, expected `\"Aliases\"`, but instead found: `\"NotAliases\"`",
            ),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n        </Aliases>",
            Ok(Aliases {
                value: IndividualAliases {
                    values: vec![Alias {
                        id: None,
                        value: BodyStr { val: "Foo" },
                    }],
                },
            }),
        ),
        (
            "<Aliases>\n            <Alias>Foo</Alias>\n            <Alias>bar</Alias>\n            <Alias id=\"Baz\">Qux</Alias>\n        </Aliases>",
            Ok(Aliases {
                value: IndividualAliases {
                    values: vec![
                        Alias {
                            id: None,
                            value: BodyStr { val: "Foo" },
                        },
                        Alias {
                            id: None,
                            value: BodyStr { val: "bar" },
                        },
                        Alias {
                            id: Some(Id {
                                val: Str { val: "Baz" },
                            }),
                            value: BodyStr { val: "Qux" },
                        },
                    ],
                },
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
