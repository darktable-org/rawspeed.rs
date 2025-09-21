use rawspeed_metadata_colorfilterarray::colorfilterarray::ColorFilterArray;

use super::aliases;
use super::blackareas;
use super::cfa;
use super::cfa2;
use super::colormatrices;
use super::crop;
use super::decoder_version;
use super::hints;
use super::id;
use super::make;
use super::mode;
use super::model;
use super::sensor;
use super::supported;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub struct MaybeCFA {
    val: Option<ColorFilterArray>,
}

impl MaybeCFA {
    pub const fn some(val: ColorFilterArray) -> Self {
        Self { val: Some(val) }
    }
    pub const fn none() -> Self {
        Self { val: None }
    }
}

impl core::ops::Deref for MaybeCFA {
    type Target = Option<ColorFilterArray>;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for MaybeCFA {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        if let Ok(cfa) = input.parse::<cfa::CFA>() {
            return Ok(Self::some(cfa.take_cfa()));
        }
        if let Ok(cfa) = input.parse::<cfa2::CFA2>() {
            return Ok(Self::some(cfa.take_cfa()));
        }
        Ok(Self::none())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sensors {
    pub values: Vec<sensor::Sensor>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Sensors {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let mut values = Vec::new();
        while let Ok(row) = input.parse() {
            values.push(row);
        }
        Ok(Self { values })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Camera<'a> {
    pub make: make::Make<'a>,
    pub model: model::Model<'a>,
    pub mode: Option<mode::Mode<'a>>,
    pub decoder_version: Option<decoder_version::DecoderVersion>,
    pub supported: supported::Supported,
    pub id: Option<id::ID<'a>>,
    pub cfa: MaybeCFA,
    pub crop: Option<crop::Crop>,
    pub sensors: Sensors,
    pub blackaras: Option<blackareas::BlackAreas>,
    pub aliases: Option<aliases::Aliases<'a>>,
    pub hints: Option<hints::Hints<'a>>,
    pub colormatrices: Option<colormatrices::ColorMatrices>,
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Camera<'a> {
    #[cfg_attr(not(test), expect(clippy::missing_inline_in_public_items))]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        const EXPECTED_NAME: &str = "Camera";
        input.parse::<xmlparser::Lt<'a>>()?;
        match *input.parse::<xmlparser::ElementName<'a>>()? {
            EXPECTED_NAME => {}
            name => {
                return Err(format!(
                    "Error while parsing element, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                ));
            }
        }
        let make = input.parse()?;
        let model = input.parse()?;
        let mode = input.parse()?;
        let decoder_version = input.parse()?;
        let supported = input.parse()?;
        input.parse::<xmlparser::Gt<'a>>()?;
        let id = input.parse()?;
        let cfa = input.parse()?;
        let crop = input.parse()?;
        let sensors = input.parse()?;
        let blackaras = input.parse()?;
        let aliases = input.parse()?;
        let hints = input.parse()?;
        let colormatrices = input.parse()?;
        input.parse::<xmlparser::Lt<'a>>()?;
        input.parse::<xmlparser::ElementSlash<'a>>()?;
        match *input.parse::<xmlparser::ElementName<'a>>()? {
            EXPECTED_NAME => {}
            name => {
                return Err(format!(
                    "Error while parsing element, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                ));
            }
        }
        input.parse::<xmlparser::Gt<'a>>()?;
        Ok(Self {
            make,
            model,
            mode,
            decoder_version,
            supported,
            id,
            cfa,
            crop,
            sensors,
            blackaras,
            aliases,
            hints,
            colormatrices,
        })
    }
}

#[cfg(test)]
mod tests;
