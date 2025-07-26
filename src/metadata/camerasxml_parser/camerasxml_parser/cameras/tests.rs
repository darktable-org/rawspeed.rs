use super::super::Str;
use super::super::camera::Camera;
use super::super::camera::MaybeCFA;
use super::super::camera::Sensors;
use super::super::cameras::Cameras;
use super::super::cameras::IndividualCameras;
use super::super::make::Make;
use super::super::model::Model;
use super::xmlparser;

type T<'a> = Cameras<'a>;

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
        "<Cameras",
        "<Cameras ",
        "<NotCameras ",
        "<Cameras>",
        "<Cameras>
        <",
        "<Cameras>
        </Cameras",
        "<Cameras>
        </Cameras ",
        "<Cameras>
        </Cameras>",
        "<Cameras>
            <Camera make=\"Make\" model=\"Model\">
            </Camera>
        </Cameras>",
        "<Cameras>
            <Camera make=\"Make\" model=\"Model\">
            </Camera>
            <Camera make=\"Other Make\" model=\"Other Model\">
            </Camera>
        </Cameras>",
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
            "<Cameras",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Cameras\")`",
            ),
        ),
        (
            "<Cameras ",
            Err(
                "While trying to match `\"Gt\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotCameras ",
            Err(
                "Error while parsing element, expected `\"Cameras\"`, but instead found: `\"NotCameras\"`",
            ),
        ),
        (
            "<Cameras>",
            Err("unexpected end of input, expected `Camera`"),
        ),
        (
            "<Cameras>\n        <",
            Err("unexpected end of input, expected `Camera`"),
        ),
        (
            "<Cameras>\n        </Cameras",
            Err("unexpected end of input, expected `Camera`"),
        ),
        (
            "<Cameras>\n        </Cameras ",
            Err("unexpected end of input, expected `Camera`"),
        ),
        (
            "<Cameras>\n        </Cameras>",
            Err("unexpected end of input, expected `Camera`"),
        ),
        (
            "<Cameras>\n            <Camera make=\"Make\" model=\"Model\">\n            </Camera>\n        </Cameras>",
            Ok(Cameras {
                cameras: IndividualCameras {
                    values: vec![Camera {
                        make: Make {
                            val: Str { val: "Make" },
                        },
                        model: Model {
                            val: Str { val: "Model" },
                        },
                        mode: None,
                        decoder_version: None,
                        supported: None,
                        id: None,
                        cfa: MaybeCFA::None,
                        crop: None,
                        sensors: Sensors { values: vec![] },
                        blackaras: None,
                        aliases: None,
                        hints: None,
                        colormatrices: None,
                    }],
                },
            }),
        ),
        (
            "<Cameras>\n            <Camera make=\"Make\" model=\"Model\">\n            </Camera>\n            <Camera make=\"Other Make\" model=\"Other Model\">\n            </Camera>\n        </Cameras>",
            Ok(Cameras {
                cameras: IndividualCameras {
                    values: vec![
                        Camera {
                            make: Make {
                                val: Str { val: "Make" },
                            },
                            model: Model {
                                val: Str { val: "Model" },
                            },
                            mode: None,
                            decoder_version: None,
                            supported: None,
                            id: None,
                            cfa: MaybeCFA::None,
                            crop: None,
                            sensors: Sensors { values: vec![] },
                            blackaras: None,
                            aliases: None,
                            hints: None,
                            colormatrices: None,
                        },
                        Camera {
                            make: Make {
                                val: Str { val: "Other Make" },
                            },
                            model: Model {
                                val: Str { val: "Other Model" },
                            },
                            mode: None,
                            decoder_version: None,
                            supported: None,
                            id: None,
                            cfa: MaybeCFA::None,
                            crop: None,
                            sensors: Sensors { values: vec![] },
                            blackaras: None,
                            aliases: None,
                            hints: None,
                            colormatrices: None,
                        },
                    ],
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
