use rawspeed_metadata_colorfilterarray::colorfilterarray::{
    ColorFilterArray, ColorVariant,
};
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::RowLength;

use crate::camerasxml_parser::cfa::CFA;

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
        "<CFA width",
        "<CFA width ",
        "<CFA not_width ",
        "<CFA width=",
        "<CFA width=foo",
        "<CFA width=\"foo\"",
        "<CFA width=\"1\"",
        "<CFA width=\"1\"height",
        "<CFA width=\"1\" height",
        "<CFA width=\"1\" height ",
        "<CFA width=\"1\" not_height ",
        "<CFA width=\"1\" height=",
        "<CFA width=\"1\" height=foo",
        "<CFA width=\"1\" height=\"foo\"",
        "<CFA width=\"1\" height=\"1\"",
        "<CFA width=\"1\" height=\"1\">",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        <",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </CFA",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </CFA ",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </NotCFA ",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"1\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"1\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"0\">GREEN</Color>
        </CFA>",
        //
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"1\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"1\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
        </CFA>",
        "<CFA width=\"2\" height=\"2\">
            <Color x=\"0\" y=\"0\">RED</Color>
            <Color x=\"1\" y=\"0\">GREEN</Color>
            <Color x=\"0\" y=\"1\">GREEN</Color>
            <Color x=\"1\" y=\"1\">BLUE</Color>
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
            "<CFA width=\"1\" height=\"1\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">",
            Err("unexpected end of input, expected `Color`"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </CFA",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA\")`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </CFA ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </NotCFA ",
            Err(
                "Error while parsing element, expected `\"CFA\"`, but instead found: `\"NotCFA\"`",
            ),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </CFA>",
            Ok(CFA {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Green],
                    RowLength::new(core::num::NonZero::new(1).unwrap()),
                ),
            }),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("unexpected row index, expected 0 got 1"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n        </CFA>",
            Err("unexpected column index, expected 0 got 0"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"1\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("unexpected row index, expected 0 got 1"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"0\">GREEN</Color>\n        </CFA>",
            Err("unexpected row index, expected 1 got 0"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA>",
            Ok(CFA {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red],
                    RowLength::new(core::num::NonZero::new(1).unwrap()),
                ),
            }),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row length, got 2 expected 1"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"1\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Ok(CFA {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red, ColorVariant::Green],
                    RowLength::new(core::num::NonZero::new(1).unwrap()),
                ),
            }),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"1\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("unexpected CFA matrix row length, got 2 expected 1"),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA>",
            Err("unexpected CFA matrix row length, got 1 expected 2"),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n        </CFA>",
            Ok(CFA {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red, ColorVariant::Green],
                    RowLength::new(core::num::NonZero::new(2).unwrap()),
                ),
            }),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"2\" height=\"1\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row length, got 1 expected 2"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n        </CFA>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n        </CFA>",
            Err("Inconsistent row length"),
        ),
        (
            "<CFA width=\"2\" height=\"2\">\n            <Color x=\"0\" y=\"0\">RED</Color>\n            <Color x=\"1\" y=\"0\">GREEN</Color>\n            <Color x=\"0\" y=\"1\">GREEN</Color>\n            <Color x=\"1\" y=\"1\">BLUE</Color>\n        </CFA>",
            Ok(CFA {
                data: ColorFilterArray::new(
                    vec![
                        ColorVariant::Red,
                        ColorVariant::Green,
                        ColorVariant::Green,
                        ColorVariant::Blue,
                    ],
                    RowLength::new(core::num::NonZero::new(2).unwrap()),
                ),
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
