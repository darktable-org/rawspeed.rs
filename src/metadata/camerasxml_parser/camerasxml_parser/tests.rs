use super::{
    Str,
    camera::{Camera, MaybeCFA, Sensors},
    cameras::{Cameras, IndividualCameras},
    make::Make,
    model::Model,
    supported::Supported,
    xmlparser,
};

type T<'a> = Cameras<'a>;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T<'static>, String> {
    Result::Err(str.to_owned())
}

#[test]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<Cameras>
            <Camera make=\"Make\" model=\"Model\">
            </Camera>
        </Cameras>",
        "garbage
        <Cameras>
            <Camera make=\"Make\" model=\"Model\">
            </Camera>
        </Cameras>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![
        (
            "",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
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
                        supported: Supported::Supported,
                        id: None,
                        cfa: MaybeCFA::none(),
                        crop: None,
                        sensors: Sensors { values: vec![] },
                        blackareas: None,
                        aliases: None,
                        hints: None,
                        colormatrices: None,
                    }],
                },
            }),
        ),
        (
            "garbage\n        <Cameras>\n            <Camera make=\"Make\" model=\"Model\">\n            </Camera>\n        </Cameras>",
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
                        supported: Supported::Supported,
                        id: None,
                        cfa: MaybeCFA::none(),
                        crop: None,
                        sensors: Sensors { values: vec![] },
                        blackareas: None,
                        aliases: None,
                        hints: None,
                        colormatrices: None,
                    }],
                },
            }),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, super::parse_str(input)));
    }
    assert_eq!(results, expected);
}
