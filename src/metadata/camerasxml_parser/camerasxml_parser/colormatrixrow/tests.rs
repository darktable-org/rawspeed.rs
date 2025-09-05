use super::ColorMatrixRow;
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
        "<ColorMatrixRow plane=\"0\">666",
        "<ColorMatrixRow plane=\"0\">666<",
        "<ColorMatrixRow plane=\"0\">666</ColorMatrixRow",
        "<ColorMatrixRow plane=\"0\">0</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 -1</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"-1\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"1\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"2\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"3\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"4\">0 -1 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 -1 2 3</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 -1 2</NotColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 abcd 2</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 32767 -32768</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 32768 0</ColorMatrixRow>",
        "<ColorMatrixRow plane=\"0\">0 -32769 0</ColorMatrixRow>",
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
            "<ColorMatrixRow plane=\"0\">666",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"666\")`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">666<",
            Err("Color matrix row must have 3 components, got 1"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">666</ColorMatrixRow",
            Err("Color matrix row must have 3 components, got 1"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0</ColorMatrixRow>",
            Err("Color matrix row must have 3 components, got 1"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 -1</ColorMatrixRow>",
            Err("Color matrix row must have 3 components, got 2"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 -1 2</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane::new(0),
                values: PlaneValues { values: [0, -1, 2] },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"-1\">0 -1 2</ColorMatrixRow>",
            Err("Invalid plane index: -1"),
        ),
        (
            "<ColorMatrixRow plane=\"1\">0 -1 2</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane::new(1),
                values: PlaneValues { values: [0, -1, 2] },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"2\">0 -1 2</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane::new(2),
                values: PlaneValues { values: [0, -1, 2] },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"3\">0 -1 2</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane::new(3),
                values: PlaneValues { values: [0, -1, 2] },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"4\">0 -1 2</ColorMatrixRow>",
            Err("Invalid plane index: 4"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 -1 2 3</ColorMatrixRow>",
            Err("Color matrix row must have 3 components, got 4"),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 -1 2</NotColorMatrixRow>",
            Err(
                "Error while parsing element, expected `\"ColorMatrixRow\"`, but instead found: `\"NotColorMatrixRow\"`",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 abcd 2</ColorMatrixRow>",
            Err(
                "Unable to parse plane components as integers: invalid digit found in string",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 32767 -32768</ColorMatrixRow>",
            Ok(ColorMatrixRow {
                plane: Plane::new(0),
                values: PlaneValues {
                    values: [0, 0x7FFF, -0x8000],
                },
            }),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 32768 0</ColorMatrixRow>",
            Err(
                "Unable to parse plane components as integers: number too large to fit in target type",
            ),
        ),
        (
            "<ColorMatrixRow plane=\"0\">0 -32769 0</ColorMatrixRow>",
            Err(
                "Unable to parse plane components as integers: number too small to fit in target type",
            ),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
