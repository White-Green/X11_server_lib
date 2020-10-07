#![allow(unused_imports)]
#![deny(dead_code)]

mod request {
    use std::collections::HashSet;
    use std::io::{BufReader, BufWriter};
    use std::iter::FromIterator;

    use crate::read_util::{ByteOrder, Readable, Writable};
    use crate::request::create_gc::{CreateGCRequest, CreateGCValue, CreateGCValueArcMode, CreateGCValueCapStyle, CreateGCValueFillRule, CreateGCValueFillStyle, CreateGCValueFunction, CreateGCValueJoinStyle, CreateGCValueLineStyle, CreateGCValueMaskValue, CreateGCValueSubwindowMode};

    #[test]
    fn read_test() {
        let input = [0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0];
        let value = CreateGCRequest::read(&mut BufReader::new(&input[..]), &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, CreateGCRequest {
            cid: 1,
            drawable: 2,
            value_mask: HashSet::new(),
            value: Default::default(),
        });
        let input = [0, 0, 17, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0x7f, 0xff, 0xff,
            3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 7, 1, 1, 1, 1, 1, 0, 0, 0, 8, 0, 0, 0, 9, 0, 10, 0, 11, 0, 0, 0, 12, 1, 1, 0, 13, 0, 14, 0, 0, 0, 15, 0, 16, 17, 1, 0, 0,
        ];
        let value = CreateGCRequest::read(&mut BufReader::new(&input[..]), &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, CreateGCRequest {
            cid: 1,
            drawable: 2,
            value_mask: HashSet::from_iter([CreateGCValueMaskValue::Function,
                CreateGCValueMaskValue::PlaneMask,
                CreateGCValueMaskValue::Foreground,
                CreateGCValueMaskValue::Background,
                CreateGCValueMaskValue::LineWidth,
                CreateGCValueMaskValue::LineStyle,
                CreateGCValueMaskValue::CapStyle,
                CreateGCValueMaskValue::JoinStyle,
                CreateGCValueMaskValue::FillStyle,
                CreateGCValueMaskValue::FillRule,
                CreateGCValueMaskValue::Tile,
                CreateGCValueMaskValue::Stipple,
                CreateGCValueMaskValue::TileStippleXOrigin,
                CreateGCValueMaskValue::TileStippleYOrigin,
                CreateGCValueMaskValue::Font,
                CreateGCValueMaskValue::SubwindowMode,
                CreateGCValueMaskValue::GraphicsExposures,
                CreateGCValueMaskValue::ClipXOrigin,
                CreateGCValueMaskValue::ClipYOrigin,
                CreateGCValueMaskValue::ClipMask,
                CreateGCValueMaskValue::DashOffset,
                CreateGCValueMaskValue::Dashes,
                CreateGCValueMaskValue::ArcMode].to_vec()),
            value: CreateGCValue {
                function: CreateGCValueFunction::Copy,
                plane_mask: 4,
                foreground: 5,
                background: 6,
                line_width: 7,
                line_style: CreateGCValueLineStyle::OnOffDash,
                cap_style: CreateGCValueCapStyle::Butt,
                join_style: CreateGCValueJoinStyle::Round,
                fill_style: CreateGCValueFillStyle::Tiled,
                fill_rule: CreateGCValueFillRule::Winding,
                tile: 8,
                stipple: 9,
                tile_stipple_x_origin: 10,
                tile_stipple_y_origin: 11,
                font: 12,
                subwindow_mode: CreateGCValueSubwindowMode::IncludeInferiors,
                graphics_exposures: true,
                clip_x_origin: 13,
                clip_y_origin: 14,
                clip_mask: Some(15),
                dash_offset: 16,
                dashes: 17,
                arc_mode: CreateGCValueArcMode::PieSlice,
            },
        });
    }

    #[test]
    fn write_test() {
        let value = CreateGCRequest {
            cid: 1,
            drawable: 2,
            value_mask: HashSet::new(),
            value: Default::default(),
        };
        let mut buffer = [255; 17];
        CreateGCRequest::write(&mut BufWriter::new(&mut buffer[..]), value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(buffer, [55, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 255]);
        let value = CreateGCRequest {
            cid: 1,
            drawable: 2,
            value_mask: HashSet::from_iter([CreateGCValueMaskValue::Function,
                CreateGCValueMaskValue::PlaneMask,
                CreateGCValueMaskValue::Foreground,
                CreateGCValueMaskValue::Background,
                CreateGCValueMaskValue::LineWidth,
                CreateGCValueMaskValue::LineStyle,
                CreateGCValueMaskValue::CapStyle,
                CreateGCValueMaskValue::JoinStyle,
                CreateGCValueMaskValue::FillStyle,
                CreateGCValueMaskValue::FillRule,
                CreateGCValueMaskValue::Tile,
                CreateGCValueMaskValue::Stipple,
                CreateGCValueMaskValue::TileStippleXOrigin,
                CreateGCValueMaskValue::TileStippleYOrigin,
                CreateGCValueMaskValue::Font,
                CreateGCValueMaskValue::SubwindowMode,
                CreateGCValueMaskValue::GraphicsExposures,
                CreateGCValueMaskValue::ClipXOrigin,
                CreateGCValueMaskValue::ClipYOrigin,
                CreateGCValueMaskValue::ClipMask,
                CreateGCValueMaskValue::DashOffset,
                CreateGCValueMaskValue::Dashes,
                CreateGCValueMaskValue::ArcMode].to_vec()),
            value: CreateGCValue {
                function: CreateGCValueFunction::And,
                plane_mask: 4,
                foreground: 5,
                background: 6,
                line_width: 7,
                line_style: CreateGCValueLineStyle::OnOffDash,
                cap_style: CreateGCValueCapStyle::Round,
                join_style: CreateGCValueJoinStyle::Round,
                fill_style: CreateGCValueFillStyle::Tiled,
                fill_rule: CreateGCValueFillRule::Winding,
                tile: 8,
                stipple: 9,
                tile_stipple_x_origin: 10,
                tile_stipple_y_origin: 11,
                font: 12,
                subwindow_mode: CreateGCValueSubwindowMode::IncludeInferiors,
                graphics_exposures: false,
                clip_x_origin: 13,
                clip_y_origin: 14,
                clip_mask: Some(15),
                dash_offset: 16,
                dashes: 17,
                arc_mode: CreateGCValueArcMode::Chord,
            },
        };
        let mut buffer = [255; 69];
        CreateGCRequest::write(&mut BufWriter::new(&mut buffer[..]), value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(buffer, [55, 0, 0, 17, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0x7f, 0xff, 0xff,
            1, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 7, 1, 2, 1, 1, 1, 0, 0, 0, 8, 0, 0, 0, 9, 0, 10, 0, 11, 0, 0, 0, 12, 1, 0, 0, 13, 0, 14, 0, 0, 0, 15, 0, 16, 17, 0, 0, 0, 255]);
    }
}