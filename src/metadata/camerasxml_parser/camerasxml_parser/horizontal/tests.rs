use super::{
    super::{Int, height, y},
    Horizontal, xmlparser,
};

type T = Horizontal;

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
        "<Horizontal",
        "<Horizontal ",
        "<NotHorizontal ",
        "<Horizontal noty",
        "<Horizontal y",
        "<Horizontal y=",
        "<Horizontal y=Foo",
        "<Horizontal y=\"Foo\"",
        "<Horizontal y=42",
        "<Horizontal y=\"42\"",
        "<Horizontal noty=\"42\"",
        "<Horizontal y=\"42\"height",
        "<Horizontal y=\"42\" height",
        "<Horizontal y=\"42\" height=",
        "<Horizontal y=\"42\" height=Bar",
        "<Horizontal y=\"42\" height=\"Bar\"",
        "<Horizontal y=\"42\" height=24",
        "<Horizontal y=\"42\" height=\"24\"",
        "<Horizontal y=\"42\" notheight=\"24\"",
        "<Horizontal y=\"42\" height=\"24\">",
        "<Horizontal y=\"42\" height=\"24\"/",
        "<Horizontal y=\"42\" height=\"24\"/>",
        "<Horizontal y=\"42\" height=\"24i32\"/>",
        "<Horizontal y=\"42\" height=\" 24\"/>",
        "<Horizontal y=\"42\" height=\"24 \"/>",
        "<Horizontal y=\"42\" height=\" 24 \"/>",
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
            "<Horizontal",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Horizontal\")`",
            ),
        ),
        (
            "<Horizontal ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotHorizontal ",
            Err(
                "Error while parsing element, expected `\"Horizontal\"`, but instead found: `\"NotHorizontal\"`",
            ),
        ),
        (
            "<Horizontal noty",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"noty\")`",
            ),
        ),
        (
            "<Horizontal y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<Horizontal y=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Horizontal y=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Horizontal y=\"Foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Foo\" }` as an integer",
            ),
        ),
        (
            "<Horizontal y=42",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"42\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Horizontal noty=\"42\"",
            Err(
                "Error while parsing attribute, expected `\"y\"`, but instead found: `\"noty\"`",
            ),
        ),
        (
            "<Horizontal y=\"42\"height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=24",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"24\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\"24\"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Horizontal y=\"42\" notheight=\"24\"",
            Err(
                "Error while parsing attribute, expected `\"height\"`, but instead found: `\"notheight\"`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\"24\">",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `Gt(\">\")`",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\"24\"/",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Horizontal y=\"42\" height=\"24\"/>",
            Ok(Horizontal {
                y: y::Y {
                    val: Int { val: 42 },
                },
                height: height::Height {
                    val: Int { val: 24 },
                },
            }),
        ),
        (
            "<Horizontal y=\"42\" height=\"24i32\"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"24i32\" }` as an integer",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\" 24\"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \" 24\" }` as an integer",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\"24 \"/>",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"24 \" }` as an integer",
            ),
        ),
        (
            "<Horizontal y=\"42\" height=\" 24 \"/>",
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
