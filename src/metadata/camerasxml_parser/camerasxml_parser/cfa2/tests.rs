use super::super::Int;
use super::super::colorrow::ColorRow;
use super::super::colorrow::ColorRowValues;
use super::super::colorrow::ColorVariant;
use super::super::height::Height;
use super::super::width::Width;
use super::super::y::Y;
use super::CFA2;
use super::CFA2Colors;
use super::xmlparser;

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
            <ColorRow y=\"11\">G</ColorRow>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        <",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        </",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        </CFA2",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        </CFA2 ",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        </NotCFA2 ",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"11\">G</ColorRow>
        </CFA2>",
        "<CFA2 width=\"1\" height=\"1\">
            <ColorRow y=\"0\">G</ColorRow>
            <ColorRow y=\"1\">R</ColorRow>
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
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        </CFA2",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"CFA2\")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        </CFA2 ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        </NotCFA2 ",
            Err(
                "Error while parsing element, expected `\"CFA2\"`, but instead found: `\"NotCFA2\"`",
            ),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"11\">G</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFA2Colors {
                    values: vec![ColorRow {
                        y: Y {
                            val: Int { val: 11 },
                        },
                        value: ColorRowValues {
                            values: vec![ColorVariant::G],
                        },
                    }],
                },
            }),
        ),
        (
            "<CFA2 width=\"1\" height=\"1\">\n            <ColorRow y=\"0\">G</ColorRow>\n            <ColorRow y=\"1\">R</ColorRow>\n        </CFA2>",
            Ok(CFA2 {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFA2Colors {
                    values: vec![
                        ColorRow {
                            y: Y {
                                val: Int { val: 0 },
                            },
                            value: ColorRowValues {
                                values: vec![ColorVariant::G],
                            },
                        },
                        ColorRow {
                            y: Y {
                                val: Int { val: 1 },
                            },
                            value: ColorRowValues {
                                values: vec![ColorVariant::R],
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
