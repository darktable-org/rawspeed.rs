use super::super::Int;
use super::super::height::Height;
use super::super::width::Width;
use super::super::x::X;
use super::super::y::Y;
use super::Crop;
use super::xmlparser;

type T = Crop;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T, String> {
    Result::Err(str.to_owned())
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<Crop",
        "<Cropname",
        "<Crop x",
        "<Crop x=",
        "<Crop x=Foo",
        "<Crop x=\"Foo\"",
        "<Crop x=\"42\"",
        "<Crop not_x=\"42\"",
        "<Crop x=\"42\"y",
        "<Crop x=\"42\" y=",
        "<Crop x=\"42\" y=Bar",
        "<Crop x=\"42\" y=\"Bar\"",
        "<Crop x=\"42\" y=\"24\"",
        "<Crop x=\"42\" not_y=\"24\"",
        "<Crop x=\"42\" y=\"24\"width",
        "<Crop x=\"42\" y=\"24\" width",
        "<Crop x=\"42\" y=\"24\" width=",
        "<Crop x=\"42\" y=\"24\" width=Bar",
        "<Crop x=\"42\" y=\"24\" width=\"Bar\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\"",
        "<Crop x=\"42\" y=\"24\" not_width=\"22\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\"height",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=Bar",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"Bar\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" not_height=\"44\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T>)> = vec![
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
            "<Crop",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Crop\")`",
            ),
        ),
        (
            "<Cropname",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Cropname\")`",
            ),
        ),
        (
            "<Crop x",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"x\")`",
            ),
        ),
        (
            "<Crop x=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Crop x=\"Foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Foo\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop not_x=\"42\"",
            Err(
                "Error while parsing attribute, expected `\"x\"`, but instead found: `\"not_x\"`",
            ),
        ),
        (
            "<Crop x=\"42\"y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" not_y=\"24\"",
            Err(
                "Error while parsing attribute, expected `\"y\"`, but instead found: `\"not_y\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\"width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" not_width=\"22\"",
            Err(
                "Error while parsing attribute, expected `\"width\"`, but instead found: `\"not_width\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\"height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" not_height=\"44\"",
            Err(
                "Error while parsing attribute, expected `\"height\"`, but instead found: `\"not_height\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/>",
            Ok(Crop {
                x: X {
                    val: Int { val: 42 },
                },
                y: Y {
                    val: Int { val: 24 },
                },
                width: Width {
                    val: Int { val: 22 },
                },
                height: Height {
                    val: Int { val: 44 },
                },
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T>(input)));
    }
    assert_eq!(results, expected);
}
