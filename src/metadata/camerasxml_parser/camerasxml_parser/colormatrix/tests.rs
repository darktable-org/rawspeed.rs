use super::super::Int;
use super::super::colormatrixrow::ColorMatrixRow;
use super::super::colormatrixrow::PlaneValues;
use super::super::plane::Plane;
use super::super::planes::Planes;
use super::ColorMatrix;
use super::ColorMatrixRows;
use super::xmlparser;

type T<'a> = ColorMatrix;

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
        "<ColorMatrix",
        "<ColorMatrix ",
        "<NotColorMatrix ",
        "<ColorMatrixplanes",
        "<ColorMatrix planes",
        "<ColorMatrix planes ",
        "<ColorMatrix not_planes ",
        "<ColorMatrix planes=",
        "<ColorMatrix planes=foo",
        "<ColorMatrix planes=\"foo\"",
        "<ColorMatrix planes=\"foo\"",
        "<ColorMatrix planes=\"0\"",
        "<ColorMatrix planes=\"0\">",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
        <",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
        </",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
        </ColorMatrix",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
        </NotColorMatrix>",
        "<ColorMatrix planes=\"0\">
            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> -2523 422 -4532 </ColorMatrixRow>
        </ColorMatrix>",
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
            "<ColorMatrix",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrix\")`",
            ),
        ),
        (
            "<ColorMatrix ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotColorMatrix ",
            Err(
                "Error while parsing element, expected `\"ColorMatrix\"`, but instead found: `\"NotColorMatrix\"`",
            ),
        ),
        (
            "<ColorMatrixplanes",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrixplanes\")`",
            ),
        ),
        (
            "<ColorMatrix planes",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"planes\")`",
            ),
        ),
        (
            "<ColorMatrix planes ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<ColorMatrix not_planes ",
            Err(
                "Error while parsing attribute, expected `\"planes\"`, but instead found: `\"not_planes\"`",
            ),
        ),
        (
            "<ColorMatrix planes=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrix planes=foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"foo\")`",
            ),
        ),
        (
            "<ColorMatrix planes=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<ColorMatrix planes=\"foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"foo\" }` as an integer",
            ),
        ),
        (
            "<ColorMatrix planes=\"0\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<ColorMatrix planes=\"0\">",
            Err("unexpected end of input, expected `ColorMatrixRow`"),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n        </ColorMatrix",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrix\")`",
            ),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n        </ColorMatrix>",
            Ok(ColorMatrix {
                planes: Planes {
                    val: Int { val: 0 },
                },
                values: ColorMatrixRows {
                    values: vec![ColorMatrixRow {
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
                    }],
                },
            }),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n        </NotColorMatrix>",
            Err(
                "Error while parsing element, expected `\"ColorMatrix\"`, but instead found: `\"NotColorMatrix\"`",
            ),
        ),
        (
            "<ColorMatrix planes=\"0\">\n            <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> -2523 422 -4532 </ColorMatrixRow>\n        </ColorMatrix>",
            Ok(ColorMatrix {
                planes: Planes {
                    val: Int { val: 0 },
                },
                values: ColorMatrixRows {
                    values: vec![
                        ColorMatrixRow {
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
                        },
                        ColorMatrixRow {
                            plane: Plane {
                                val: Int { val: 2 },
                            },
                            values: PlaneValues {
                                values: vec![
                                    Int { val: -2523 },
                                    Int { val: 422 },
                                    Int { val: -4532 },
                                ],
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
