use super::super::Int;
use super::super::colormatrix::ColorMatrix;
use super::super::colormatrix::ColorMatrixRows;
use super::super::colormatrixrow::ColorMatrixRow;
use super::super::colormatrixrow::PlaneValues;
use super::super::plane::Plane;
use super::super::planes::Planes;
use super::ColorMatrices;
use super::xmlparser;

type T<'a> = ColorMatrices;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T<'static>, String> {
    Result::Err(str.to_owned())
}

#[test]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<ColorMatrices",
        "<ColorMatrices ",
        "<NotColorMatrices ",
        "<ColorMatrices>
            <ColorMatrix planes=\"0\">
                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
            </ColorMatrix>
        </ColorMatrices>",
        "<ColorMatrices>
            <ColorMatrix planes=\"0\">
                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
            </ColorMatrix>
        </NotColorMatrices>",
        "<ColorMatrices>
            <ColorMatrix planes=\"0\">
                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
            </ColorMatrix>
            <ColorMatrix planes=\"0\">
                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>
            </ColorMatrix>
        </NotColorMatrices>",
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
            "<ColorMatrices",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrices\")`",
            ),
        ),
        (
            "<ColorMatrices ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotColorMatrices ",
            Err(
                "Error while parsing element, expected `\"ColorMatrices\"`, but instead found: `\"NotColorMatrices\"`",
            ),
        ),
        (
            "<ColorMatrices>\n            <ColorMatrix planes=\"0\">\n                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n            </ColorMatrix>\n        </ColorMatrices>",
            Ok(ColorMatrices {
                value: ColorMatrix {
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
                },
            }),
        ),
        (
            "<ColorMatrices>\n            <ColorMatrix planes=\"0\">\n                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n            </ColorMatrix>\n        </NotColorMatrices>",
            Err(
                "Error while parsing element, expected `\"ColorMatrices\"`, but instead found: `\"NotColorMatrices\"`",
            ),
        ),
        (
            "<ColorMatrices>\n            <ColorMatrix planes=\"0\">\n                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n            </ColorMatrix>\n            <ColorMatrix planes=\"0\">\n                <ColorMatrixRow plane=\"0\"> 21412 -4324 51 </ColorMatrixRow>\n            </ColorMatrix>\n        </NotColorMatrices>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementName(\"ColorMatrix\")`",
            ),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
