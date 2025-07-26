use super::*;

#[test]
fn parse_leading_garbage_test() {
    let input = " prefix ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: " prefix " }));
}

#[test]
fn parse_leading_lt_failure_test() {
    let input = " prefix ";
    let mut p = ParseStream::new(input);
    assert_eq!(
        p.parse::<Lt<'_>>(),
        Err(
            "While trying to match `\"Lt\"`, but the following was encountered instead: `Garbage(\" prefix \")`".to_owned()));
}

#[test]
fn parse_leading_lt_failure_repeat_test() {
    let input = " prefix ";
    let mut p = ParseStream::new(input);
    assert_eq!(
        p.parse::<Lt<'_>>(),
        Err(
            "While trying to match `\"Lt\"`, but the following was encountered instead: `Garbage(\" prefix \")`".to_owned()));
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: " prefix " }));
    assert_eq!(
        p.parse::<Garbage<'_>>(),
        Err(
            "While trying to match `\"Garbage\"`, encountered end of stream"
                .to_owned()
        )
    );
}

#[test]
fn parse_leading_lt_test() {
    let input = "<";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
}

#[test]
fn parse_leading_lt_after_whitespace_test() {
    let input = " \t \r \n <";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
}

#[test]
fn parse_leading_lt_after_garbage_test() {
    let input = " garbage <";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: " garbage " }));
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
}

#[test]
fn parse_start_name_test() {
    let input = " < start_name ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
}

#[test]
fn parse_end_name_test() {
    let input = " < / end_name ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
}

#[test]
fn parse_end_name_with_lt_test() {
    let input = " < / end_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_end_name_with_garbage_test() {
    let input = " < / end_name garbage ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: "garbage " }));
}

#[test]
fn parse_end_name_with_garbage_and_lt_test() {
    let input = " < / end_name garbage > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: "garbage > " }));
}

#[test]
fn parse_end_name_with_garbage_after_lt_test() {
    let input = " < / end_name > garbage ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: " garbage " }));
}

#[test]
fn parse_start_name_with_gt_test() {
    let input = " < start_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_start_name_with_garbage_after_gt_test() {
    let input = " < start_name > garbage ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: " garbage " }));
}

#[test]
fn parse_start_name_with_garbage_before_gt_test() {
    let input = " < start_name garbage > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeName<'_>>(),
        Ok(ElementAttributeName { buf: "garbage" })
    );
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: "> " }));
}

#[test]
fn parse_start_name_with_garbage_and_eq_before_gt_test() {
    let input = " < start_name garbage =  > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeName<'_>>(),
        Ok(ElementAttributeName { buf: "garbage" })
    );
    assert_eq!(
        p.parse::<ElementAttributeEq<'_>>(),
        Ok(ElementAttributeEq { buf: "=" })
    );
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: "> " }));
}

#[test]
fn parse_start_name_with_garbage_before_and_after_eq_test() {
    let input = " < start_name garbage = other_garbage > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeName<'_>>(),
        Ok(ElementAttributeName { buf: "garbage" })
    );
    assert_eq!(
        p.parse::<ElementAttributeEq<'_>>(),
        Ok(ElementAttributeEq { buf: "=" })
    );
    assert_eq!(
        p.parse::<Garbage<'_>>(),
        Ok(Garbage {
            buf: "other_garbage > "
        })
    );
}

#[test]
fn parse_start_name_with_attr_using_double_quotes_test() {
    let input = " < start_name attr_name = \"attr'val\" > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeName<'_>>(),
        Ok(ElementAttributeName { buf: "attr_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeEq<'_>>(),
        Ok(ElementAttributeEq { buf: "=" })
    );
    assert_eq!(
        p.parse::<ElementAttributeValue<'_>>(),
        Ok(ElementAttributeValue { buf: "attr'val" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_start_name_with_attr_using_single_quotes_test() {
    let input = " < start_name attr_name = 'attr\"val' > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeName<'_>>(),
        Ok(ElementAttributeName { buf: "attr_name" })
    );
    assert_eq!(
        p.parse::<ElementAttributeEq<'_>>(),
        Ok(ElementAttributeEq { buf: "=" })
    );
    assert_eq!(
        p.parse::<ElementAttributeValue<'_>>(),
        Ok(ElementAttributeValue { buf: "attr\"val" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_start_name_with_trailing_slash_test() {
    let input = " < start_name / > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_start_name_with_leading_and_trailing_slashes_test() {
    let input = " < / start_name / > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<Garbage<'_>>(), Ok(Garbage { buf: "/ > " }));
}

#[test]
fn parse_start_name_with_leading_and_trailing_slashes_and_attr_test() {
    let input = " < / start_name attr=\"val\" / > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(
        p.parse::<Garbage<'_>>(),
        Ok(Garbage {
            buf: "attr=\"val\" / > "
        })
    );
}

#[test]
fn parse_two_opening_elts_test() {
    let input = " < start_name > < other_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "other_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_two_opening_elts_and_verbatim_inbetween_test() {
    let input = " < start_name > con tent < other_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(
        p.parse::<ElementContentVerbatim<'_>>(),
        Ok(ElementContentVerbatim { buf: " con tent " })
    );
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "other_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_verbatim_after_leading_slash_test() {
    let input = " < / end_name > con tent < other_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "end_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(
        p.parse::<ElementContentVerbatim<'_>>(),
        Ok(ElementContentVerbatim { buf: " con tent " })
    );
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "other_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}

#[test]
fn parse_verbatim_after_trailing_slash_test() {
    let input = " < start_name / > con tent < other_name > ";
    let mut p = ParseStream::new(input);
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "start_name" })
    );
    assert_eq!(p.parse::<ElementSlash<'_>>(), Ok(ElementSlash { buf: "/" }));
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
    assert_eq!(
        p.parse::<ElementContentVerbatim<'_>>(),
        Ok(ElementContentVerbatim { buf: " con tent " })
    );
    assert_eq!(p.parse::<Lt<'_>>(), Ok(Lt { buf: "<" }));
    assert_eq!(
        p.parse::<ElementName<'_>>(),
        Ok(ElementName { buf: "other_name" })
    );
    assert_eq!(p.parse::<Gt<'_>>(), Ok(Gt { buf: ">" }));
}
