use super::{
    super::{Str, name::Name, value::Value},
    Hints, IndividualHints,
    hint::Hint,
    xmlparser,
};

type T<'a> = Hints<'a>;

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
        "<Hints",
        "<Hints>",
        "<NotHints>",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
        <",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
        </",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
        </Hints",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
        </NotHints",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
        </Hints>",
        "<Hints>
            <Hint name=\"Foo\" value=\"Bar\"/>
            <Hint name=\"Quuz\" value=\"Quux\"/>
        </Hints>",
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
            "<Hints",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Hints\")`",
            ),
        ),
        ("<Hints>", Err("unexpected end of input, expected `Hint`")),
        (
            "<NotHints>",
            Err(
                "Error while parsing element, expected `\"Hints\"`, but instead found: `\"NotHints\"`",
            ),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n        </Hints",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Hints\")`",
            ),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n        </NotHints",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"NotHints\")`",
            ),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n        </Hints>",
            Ok(Hints {
                value: IndividualHints {
                    values: vec![Hint {
                        name: Name {
                            val: Str { val: "Foo" },
                        },
                        value: Value {
                            val: Str { val: "Bar" },
                        },
                    }],
                },
            }),
        ),
        (
            "<Hints>\n            <Hint name=\"Foo\" value=\"Bar\"/>\n            <Hint name=\"Quuz\" value=\"Quux\"/>\n        </Hints>",
            Ok(Hints {
                value: IndividualHints {
                    values: vec![
                        Hint {
                            name: Name {
                                val: Str { val: "Foo" },
                            },
                            value: Value {
                                val: Str { val: "Bar" },
                            },
                        },
                        Hint {
                            name: Name {
                                val: Str { val: "Quuz" },
                            },
                            value: Value {
                                val: Str { val: "Quux" },
                            },
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
