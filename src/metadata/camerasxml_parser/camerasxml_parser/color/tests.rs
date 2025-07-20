use super::super::Int;
use super::super::color::Color;
use super::ColorVariant;
use super::x::X;
use super::xmlparser;
use super::y::Y;

type T<'a> = Color;

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
        "<Color ",
        "<NotColor ",
        "<Colorx",
        "<Color x",
        "<Color x ",
        "<Color not_x ",
        "<Color x=",
        "<Color x=Foo",
        "<Color x=\"foo\"",
        "<Color x=\"11\"",
        "<Color x=\"11\"y",
        "<Color x=\"11\" y",
        "<Color x=\"11\" y ",
        "<Color x=\"11\" not_y ",
        "<Color x=\"11\" y=",
        "<Color x=\"11\" y=22",
        "<Color x=\"11\" y=\"bar\"",
        "<Color x=\"11\" y=\"22\"",
        "<Color x=\"11\" y=\"22\">",
        "<Color x=\"11\" y=\"22\">RED",
        "<Color x=\"11\" y=\"22\">RED<",
        "<Color x=\"11\" y=\"22\">RED</Color",
        "<Color x=\"11\" y=\"22\">RED</Color>",
        "<Color x=\"11\" y=\"22\">RED</NotColor>",
        "<Color x=\"11\" y=\"22\"> RED </Color>",
        "<Color x=\"11\" y=\"22\"> red </Color>",
        "<Color x=\"11\" y=\"22\"> RED </Color>",
        "<Color x=\"11\" y=\"22\">GREEN</Color>",
        "<Color x=\"11\" y=\"22\">BLUE</Color>",
        "<Color x=\"11\" y=\"22\">FUJI_GREEN</Color>",
        "<Color x=\"11\" y=\"22\">MAGENTA</Color>",
        "<Color x=\"11\" y=\"22\">YELLOW</Color>",
        "<Color x=\"11\" y=\"22\">CYAN</Color>",
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
            "<Color ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotColor ",
            Err(
                "Error while parsing element, expected `\"Color\"`, but instead found: `\"NotColor\"`",
            ),
        ),
        (
            "<Colorx",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Colorx\")`",
            ),
        ),
        (
            "<Color x",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"x\")`",
            ),
        ),
        (
            "<Color x ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Color not_x ",
            Err(
                "Error while parsing attribute, expected `\"x\"`, but instead found: `\"not_x\"`",
            ),
        ),
        (
            "<Color x=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Color x=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Color x=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<Color x=\"11\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Color x=\"11\"y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<Color x=\"11\" y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<Color x=\"11\" y ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Color x=\"11\" not_y ",
            Err(
                "Error while parsing attribute, expected `\"y\"`, but instead found: `\"not_y\"`",
            ),
        ),
        (
            "<Color x=\"11\" y=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Color x=\"11\" y=22",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"22\")`",
            ),
        ),
        (
            "<Color x=\"11\" y=\"bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"bar\" }` as an integer",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Color x=\"11\" y=\"22\">",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, encountered end of stream",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\">RED",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"RED\")`",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\">RED<",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\">RED</Color",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Color\")`",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\">RED</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Red,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">RED</NotColor>",
            Err(
                "Error while parsing element, expected `\"Color\"`, but instead found: `\"NotColor\"`",
            ),
        ),
        (
            "<Color x=\"11\" y=\"22\"> RED </Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Red,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\"> red </Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Red,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\"> RED </Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Red,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">GREEN</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Green,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">BLUE</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Blue,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">FUJI_GREEN</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::FujiGreen,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">MAGENTA</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Magenta,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">YELLOW</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Yellow,
            }),
        ),
        (
            "<Color x=\"11\" y=\"22\">CYAN</Color>",
            Ok(Color {
                x: X {
                    val: Int { val: 11 },
                },
                y: Y {
                    val: Int { val: 22 },
                },
                value: ColorVariant::Cyan,
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
