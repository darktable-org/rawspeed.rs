use super::super::colormatrix::ColorMatrix;
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
            <ColorMatrix planes=\"3\">
                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
            </ColorMatrix>
        </ColorMatrices>",
        "<ColorMatrices>
            <ColorMatrix planes=\"3\">
                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
            </ColorMatrix>
        </NotColorMatrices>",
        "<ColorMatrices>
            <ColorMatrix planes=\"3\">
                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
            </ColorMatrix>
            <ColorMatrix planes=\"3\">
                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
                <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>
                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
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
            "<ColorMatrices>\n            <ColorMatrix planes=\"3\">\n                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            </ColorMatrix>\n        </ColorMatrices>",
            Ok(ColorMatrices {
                value: ColorMatrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            }),
        ),
        (
            "<ColorMatrices>\n            <ColorMatrix planes=\"3\">\n                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            </ColorMatrix>\n        </NotColorMatrices>",
            Err(
                "Error while parsing element, expected `\"ColorMatrices\"`, but instead found: `\"NotColorMatrices\"`",
            ),
        ),
        (
            "<ColorMatrices>\n            <ColorMatrix planes=\"3\">\n                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            </ColorMatrix>\n            <ColorMatrix planes=\"3\">\n                <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>\n                <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            </ColorMatrix>\n        </NotColorMatrices>",
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
