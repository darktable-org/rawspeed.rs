use super::Str;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct IsoValues {
    pub values: Vec<i32>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for IsoValues {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let raw_values = input.parse::<Str<'a>>()?;
        let mut values = vec![];
        for val in raw_values.split_ascii_whitespace() {
            if let Ok(val) = val.parse() {
                values.push(val);
            } else {
                return Err(format!("Unable to parse `{val:?}` as an integer"));
            }
        }
        Ok(Self { values })
    }
}

impl_attr_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct IsoList {
        iso_list: IsoValues,
    }
);
