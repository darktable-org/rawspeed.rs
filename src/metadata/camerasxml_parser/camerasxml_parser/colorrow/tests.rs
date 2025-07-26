use super::super::Int;
use super::super::y::Y;
use super::ColorRow;
use super::ColorRowValues;
use super::ColorVariant;
use super::xmlparser;

type T<'a> = ColorRow;

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
        "<ColorRow",
        "<ColorRow ",
        "<NotColorRow ",
        "<ColorRow y",
        "<ColorRow y ",
        "<ColorRow not_y ",
        "<ColorRow y=",
        "<ColorRow y=Foo",
        "<ColorRow y=\"foo\"",
        "<ColorRow y=\"11\"",
        "<ColorRow y=\"11\">",
        "<ColorRow y=\"11\">R",
        "<ColorRow y=\"11\">R<",
        "<ColorRow y=\"11\">R</ColorRow",
        "<ColorRow y=\"11\">R</ColorRow ",
        "<ColorRow y=\"11\">R</NotColorRow ",
        "<ColorRow y=\"11\">R</ColorRow>",
        "<ColorRow y=\"11\">G</ColorRow>",
        "<ColorRow y=\"11\">B</ColorRow>",
        "<ColorRow y=\"11\">RGB</ColorRow>",
        "<ColorRow y=\"11\"> R G B </ColorRow>",
        "<ColorRow y=\"11\">X</ColorRow>",
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
            "<ColorRow",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorRow\")`",
            ),
        ),
        (
            "<ColorRow ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotColorRow ",
            Err(
                "Error while parsing element, expected `\"ColorRow\"`, but instead found: `\"NotColorRow\"`",
            ),
        ),
        (
            "<ColorRow y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<ColorRow y ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<ColorRow not_y ",
            Err(
                "Error while parsing attribute, expected `\"y\"`, but instead found: `\"not_y\"`",
            ),
        ),
        (
            "<ColorRow y=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorRow y=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<ColorRow y=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<ColorRow y=\"11\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<ColorRow y=\"11\">",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorRow y=\"11\">R",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"R\")`",
            ),
        ),
        (
            "<ColorRow y=\"11\">R<",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorRow y=\"11\">R</ColorRow",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorRow\")`",
            ),
        ),
        (
            "<ColorRow y=\"11\">R</ColorRow ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<ColorRow y=\"11\">R</NotColorRow ",
            Err(
                "Error while parsing element, expected `\"ColorRow\"`, but instead found: `\"NotColorRow\"`",
            ),
        ),
        (
            "<ColorRow y=\"11\">R</ColorRow>",
            Ok(ColorRow {
                y: Y {
                    val: Int { val: 11 },
                },
                value: ColorRowValues {
                    values: vec![ColorVariant::R],
                },
            }),
        ),
        (
            "<ColorRow y=\"11\">G</ColorRow>",
            Ok(ColorRow {
                y: Y {
                    val: Int { val: 11 },
                },
                value: ColorRowValues {
                    values: vec![ColorVariant::G],
                },
            }),
        ),
        (
            "<ColorRow y=\"11\">B</ColorRow>",
            Ok(ColorRow {
                y: Y {
                    val: Int { val: 11 },
                },
                value: ColorRowValues {
                    values: vec![ColorVariant::B],
                },
            }),
        ),
        (
            "<ColorRow y=\"11\">RGB</ColorRow>",
            Ok(ColorRow {
                y: Y {
                    val: Int { val: 11 },
                },
                value: ColorRowValues {
                    values: vec![
                        ColorVariant::R,
                        ColorVariant::G,
                        ColorVariant::B,
                    ],
                },
            }),
        ),
        (
            "<ColorRow y=\"11\"> R G B </ColorRow>",
            Ok(ColorRow {
                y: Y {
                    val: Int { val: 11 },
                },
                value: ColorRowValues {
                    values: vec![
                        ColorVariant::R,
                        ColorVariant::G,
                        ColorVariant::B,
                    ],
                },
            }),
        ),
        (
            "<ColorRow y=\"11\">X</ColorRow>",
            Err("Unexpected color: X"),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
