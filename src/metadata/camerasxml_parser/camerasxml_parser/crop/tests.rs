use rawspeed_std::coord_common::{
    ColIndex, ColOffset, Coord2D, RowCount, RowIndex, RowLength, RowOffset,
};

use super::{Crop, xmlparser};
use crate::camerasxml_parser::crop;

type T = Crop;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T, String> {
    Result::Err(str.to_owned())
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<Crop",
        "<Cropname",
        "<Crop x",
        "<Crop x=",
        "<Crop x=Foo",
        "<Crop x=\"Foo\"",
        "<Crop x=\"42\"",
        "<Crop not_x=\"42\"",
        "<Crop x=\"42\"y",
        "<Crop x=\"42\" y=",
        "<Crop x=\"42\" y=Bar",
        "<Crop x=\"42\" y=\"Bar\"",
        "<Crop x=\"42\" y=\"24\"",
        "<Crop x=\"42\" not_y=\"24\"",
        "<Crop x=\"42\" y=\"24\"width",
        "<Crop x=\"42\" y=\"24\" width",
        "<Crop x=\"42\" y=\"24\" width=",
        "<Crop x=\"42\" y=\"24\" width=Bar",
        "<Crop x=\"42\" y=\"24\" width=\"Bar\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\"",
        "<Crop x=\"42\" y=\"24\" not_width=\"22\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\"height",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=Bar",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"Bar\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" not_height=\"44\"",
        "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/",
        "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"-1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"0\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"1\"/>",
        "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T>)> = vec![
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
            "<Crop",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Crop\")`",
            ),
        ),
        (
            "<Cropname",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Cropname\")`",
            ),
        ),
        (
            "<Crop x",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"x\")`",
            ),
        ),
        (
            "<Crop x=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Crop x=\"Foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Foo\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop not_x=\"42\"",
            Err(
                "Error while parsing attribute, expected `\"x\"`, but instead found: `\"not_x\"`",
            ),
        ),
        (
            "<Crop x=\"42\"y",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"y\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" not_y=\"24\"",
            Err(
                "Error while parsing attribute, expected `\"y\"`, but instead found: `\"not_y\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\"width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"width\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" not_width=\"22\"",
            Err(
                "Error while parsing attribute, expected `\"width\"`, but instead found: `\"not_width\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\"height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"height\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" not_height=\"44\"",
            Err(
                "Error while parsing attribute, expected `\"height\"`, but instead found: `\"not_height\"`",
            ),
        ),
        (
            "<Crop x=\"42\" y=\"24\" width=\"22\" height=\"44\"/",
            Err("While trying to match `\"Gt\"`, encountered end of stream"),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"0\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"0\" width=\"-1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"0\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"1\" width=\"-1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(0),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"0\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"0\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"0\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"0\" width=\"-1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(0),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"0\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(0)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Absolute(RowLength::new(1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"0\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(0)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Absolute(RowCount::new(1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"1\" width=\"-1\" height=\"-1\"/>",
            Ok(Crop::new(
                crop::AbsoluteCropPosition::new(Coord2D::new(
                    RowIndex::new(1),
                    ColIndex::new(1),
                )),
                crop::CropSize::new(
                    crop::Width::Relative(ColOffset::new(-1)),
                    crop::Height::Relative(RowOffset::new(-1)),
                ),
            )),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"0\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"1\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"0\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"0\" width=\"-1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"0\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"1\" width=\"-1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"0\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"0\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
        (
            "<Crop x=\"-1\" y=\"-1\" width=\"-1\" height=\"-1\"/>",
            Err("Crop x/y must be non-negative"),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T>(input)));
    }
    assert_eq!(results, expected);
}
