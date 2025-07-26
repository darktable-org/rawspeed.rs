use super::BodyStr;
use super::x;
use super::xmlparser;
use super::y;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorVariant {
    Red,
    Green,
    Blue,
    FujiGreen,
    Magenta,
    Yellow,
    Cyan,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for ColorVariant {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let color_str =
            input.parse::<BodyStr<'a>>()?.trim().to_ascii_uppercase();
        let c = match color_str.as_str() {
            "RED" => ColorVariant::Red,
            "GREEN" => ColorVariant::Green,
            "BLUE" => ColorVariant::Blue,
            "FUJI_GREEN" => ColorVariant::FujiGreen,
            "MAGENTA" => ColorVariant::Magenta,
            "YELLOW" => ColorVariant::Yellow,
            "CYAN" => ColorVariant::Cyan,
            _ => {
                return Err(format!(
                    "Unable to parse `{color_str:?}` as a color"
                ));
            }
        };
        Ok(c)
    }
}

impl_elt_with_body_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Color {
        x: x::X,
        y: y::Y,
        value: ColorVariant,
    }
);

#[cfg(test)]
mod tests;
