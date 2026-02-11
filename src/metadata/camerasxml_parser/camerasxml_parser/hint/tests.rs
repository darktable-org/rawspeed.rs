use super::{
    super::{Str, name::Name, value::Value},
    Hint, xmlparser,
};

type T<'a> = Hint<'a>;

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
        "<Hint",
        "<Hint ",
        "<NotHint ",
        "<Hint notname",
        "<Hint name",
        "<Hint name=",
        "<Hint name=Foo",
        "<Hint name=Foo ",
        "<Hint name= Foo",
        "<Hint name= Foo ",
        "<Hint name=\"Foo\"",
        "<Hint name=\" f o o \"",
        "<Hint notname=\" f o o \"",
        "<Hint name=\" f o o \"value",
        "<Hint name=\" f o o \" value",
        "<Hint name=\" f o o \" value=",
        "<Hint name=\" f o o \" value= b a r ",
        "<Hint name=\" f o o \" value=\" b a r \"",
        "<Hint name=\" f o o \" value= b a r ",
        "<Hint name=\" f o o \" value=\" b a r \"",
        "<Hint name=\" f o o \" notvalue=\" b a r \"",
        "<Hint name=\" f o o \" value=\" b a r \">",
        "<Hint name=\" f o o \" value=\" b a r \"/",
        "<Hint name=\" f o o \" value=\" b a r \"/>",
        "<Hint name=\" f o o \" value=\" b a r \"/>",
        "<Hint name=\" f o o \" value=\"  b a r \"/>",
        "<Hint name=\" f o o \" value=\" b a r \"/>",
        "<Hint name=\" f o o \" value=\" b a r \"/>",
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
            "<Hint",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Hint\")`",
            ),
        ),
        (
            "<Hint ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotHint ",
            Err(
                "Error while parsing element, expected `\"Hint\"`, but instead found: `\"NotHint\"`",
            ),
        ),
        (
            "<Hint notname",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"notname\")`",
            ),
        ),
        (
            "<Hint name",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"name\")`",
            ),
        ),
        (
            "<Hint name=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint name=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Hint name=Foo ",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo \")`",
            ),
        ),
        (
            "<Hint name= Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Hint name= Foo ",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo \")`",
            ),
        ),
        (
            "<Hint name=\"Foo\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint name=\" f o o \"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint notname=\" f o o \"",
            Err(
                "Error while parsing attribute, expected `\"name\"`, but instead found: `\"notname\"`",
            ),
        ),
        (
            "<Hint name=\" f o o \"value",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"value\")`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"value\")`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint name=\" f o o \" value= b a r ",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"b a r \")`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint name=\" f o o \" value= b a r ",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"b a r \")`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Hint name=\" f o o \" notvalue=\" b a r \"",
            Err(
                "Error while parsing attribute, expected `\"value\"`, but instead found: `\"notvalue\"`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \">",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `Gt(\">\")`",
            ),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"/",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"/>",
            Ok(Hint {
                name: Name {
                    val: Str { val: " f o o " },
                },
                value: Value {
                    val: Str { val: " b a r " },
                },
            }),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"/>",
            Ok(Hint {
                name: Name {
                    val: Str { val: " f o o " },
                },
                value: Value {
                    val: Str { val: " b a r " },
                },
            }),
        ),
        (
            "<Hint name=\" f o o \" value=\"  b a r \"/>",
            Ok(Hint {
                name: Name {
                    val: Str { val: " f o o " },
                },
                value: Value {
                    val: Str { val: "  b a r " },
                },
            }),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"/>",
            Ok(Hint {
                name: Name {
                    val: Str { val: " f o o " },
                },
                value: Value {
                    val: Str { val: " b a r " },
                },
            }),
        ),
        (
            "<Hint name=\" f o o \" value=\" b a r \"/>",
            Ok(Hint {
                name: Name {
                    val: Str { val: " f o o " },
                },
                value: Value {
                    val: Str { val: " b a r " },
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
