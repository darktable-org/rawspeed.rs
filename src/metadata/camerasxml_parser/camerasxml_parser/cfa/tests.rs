use super::super::Int;
use super::super::color::Color;
use super::super::color::ColorVariant;
use super::super::height::Height;
use super::super::width::Width;
use super::super::x::X;
use super::super::y::Y;
use super::CFA;
use super::CFAColors;
use super::xmlparser;

type T<'a> = CFA;

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
        "<CFA",
        "<CFA ",
        "<NotCFA ",
        "<CFAwidth",
        "<CFA width",
        "<CFA width ",
        "<CFA not_width ",
        "<CFA width=",
        "<CFA width=foo",
        "<CFA width=\"foo\"",
        "<CFA width=\"foo\"",
        "<CFA width=\"1\"",
        "<CFA width=\"1\"height",
        "<CFA width=\"1\" height",
        "<CFA width=\"1\" height ",
        "<CFA width=\"1\" not_height ",
        "<CFA width=\"1\" height=",
        "<CFA width=\"1\" height=foo",
        "<CFA width=\"1\" height=\"foo\"",
        "<CFA width=\"1\" height=\"foo\"",
        "<CFA width=\"1\" height=\"1\"",
        "<CFA width=\"1\" height=\"1\">",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        <",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA ",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </NotCFA ",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"1\">GREEN</Color>
        </CFA>",
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
            "<CFA",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA\")`",
            ),
        ),
        (
            "<CFA ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotCFA ",
            Err(
                "Error while parsing element, expected `\"CFA\"`, but instead found: `\"NotCFA\"`",
            ),
        ),
        (
            "<CFAwidth",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFAwidth\")`",
            ),
        ),
        (
            "<CFA width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<CFA width ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA not_width ",
            Err(
                "Error while parsing attribute, expected `\"width\"`, but instead found: `\"not_width\"`",
            ),
        ),
        (
            "<CFA width=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<CFA width=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA width=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA width=\"1\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\"height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<CFA width=\"1\" height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<CFA width=\"1\" height ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA width=\"1\" not_height ",
            Err(
                "Error while parsing attribute, expected `\"height\"`, but instead found: `\"not_height\"`",
            ),
        ),
        (
            "<CFA width=\"1\" height=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\" height=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">",
            Err("unexpected end of input, expected `Color`"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA\")`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </NotCFA ",
            Err(
                "Error while parsing element, expected `\"CFA\"`, but instead found: `\"NotCFA\"`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA>",
            Ok(CFA {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFAColors {
                    values: vec![Color {
                        x: X {
                            val: Int { val: 0 },
                        },
                        y: Y {
                            val: Int { val: 0 },
                        },
                        value: ColorVariant::Red,
                    }],
                },
            }),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"1\">GREEN</Color>\n        </CFA>",
            Ok(CFA {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFAColors {
                    values: vec![
                        Color {
                            x: X {
                                val: Int { val: 0 },
                            },
                            y: Y {
                                val: Int { val: 0 },
                            },
                            value: ColorVariant::Red,
                        },
                        Color {
                            x: X {
                                val: Int { val: 1 },
                            },
                            y: Y {
                                val: Int { val: 1 },
                            },
                            value: ColorVariant::Green,
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
