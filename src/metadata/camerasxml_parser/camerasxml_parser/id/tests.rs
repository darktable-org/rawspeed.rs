use super::{
    super::{BodyStr, Str},
    ID,
    make::Make,
    model::Model,
    xmlparser,
};

type T<'a> = ID<'a>;

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
        "<ID",
        "<IDmake",
        "<ID make",
        "<ID make=",
        "<ID make=Foo",
        "<ID make=\"Foo\"",
        "<ID make=\"Foo\"model",
        "<ID make=\"Foo\" model",
        "<ID make=\"Foo\" model=",
        "<ID make=\"Foo\" model=Bar",
        "<ID make=\"Foo\" model=\"Bar\"",
        "<ID make=\"Foo\" model=\"Bar\">",
        "<ID make=\"Foo\" model=\"Bar\">Baz",
        "<ID make=\"Foo\" model=\"Bar\">Baz<",
        "<ID make=\"Foo\" model=\"Bar\">Baz</ID",
        "<ID make=\"Foo\" model=\"Bar\">Bagz</ID>",
        "<ID make=\"Foo\" model=\"Bar\"> Baz Quux </ID>",
        "<ID make=\"Foo\" model=\"Bar\"></ID>",
        "<ID make=\"Foo\" model=\"Bar\"> </ID>",
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
            "<ID",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ID\")`",
            ),
        ),
        (
            "<IDmake",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"IDmake\")`",
            ),
        ),
        (
            "<ID make",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"make\")`",
            ),
        ),
        (
            "<ID make=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<ID make=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<ID make=\"Foo\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<ID make=\"Foo\"model",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"model\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"model\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<ID make=\"Foo\" model=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\">",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, encountered end of stream",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\">Baz",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Garbage(\"Baz\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\">Baz<",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\">Baz</ID",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"ID\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\">Bagz</ID>",
            Ok(ID {
                make: Make {
                    val: Str { val: "Foo" },
                },
                model: Model {
                    val: Str { val: "Bar" },
                },
                value: BodyStr { val: "Bagz" },
            }),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\"> Baz Quux </ID>",
            Ok(ID {
                make: Make {
                    val: Str { val: "Foo" },
                },
                model: Model {
                    val: Str { val: "Bar" },
                },
                value: BodyStr { val: " Baz Quux " },
            }),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\"></ID>",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Lt(\"<\")`",
            ),
        ),
        (
            "<ID make=\"Foo\" model=\"Bar\"> </ID>",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Lt(\"<\")`",
            ),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
