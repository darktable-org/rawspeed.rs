use super::ColorMatrix;
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
        "<ColorMatrix planes=\"0\">",
        "<ColorMatrix planes=\"1\">",
        "<ColorMatrix planes=\"2\">",
        "<ColorMatrix planes=\"3\">",
        "<ColorMatrix planes=\"4\">",
        "<ColorMatrix planes=\"5\">",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        ",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        <",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </ColorMatrix",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"4\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
            <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"4\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
            <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </NotColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> -1 -1 -1 </ColorMatrixRow>
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
        </ColorMatrix>",
        "<ColorMatrix planes=\"3\">
            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>
            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> -1 -1 -1 </ColorMatrixRow>
            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>
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
            Err("Unsupported number of color planes (0)"),
        ),
        (
            "<ColorMatrix planes=\"0\">",
            Err("Unsupported number of color planes (0)"),
        ),
        (
            "<ColorMatrix planes=\"0\">",
            Err("Unsupported number of color planes (0)"),
        ),
        (
            "<ColorMatrix planes=\"1\">",
            Err("Unsupported number of color planes (1)"),
        ),
        (
            "<ColorMatrix planes=\"2\">",
            Err("Unsupported number of color planes (2)"),
        ),
        (
            "<ColorMatrix planes=\"3\">",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<ColorMatrix planes=\"4\">",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<ColorMatrix planes=\"5\">",
            Err("Unsupported number of color planes (5)"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        ",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ColorMatrix\")`",
            ),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix>",
            Ok(ColorMatrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected color matrix row count, got 2 expected 3"),
        ),
        (
            "<ColorMatrix planes=\"4\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>\n        </ColorMatrix>",
            Ok(ColorMatrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])),
        ),
        (
            "<ColorMatrix planes=\"4\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected color matrix row count, got 3 expected 4"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected color matrix row count, got 4 expected 3"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </NotColorMatrix>",
            Err(
                "Error while parsing element, expected `\"ColorMatrix\"`, but instead found: `\"NotColorMatrix\"`",
            ),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> -1 -1 -1 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected plane, got 0 expected 1"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected plane, got 1 expected 0"),
        ),
        (
            "<ColorMatrix planes=\"3\">\n            <ColorMatrixRow plane=\"0\"> 0 1 2 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"1\"> 3 4 5 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> -1 -1 -1 </ColorMatrixRow>\n            <ColorMatrixRow plane=\"2\"> 6 7 8 </ColorMatrixRow>\n        </ColorMatrix>",
            Err("unexpected plane, got 2 expected 3"),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
