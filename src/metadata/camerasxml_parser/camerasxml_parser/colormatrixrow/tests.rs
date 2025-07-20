use super::ColorMatrixRow;
use super::Int;
use super::PlaneValues;
use super::plane::Plane;
use super::xmlparser;

type T<'a> = ColorMatrixRow;

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
        "<ColorMatrixRow",
        "<ColorMatrixRow ",
        "<NotColorMatrixRow ",
        "<ColorMatrixRowplane",
        "<ColorMatrixRow plane",
        "<ColorMatrixRow plane=",
        "<ColorMatrixRow not_plane=",
        "<ColorMatrixRow plane=foo",
        "<ColorMatrixRow plane=\"foo\"",
        "<ColorMatrixRow plane=\"foo\"",
        "<ColorMatrixRow plane=\"0\">",
        "<ColorMatrixRow plane=\"0\">Baz",
        "<ColorMatrixRow plane=\"0\">3412312",
        "<ColorMatrixRow plane=\"0\">3412312<",
        "<ColorMatrixRow plane=\"0\">3412312</ColorMatrixRow",
        "<ColorMatrixRow plane=\"0\">124123421</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">124123421</NotColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">124123421 -545432</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>",
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
            "<ColorMatrixRow",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrixRow\")`",
            ),
        ),
        (
            "<ColorMatrixRow ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotColorMatrixRow ",
            Err(
                "Error while parsing element, expected `\"ColorMatrixRow\"`, but instead found: `\"NotColorMatrixRow\"`",
            ),
        ),
        (
            "<ColorMatrixRowplane",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrixRowplane\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"plane\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrixRow not_plane=",
            Err(
                "Error while parsing attribute, expected `\"plane\"`, but instead found: `\"not_plane\"`",
            ),
        ),
        (
            "<ColorMatrixRow plane=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">Baz",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"Baz\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">3412312",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"3412312\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">3412312<",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">3412312</ColorMatrixRow",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrixRow\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">124123421</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane {
                    val: Int { val: 0 },
                },
                values: PlaneValues {
                    values: vec![Int { val: 124_123_421 }],
                },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"0\">124123421</NotColorMatrixRow>",
            Err(
                "Error while parsing element, expected `\"ColorMatrixRow\"`, but instead found: `\"NotColorMatrixRow\"`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">124123421 -545432</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane {
                    val: Int { val: 0 },
                },
                values: PlaneValues {
                    values: vec![
                        Int { val: 124_123_421 },
                        Int { val: -545_432 },
                    ],
                },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane {
                    val: Int { val: 0 },
                },
                values: PlaneValues {
                    values: vec![
                        Int { val: 21412 },
                        Int { val: -4324 },
                        Int { val: 51 },
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
