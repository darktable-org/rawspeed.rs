use super::super::Int;
use super::super::blackareas::BlackAreas;
use super::super::height::Height;
use super::super::horizontal::Horizontal;
use super::super::vertical::Vertical;
use super::super::width::Width;
use super::super::x::X;
use super::super::y::Y;
use super::IndividualBlackAreas;
use super::xmlparser;

type T<'a> = BlackAreas;

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
        "<BlackAreas",
        "<BlackAreas ",
        "<NotBlackAreas ",
        "<BlackAreas>",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        <",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        </",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        </BlackAreas",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        </BlackAreas ",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        </NotBlackAreas ",
        "<BlackAreas>
            <Vertical x=\"32\" width=\"24\"/>
        </BlackAreas>",
        "<BlackAreas>
        <Vertical x=\"32\" width=\"24\"/>
            <Horizontal y=\"42\" height=\"11\"/>
            <Vertical x=\"23\" width=\"42\"/>
            <Horizontal y=\"22\" height=\"21\"/>
            <Vertical x=\"33\" width=\"422\"/>
        </BlackAreas>",
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
            "<BlackAreas",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"BlackAreas\")`",
            ),
        ),
        (
            "<BlackAreas ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotBlackAreas ",
            Err(
                "Error while parsing element, expected `\"BlackAreas\"`, but instead found: `\"NotBlackAreas\"`",
            ),
        ),
        (
            "<BlackAreas>",
            Err("unexpected end of input, expected black areas"),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        </BlackAreas",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"BlackAreas\")`",
            ),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        </BlackAreas ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        </NotBlackAreas ",
            Err(
                "Error while parsing element, expected `\"BlackAreas\"`, but instead found: `\"NotBlackAreas\"`",
            ),
        ),
        (
            "<BlackAreas>\n            <Vertical x=\"32\" width=\"24\"/>\n        </BlackAreas>",
            Ok(BlackAreas {
                value: IndividualBlackAreas {
                    verticals: vec![Vertical {
                        x: X {
                            val: Int { val: 32 },
                        },
                        width: Width {
                            val: Int { val: 24 },
                        },
                    }],
                    horizontals: vec![],
                },
            }),
        ),
        (
            "<BlackAreas>\n        <Vertical x=\"32\" width=\"24\"/>\n            <Horizontal y=\"42\" height=\"11\"/>\n            <Vertical x=\"23\" width=\"42\"/>\n            <Horizontal y=\"22\" height=\"21\"/>\n            <Vertical x=\"33\" width=\"422\"/>\n        </BlackAreas>",
            Ok(BlackAreas {
                value: IndividualBlackAreas {
                    verticals: vec![
                        Vertical {
                            x: X {
                                val: Int { val: 32 },
                            },
                            width: Width {
                                val: Int { val: 24 },
                            },
                        },
                        Vertical {
                            x: X {
                                val: Int { val: 23 },
                            },
                            width: Width {
                                val: Int { val: 42 },
                            },
                        },
                        Vertical {
                            x: X {
                                val: Int { val: 33 },
                            },
                            width: Width {
                                val: Int { val: 422 },
                            },
                        },
                    ],
                    horizontals: vec![
                        Horizontal {
                            y: Y {
                                val: Int { val: 42 },
                            },
                            height: Height {
                                val: Int { val: 11 },
                            },
                        },
                        Horizontal {
                            y: Y {
                                val: Int { val: 22 },
                            },
                            height: Height {
                                val: Int { val: 21 },
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
