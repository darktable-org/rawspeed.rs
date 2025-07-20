use super::BodyStr;
use super::xmlparser;
use super::y;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorVariant {
    R,
    G,
    B,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ColorRowValues {
    pub values: Vec<ColorVariant>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for ColorRowValues {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut res = vec![];
        let color_str = input.parse::<BodyStr<'a>>()?;

        for s in color_str.split_ascii_whitespace() {
            for ch in s.chars() {
                let c = match ch.to_ascii_uppercase() {
                    'R' => ColorVariant::R,
                    'G' => ColorVariant::G,
                    'B' => ColorVariant::B,
                    _ => {
                        return Err(format!("Unexpected color: {ch}"));
                    }
                };
                res.push(c);
            }
        }
        Ok(Self { values: res })
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct ColorRow {
        y: y::Y,
        value: ColorRowValues,
    }
);

#[cfg(test)]
mod tests;
