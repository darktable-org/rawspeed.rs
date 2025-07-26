use super::super::Str;
use super::super::alias::Alias;
use super::BodyStr;
use super::id_attr::Id;
use super::xmlparser;

type T<'a> = Alias<'a>;

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
        "<Alias",
        "<Alias ",
        "<NotAlias ",
        "<Aliasid",
        "<Alias id",
        "<Alias id ",
        "<Alias not_id ",
        "<Alias id=",
        "<Alias id=foo",
        "<Alias id=\"foo\"",
        "<Alias id=\"foo\">",
        "<Alias id=\"foo\"><",
        "<Alias id=\"foo\"></",
        "<Alias id=\"foo\"></Alias",
        "<Alias>Bar</Alias>",
        "<Alias id=\"foo\">Bar</Alias>",
        "<Alias id=\"foo\"> Baq Quux </Alias>",
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
            "<Alias",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Alias\")`",
            ),
        ),
        (
            "<Alias ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotAlias ",
            Err(
                "Error while parsing element, expected `\"Alias\"`, but instead found: `\"NotAlias\"`",
            ),
        ),
        (
            "<Aliasid",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Aliasid\")`",
            ),
        ),
        (
            "<Alias id",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\"id\")`",
            ),
        ),
        (
            "<Alias id ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `ElementAttributeName(\"id\")`",
            ),
        ),
        (
            "<Alias not_id ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `ElementAttributeName(\"not_id\")`",
            ),
        ),
        (
            "<Alias id=",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `ElementAttributeName(\"id\")`",
            ),
        ),
        (
            "<Alias id=foo",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `ElementAttributeName(\"id\")`",
            ),
        ),
        (
            "<Alias id=\"foo\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Alias id=\"foo\">",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, encountered end of stream",
            ),
        ),
        (
            "<Alias id=\"foo\"><",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Lt(\"<\")`",
            ),
        ),
        (
            "<Alias id=\"foo\"></",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Lt(\"<\")`",
            ),
        ),
        (
            "<Alias id=\"foo\"></Alias",
            Err(
                "While trying to match `\"ElementContentVerbatim\"`, but the following was encountered instead: `Lt(\"<\")`",
            ),
        ),
        (
            "<Alias>Bar</Alias>",
            Ok(Alias {
                id: None,
                value: BodyStr { val: "Bar" },
            }),
        ),
        (
            "<Alias id=\"foo\">Bar</Alias>",
            Ok(Alias {
                id: Some(Id {
                    val: Str { val: "foo" },
                }),
                value: BodyStr { val: "Bar" },
            }),
        ),
        (
            "<Alias id=\"foo\"> Baq Quux </Alias>",
            Ok(Alias {
                id: Some(Id {
                    val: Str { val: "foo" },
                }),
                value: BodyStr { val: " Baq Quux " },
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
