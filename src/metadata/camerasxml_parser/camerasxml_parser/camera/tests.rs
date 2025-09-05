use super::super::BodyStr;
use super::super::Int;
use super::super::Str;
use super::super::alias::Alias;
use super::super::aliases::Aliases;
use super::super::aliases::IndividualAliases;
use super::super::black::Black;
use super::super::blackareas::BlackAreas;
use super::super::blackareas::IndividualBlackAreas;
use super::super::cfa::CFA;
use super::super::cfa::CFAColors;
use super::super::cfa2::CFA2;
use super::super::cfa2::CFA2Colors;
use super::super::color;
use super::super::color::Color;
use super::super::colormatrices::ColorMatrices;
use super::super::colormatrix::ColorMatrix;
use super::super::colorrow;
use super::super::colorrow::ColorRow;
use super::super::colorrow::ColorRowValues;
use super::super::crop::Crop;
use super::super::decoder_version::DecoderVersion;
use super::super::height::Height;
use super::super::hint::Hint;
use super::super::hints::Hints;
use super::super::hints::IndividualHints;
use super::super::id::ID;
use super::super::iso_list::IsoList;
use super::super::iso_list::IsoValues;
use super::super::iso_max::IsoMax;
use super::super::iso_min::IsoMin;
use super::super::mode::Mode;
use super::super::name::Name;
use super::super::sensor::Sensor;
use super::super::supported::Supported;
use super::super::value::Value;
use super::super::vertical::Vertical;
use super::super::white::White;
use super::super::width::Width;
use super::super::x::X;
use super::super::y::Y;
use super::Camera;
use super::MaybeCFA;
use super::Sensors;
use super::make::Make;
use super::model::Model;
use super::sensor;
use super::xmlparser;
use crate::camerasxml_parser::crop;
use rawspeed_std::coord_common::ColIndex;
use rawspeed_std::coord_common::Coord2D;
use rawspeed_std::coord_common::RowCount;
use rawspeed_std::coord_common::RowIndex;
use rawspeed_std::coord_common::RowLength;

type T<'a> = Camera<'a>;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T<'static>, String> {
    Result::Err(str.to_owned())
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_outer_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<Camera",
        "<Camera ",
        "<NotCamera ",
        "<Camera make",
        "<Camera make ",
        "<Camera not_make ",
        "<Camera make=\"Make\"",
        "<Camera make=\"Make\" model",
        "<Camera make=\"Make\" model ",
        "<Camera make=\"Make\" not_model ",
        "<Camera make=\"Make\" model=\"Model\"",
        "<Camera make=\"Make\" model=\"Model\">",
        "<Camera make=\"Make\" model=\"Model\">
        <",
        "<Camera make=\"Make\" model=\"Model\">
        </",
        "<Camera make=\"Make\" model=\"Model\">
        </Camera",
        "<Camera make=\"Make\" model=\"Model\">
        </Camera ",
        "<Camera make=\"Make\" model=\"Model\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" decoder_version=\"0\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"yes\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"no-samples\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"no\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"no-no-samples\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"unknown\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"unknown-no-samples\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" supported=\"garbage\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\" supported=\"yes\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" decoder_version=\"0\" supported=\"yes\">
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\" decoder_version=\"0\" supported=\"yes\">
        </Camera>",
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
            "<Camera",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Camera\")`",
            ),
        ),
        (
            "<Camera ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotCamera ",
            Err(
                "Error while parsing element, expected `\"Camera\"`, but instead found: `\"NotCamera\"`",
            ),
        ),
        (
            "<Camera make",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"make\")`",
            ),
        ),
        (
            "<Camera make ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Camera not_make ",
            Err(
                "Error while parsing attribute, expected `\"make\"`, but instead found: `\"not_make\"`",
            ),
        ),
        (
            "<Camera make=\"Make\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Camera make=\"Make\" model",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"model\")`",
            ),
        ),
        (
            "<Camera make=\"Make\" model ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Camera make=\"Make\" not_model ",
            Err(
                "Error while parsing attribute, expected `\"model\"`, but instead found: `\"not_model\"`",
            ),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\"",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n        <",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n        </",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n        </Camera",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Camera\")`",
            ),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n        </Camera ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: Some(Mode {
                    val: Str { val: "Mode" },
                }),
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" decoder_version=\"0\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: Some(DecoderVersion {
                    val: Int { val: 0 },
                }),
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"yes\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"no-samples\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::SupportedNoSamples,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"no\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Unsupported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"no-no-samples\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::UnsupportedNoSamples,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"unknown\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Unknown,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"unknown-no-samples\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::UnknownNoSamples,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" supported=\"garbage\">\n        </Camera>",
            Err("Invalid support enum: garbage"),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\" supported=\"yes\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: Some(Mode {
                    val: Str { val: "Mode" },
                }),
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" decoder_version=\"0\" supported=\"yes\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: Some(DecoderVersion {
                    val: Int { val: 0 },
                }),
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\" mode=\"Mode\" decoder_version=\"0\" supported=\"yes\">\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: Some(Mode {
                    val: Str { val: "Mode" },
                }),
                decoder_version: Some(DecoderVersion {
                    val: Int { val: 0 },
                }),
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_id_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <ID make=\"another make\" model=\"another model\">pretty name</ID>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <ID make=\"another make\" model=\"another model\">pretty name</ID>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: Some(ID {
                make: Make {
                    val: Str {
                        val: "another make",
                    },
                },
                model: Model {
                    val: Str {
                        val: "another model",
                    },
                },
                value: BodyStr { val: "pretty name" },
            }),
            cfa: MaybeCFA::None,
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: None,
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_cfa_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <CFA width=\"1\" height=\"1\">
                <Color x=\"0\" y=\"0\">RED</Color>
            </CFA>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <CFA width=\"1\" height=\"1\">\n                <Color x=\"0\" y=\"0\">RED</Color>\n            </CFA>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::CFA(CFA {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFAColors {
                    values: vec![Color {
                        x: X {
                            val: Int { val: 0 },
                        },
                        y: Y {
                            val: Int { val: 0 },
                        },
                        value: color::ColorVariant::Red,
                    }],
                },
            }),
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: None,
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_cfa2_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <CFA2 width=\"1\" height=\"1\">
                <ColorRow y=\"0\">G</ColorRow>
            </CFA2>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <CFA2 width=\"1\" height=\"1\">\n                <ColorRow y=\"0\">G</ColorRow>\n            </CFA2>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::CFA2(CFA2 {
                width: Width {
                    val: Int { val: 1 },
                },
                height: Height {
                    val: Int { val: 1 },
                },
                values: CFA2Colors {
                    values: vec![ColorRow {
                        y: Y {
                            val: Int { val: 0 },
                        },
                        value: ColorRowValues {
                            values: vec![colorrow::ColorVariant::G],
                        },
                    }],
                },
            }),
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: None,
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_crop_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <Crop x=\"0\" y=\"1\" width=\"2\" height=\"3\"/>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <Crop x=\"0\" y=\"1\" width=\"2\" height=\"3\"/>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::None,
            crop: Some(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(2)),
                    crop::Height::Absolute(RowCount::new(3)),
                ),
            )),
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: None,
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_sensors_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\"/>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\" iso_min=\"100\"/>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\" iso_max=\"200\"/>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\" iso_min=\"100\" iso_max=\"200\"/>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\" iso_list=\"100\"/>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <Sensor black=\"42\" white=\"24\" iso_min=\"50\"/>
            <Sensor black=\"42\" white=\"24\" iso_max=\"100\"/>
            <Sensor black=\"42\" white=\"24\" iso_list=\"60\"/>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![Sensor {
                        black: Black {
                            val: Int { val: 42 },
                        },
                        white: White {
                            val: Int { val: 24 },
                        },
                        bounds: sensor::Bounds::Unbounded,
                    }],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\" iso_min=\"100\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![Sensor {
                        black: Black {
                            val: Int { val: 42 },
                        },
                        white: White {
                            val: Int { val: 24 },
                        },
                        bounds: sensor::Bounds::LowerBounded(IsoMin {
                            val: Int { val: 100 },
                        }),
                    }],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\" iso_max=\"200\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![Sensor {
                        black: Black {
                            val: Int { val: 42 },
                        },
                        white: White {
                            val: Int { val: 24 },
                        },
                        bounds: sensor::Bounds::UpperBounded(IsoMax {
                            val: Int { val: 200 },
                        }),
                    }],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\" iso_min=\"100\" iso_max=\"200\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![Sensor {
                        black: Black {
                            val: Int { val: 42 },
                        },
                        white: White {
                            val: Int { val: 24 },
                        },
                        bounds: sensor::Bounds::Range((
                            IsoMin {
                                val: Int { val: 100 },
                            },
                            IsoMax {
                                val: Int { val: 200 },
                            },
                        )),
                    }],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\" iso_list=\"100\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![Sensor {
                        black: Black {
                            val: Int { val: 42 },
                        },
                        white: White {
                            val: Int { val: 24 },
                        },
                        bounds: sensor::Bounds::Enumerated(IsoList {
                            val: IsoValues { values: vec![100] },
                        }),
                    }],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <Sensor black=\"42\" white=\"24\" iso_min=\"50\"/>\n            <Sensor black=\"42\" white=\"24\" iso_max=\"100\"/>\n            <Sensor black=\"42\" white=\"24\" iso_list=\"60\"/>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors {
                    values: vec![
                        Sensor {
                            black: Black {
                                val: Int { val: 42 },
                            },
                            white: White {
                                val: Int { val: 24 },
                            },
                            bounds: sensor::Bounds::LowerBounded(IsoMin {
                                val: Int { val: 50 },
                            }),
                        },
                        Sensor {
                            black: Black {
                                val: Int { val: 42 },
                            },
                            white: White {
                                val: Int { val: 24 },
                            },
                            bounds: sensor::Bounds::UpperBounded(IsoMax {
                                val: Int { val: 100 },
                            }),
                        },
                        Sensor {
                            black: Black {
                                val: Int { val: 42 },
                            },
                            white: White {
                                val: Int { val: 24 },
                            },
                            bounds: sensor::Bounds::Enumerated(IsoList {
                                val: IsoValues { values: vec![60] },
                            }),
                        },
                    ],
                },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: None,
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_blackareas_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <BlackAreas>
                <Vertical x=\"32\" width=\"24\"/>
            </BlackAreas>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <BlackAreas>\n                <Vertical x=\"32\" width=\"24\"/>\n            </BlackAreas>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::None,
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: Some(BlackAreas {
                value: IndividualBlackAreas {
                    verticals: vec![Vertical {
                        x: X {
                            val: Int { val: 32 },
                        },
                        width: Width {
                            val: Int { val: 24 },
                        },
                    }],
                    horizontals: vec![],
                },
            }),
            aliases: None,
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_aliases_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <Aliases>
                <Alias>Foo</Alias>
            </Aliases>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <Aliases>\n                <Alias>Foo</Alias>\n            </Aliases>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::None,
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: Some(Aliases {
                value: IndividualAliases {
                    values: vec![Alias {
                        id: None,
                        value: BodyStr { val: "Foo" },
                    }],
                },
            }),
            hints: None,
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_hints_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <Hints>
                <Hint name=\"Foo\" value=\"Bar\"/>
            </Hints>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![(
        "<Camera make=\"Make\" model=\"Model\">\n            <Hints>\n                <Hint name=\"Foo\" value=\"Bar\"/>\n            </Hints>\n        </Camera>",
        Ok(Camera {
            make: Make {
                val: Str { val: "Make" },
            },
            model: Model {
                val: Str { val: "Model" },
            },
            mode: None,
            decoder_version: None,
            supported: Supported::Supported,
            id: None,
            cfa: MaybeCFA::None,
            crop: None,
            sensors: Sensors { values: vec![] },
            blackaras: None,
            aliases: None,
            hints: Some(Hints {
                value: IndividualHints {
                    values: vec![Hint {
                        name: Name {
                            val: Str { val: "Foo" },
                        },
                        value: Value {
                            val: Str { val: "Bar" },
                        },
                    }],
                },
            }),
            colormatrices: None,
        }),
    )];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}

#[test]
fn parse_colormatrices_test() {
    let inputs: Vec<&'static str> = vec![
        "<Camera make=\"Make\" model=\"Model\">
            <ColorMatrices>
                <ColorMatrix planes=\"3\">
                    <ColorMatrixRow plane=\"0\"> -0 -1 2 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"2\"> -6 7 -8 </ColorMatrixRow>
                </ColorMatrix>
            </ColorMatrices>
        </Camera>",
        "<Camera make=\"Make\" model=\"Model\">
            <ColorMatrices>
                <ColorMatrix planes=\"4\">
                    <ColorMatrixRow plane=\"0\"> -0 -1 2 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"2\"> -6 7 -8 </ColorMatrixRow>
                    <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>
                </ColorMatrix>
            </ColorMatrices>
        </Camera>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <ColorMatrices>\n                <ColorMatrix planes=\"3\">\n                    <ColorMatrixRow plane=\"0\"> -0 -1 2 </ColorMatrixRow>\n                    <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>\n                    <ColorMatrixRow plane=\"2\"> -6 7 -8 </ColorMatrixRow>\n                </ColorMatrix>\n            </ColorMatrices>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: Some(ColorMatrices {
                    value: ColorMatrix::new(vec![
                        0, -1, 2, 3, -4, 5, -6, 7, -8,
                    ]),
                }),
            }),
        ),
        (
            "<Camera make=\"Make\" model=\"Model\">\n            <ColorMatrices>\n                <ColorMatrix planes=\"4\">\n                    <ColorMatrixRow plane=\"0\"> -0 -1 2 </ColorMatrixRow>\n                    <ColorMatrixRow plane=\"1\"> 3 -4 5 </ColorMatrixRow>\n                    <ColorMatrixRow plane=\"2\"> -6 7 -8 </ColorMatrixRow>\n                    <ColorMatrixRow plane=\"3\"> 9 10 11 </ColorMatrixRow>\n                </ColorMatrix>\n            </ColorMatrices>\n        </Camera>",
            Ok(Camera {
                make: Make {
                    val: Str { val: "Make" },
                },
                model: Model {
                    val: Str { val: "Model" },
                },
                mode: None,
                decoder_version: None,
                supported: Supported::Supported,
                id: None,
                cfa: MaybeCFA::None,
                crop: None,
                sensors: Sensors { values: vec![] },
                blackaras: None,
                aliases: None,
                hints: None,
                colormatrices: Some(ColorMatrices {
                    value: ColorMatrix::new(vec![
                        0, -1, 2, 3, -4, 5, -6, 7, -8, 9, 10, 11,
                    ]),
                }),
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
