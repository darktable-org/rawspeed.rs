use super::super::Int;
use super::super::width;
use super::super::x;
use super::Vertical;
use super::xmlparser;

type T = Vertical;

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
        "<Vertical",
        "<Vertical ",
        "<NotVertical ",
        "<Vertical notx",
        "<Vertical x",
        "<Vertical x=",
        "<Vertical x=Foo",
        "<Vertical x=\"Foo\"",
        "<Vertical x=42",
        "<Vertical x=\"42\"",
        "<Vertical notx=\"42\"",
        "<Vertical x=\"42\"width",
        "<Vertical x=\"42\" width",
        "<Vertical x=\"42\" width=",
        "<Vertical x=\"42\" width=Bar",
        "<Vertical x=\"42\" width=\"Bar\"",
        "<Vertical x=\"42\" width=24",
        "<Vertical x=\"42\" width=\"24\"",
        "<Vertical x=\"42\" notheight=\"24\"",
        "<Vertical x=\"42\" width=\"24\">",
        "<Vertical x=\"42\" width=\"24\"/",
        "<Vertical x=\"42\" width=\"24\"/>",
        "<Vertical x=\"42\" width=\"24i32\"/>",
        "<Vertical x=\"42\" width=\" 24\"/>",
        "<Vertical x=\"42\" width=\"24 \"/>",
        "<Vertical x=\"42\" width=\" 24 \"/>",
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
            "<Vertical",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Vertical\")`",
            ),
        ),
        (
            "<Vertical ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotVertical ",
            Err(
                "Error while parsing element, expected `\"Vertical\"`, but instead found: `\"NotVertical\"`",
            ),
        ),
        (
            "<Vertical notx",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"notx\")`",
            ),
        ),
        (
            "<Vertical x",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"x\")`",
            ),
        ),
        (
            "<Vertical x=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Vertical x=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Vertical x=\"Foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Foo\" }` as an integer",
            ),
        ),
        (
            "<Vertical x=42",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"42\")`",
            ),
        ),
        (
            "<Vertical x=\"42\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Vertical notx=\"42\"",
            Err(
                "Error while parsing attribute, expected `\"x\"`, but instead found: `\"notx\"`",
            ),
        ),
        (
            "<Vertical x=\"42\"width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Vertical x=\"42\" width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Vertical x=\"42\" width=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Vertical x=\"42\" width=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Vertical x=\"42\" width=24",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"24\")`",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\"24\"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Vertical x=\"42\" notheight=\"24\"",
            Err(
                "Error while parsing attribute, expected `\"width\"`, but instead found: `\"notheight\"`",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\"24\">",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `Gt(\">\")`",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\"24\"/",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Vertical x=\"42\" width=\"24\"/>",
            Ok(Vertical {
                x: x::X {
                    val: Int { val: 42 },
                },
                width: width::Width {
                    val: Int { val: 24 },
                },
            }),
        ),
        (
            "<Vertical x=\"42\" width=\"24i32\"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"24i32\" }` as an integer",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\" 24\"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \" 24\" }` as an integer",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\"24 \"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"24 \" }` as an integer",
            ),
        ),
        (
            "<Vertical x=\"42\" width=\" 24 \"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \" 24 \" }` as an integer",
            ),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T>(input)));
    }
    assert_eq!(results, expected);
}
