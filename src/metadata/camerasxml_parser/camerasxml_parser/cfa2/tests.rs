use rawspeed_metadata_colorfilterarray::colorfilterarray::{
    ColorFilterArray, ColorVariant,
};
use rawspeed_metadata_xmlparser::xmlparser;
use rawspeed_std::coord_common::RowLength;

use crate::camerasxml_parser::cfa2::CFA2;

type T<'a> = CFA2;

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
        "<CFA2",
        "<CFA2 ",
        "<NotCFA2 ",
        "<CFA2 width",
        "<CFA2 width ",
        "<CFA2 not_width ",
        "<CFA2 width=",
        "<CFA2 width=foo",
        "<CFA2 width=\"foo\"",
        "<CFA2 width=\"1\"",
        "<CFA2 width=\"1\"height",
        "<CFA2 width=\"1\" height",
        "<CFA2 width=\"1\" height ",
        "<CFA2 width=\"1\" not_height ",
        "<CFA2 width=\"1\" height=",
        "<CFA2 width=\"1\" height=foo",
        "<CFA2 width=\"1\" height=\"foo\"",
        "<CFA2 width=\"1\" height=\"1\"",
        "<CFA2 width=\"1\" height=\"1\">",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        <",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        </",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        </CFA2",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        </CFA2 ",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        </NotCFA2 ",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
            <ColorRow y=\"0\">G</ColorRow>
        </CFA2>",
        //
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"1\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">R</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"2\" height=\"2\">
            <ColorRow y=\"0\">RG</ColorRow>
            <ColorRow y=\"1\">GB</ColorRow>
        </CFA2>",
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
            "<CFA2",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA2\")`",
            ),
        ),
        (
            "<CFA2 ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotCFA2 ",
            Err(
                "Error while parsing element, expected `\"CFA2\"`, but instead found: `\"NotCFA2\"`",
            ),
        ),
        (
            "<CFA2 width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<CFA2 width ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA2 not_width ",
            Err(
                "Error while parsing attribute, expected `\"width\"`, but instead found: `\"not_width\"`",
            ),
        ),
        (
            "<CFA2 width=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<CFA2 width=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA2 width=\"1\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\"height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" not_height ",
            Err(
                "Error while parsing attribute, expected `\"height\"`, but instead found: `\"not_height\"`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">",
            Err("unexpected end of input, expected `ColorRow`"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        </CFA2",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA2\")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        </CFA2 ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        </NotCFA2 ",
            Err(
                "Error while parsing element, expected `\"CFA2\"`, but instead found: `\"NotCFA2\"`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Green],
                    RowLength::new(1),
                ),
            }),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("unexpected row index, expected 0 got 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n            <ColorRow y=\"0\">G</ColorRow>\n        </CFA2>",
            Err("unexpected row index, expected 1 got 0"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red],
                    RowLength::new(1),
                ),
            }),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 1 got 2"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row length, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 2 got 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red, ColorVariant::Green],
                    RowLength::new(1),
                ),
            }),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 1 got 2"),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 2 got 1"),
        ),
        (
            "<CFA2 width=\"1\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row length, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row length, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 1 got 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                data: ColorFilterArray::new(
                    vec![ColorVariant::Red, ColorVariant::Green],
                    RowLength::new(2),
                ),
            }),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 2 got 1"),
        ),
        (
            "<CFA2 width=\"2\" height=\"1\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 2 expected 1"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row length, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">R</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 1 got 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n        </CFA2>",
            Err("unexpected CFA matrix row count, got 1 expected 2"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">G</ColorRow>\n        </CFA2>",
            Err("inconsistent row length, expected 2 got 1"),
        ),
        (
            "<CFA2 width=\"2\" height=\"2\">\n            <ColorRow y=\"0\">RG</ColorRow>\n            <ColorRow y=\"1\">GB</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                data: ColorFilterArray::new(
                    vec![
                        ColorVariant::Red,
                        ColorVariant::Green,
                        ColorVariant::Green,
                        ColorVariant::Blue,
                    ],
                    RowLength::new(2),
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
