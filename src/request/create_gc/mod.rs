use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};

mod test;

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueFunction {
    Clear,
    And,
    AndReverse,
    Copy,
    AndInverted,
    NoOp,
    Xor,
    Or,
    Nor,
    Equiv,
    Invert,
    OrReverse,
    CopyInverted,
    OrInverted,
    Nand,
    Set,
}

impl Readable for CreateGCValueFunction {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::Clear),
            1 => Ok(Self::And),
            2 => Ok(Self::AndReverse),
            3 => Ok(Self::Copy),
            4 => Ok(Self::AndInverted),
            5 => Ok(Self::NoOp),
            6 => Ok(Self::Xor),
            7 => Ok(Self::Or),
            8 => Ok(Self::Nor),
            9 => Ok(Self::Equiv),
            10 => Ok(Self::Invert),
            11 => Ok(Self::OrReverse),
            12 => Ok(Self::CopyInverted),
            13 => Ok(Self::OrInverted),
            14 => Ok(Self::Nand),
            15 => Ok(Self::Set),
            _ => Err(Error::InvalidValue("CreateGCValueFunction")),
        }
    }
}

impl Writable for CreateGCValueFunction {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::Clear => 0,
            Self::And => 1,
            Self::AndReverse => 2,
            Self::Copy => 3,
            Self::AndInverted => 4,
            Self::NoOp => 5,
            Self::Xor => 6,
            Self::Or => 7,
            Self::Nor => 8,
            Self::Equiv => 9,
            Self::Invert => 10,
            Self::OrReverse => 11,
            Self::CopyInverted => 12,
            Self::OrInverted => 13,
            Self::Nand => 14,
            Self::Set => 15,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueLineStyle {
    Solid,
    OnOffDash,
    DoubleDash,
}

impl Readable for CreateGCValueLineStyle {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::Solid),
            1 => Ok(Self::OnOffDash),
            2 => Ok(Self::DoubleDash),
            _ => Err(Error::InvalidValue("CreateGCValueLineStyle")),
        }
    }
}

impl Writable for CreateGCValueLineStyle {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::Solid => 0,
            Self::OnOffDash => 1,
            Self::DoubleDash => 2,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueCapStyle {
    NotLast,
    Butt,
    Round,
    Projecting,
}

impl Readable for CreateGCValueCapStyle {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::NotLast),
            1 => Ok(Self::Butt),
            2 => Ok(Self::Round),
            3 => Ok(Self::Projecting),
            _ => Err(Error::InvalidValue("CreateGCValueCapStyle")),
        }
    }
}

impl Writable for CreateGCValueCapStyle {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::NotLast => 0,
            Self::Butt => 1,
            Self::Round => 2,
            Self::Projecting => 3,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueJoinStyle {
    Miter,
    Round,
    Bevel,
}

impl Readable for CreateGCValueJoinStyle {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::Miter),
            1 => Ok(Self::Round),
            2 => Ok(Self::Bevel),
            _ => Err(Error::InvalidValue("CreateGCValueJoinStyle")),
        }
    }
}

impl Writable for CreateGCValueJoinStyle {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::Miter => 0,
            Self::Round => 1,
            Self::Bevel => 2,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueFillStyle {
    Solid,
    Tiled,
    Stippled,
    OpaqueStippled,
}

impl Readable for CreateGCValueFillStyle {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::Solid),
            1 => Ok(Self::Tiled),
            2 => Ok(Self::Stippled),
            3 => Ok(Self::OpaqueStippled),
            _ => Err(Error::InvalidValue("CreateGCValueFillStyle")),
        }
    }
}

impl Writable for CreateGCValueFillStyle {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::Solid => 0,
            Self::Tiled => 1,
            Self::Stippled => 2,
            Self::OpaqueStippled => 3,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueFillRule {
    EvenOdd,
    Winding,
}

impl Readable for CreateGCValueFillRule {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::EvenOdd),
            1 => Ok(Self::Winding),
            _ => Err(Error::InvalidValue("CreateGCValueFillRule")),
        }
    }
}

impl Writable for CreateGCValueFillRule {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::EvenOdd => 0,
            Self::Winding => 1,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueSubwindowMode {
    ClipByChildren,
    IncludeInferiors,
}

impl Readable for CreateGCValueSubwindowMode {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::ClipByChildren),
            1 => Ok(Self::IncludeInferiors),
            _ => Err(Error::InvalidValue("CreateGCValueSubwindowMode")),
        }
    }
}

impl Writable for CreateGCValueSubwindowMode {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::ClipByChildren => 0,
            Self::IncludeInferiors => 1,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateGCValueArcMode {
    Chord,
    PieSlice,
}

impl Readable for CreateGCValueArcMode {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Self::Chord),
            1 => Ok(Self::PieSlice),
            _ => Err(Error::InvalidValue("CreateGCValueArcMode")),
        }
    }
}

impl Writable for CreateGCValueArcMode {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let value: u8 = match data {
            Self::Chord => 0,
            Self::PieSlice => 1,
        };
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateGCValue {
    pub function: CreateGCValueFunction,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u16,
    pub line_style: CreateGCValueLineStyle,
    pub cap_style: CreateGCValueCapStyle,
    pub join_style: CreateGCValueJoinStyle,
    pub fill_style: CreateGCValueFillStyle,
    pub fill_rule: CreateGCValueFillRule,
    pub tile: u32,
    pub stipple: u32,
    pub tile_stipple_x_origin: i16,
    pub tile_stipple_y_origin: i16,
    pub font: u32,
    pub subwindow_mode: CreateGCValueSubwindowMode,
    pub graphics_exposures: bool,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
    pub clip_mask: Option<u32>,
    pub dash_offset: u16,
    pub dashes: u8,
    pub arc_mode: CreateGCValueArcMode,
}

impl Default for CreateGCValue {
    fn default() -> Self {
        CreateGCValue {
            function: CreateGCValueFunction::Copy,
            plane_mask: 0xffffffff,
            foreground: 0,
            background: 1,
            line_width: 0,
            line_style: CreateGCValueLineStyle::Solid,
            cap_style: CreateGCValueCapStyle::Butt,
            join_style: CreateGCValueJoinStyle::Miter,
            fill_style: CreateGCValueFillStyle::Solid,
            fill_rule: CreateGCValueFillRule::EvenOdd,
            tile: 0,
            stipple: 0,
            tile_stipple_x_origin: 0,
            tile_stipple_y_origin: 0,
            font: 0,
            subwindow_mode: CreateGCValueSubwindowMode::ClipByChildren,
            graphics_exposures: true,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: None,
            dash_offset: 0,
            dashes: 4,
            arc_mode: CreateGCValueArcMode::PieSlice,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CreateGCValueMaskValue {
    Function,
    PlaneMask,
    Foreground,
    Background,
    LineWidth,
    LineStyle,
    CapStyle,
    JoinStyle,
    FillStyle,
    FillRule,
    Tile,
    Stipple,
    TileStippleXOrigin,
    TileStippleYOrigin,
    Font,
    SubwindowMode,
    GraphicsExposures,
    ClipXOrigin,
    ClipYOrigin,
    ClipMask,
    DashOffset,
    Dashes,
    ArcMode,
}

impl Readable for HashSet<CreateGCValueMaskValue> {
    fn read(stream: &mut BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        let mut value = HashSet::new();
        let mask: u32 = stream.read_value(order)?;
        if mask & 0x00000001 != 0 { value.insert(CreateGCValueMaskValue::Function); }
        if mask & 0x00000002 != 0 { value.insert(CreateGCValueMaskValue::PlaneMask); }
        if mask & 0x00000004 != 0 { value.insert(CreateGCValueMaskValue::Foreground); }
        if mask & 0x00000008 != 0 { value.insert(CreateGCValueMaskValue::Background); }
        if mask & 0x00000010 != 0 { value.insert(CreateGCValueMaskValue::LineWidth); }
        if mask & 0x00000020 != 0 { value.insert(CreateGCValueMaskValue::LineStyle); }
        if mask & 0x00000040 != 0 { value.insert(CreateGCValueMaskValue::CapStyle); }
        if mask & 0x00000080 != 0 { value.insert(CreateGCValueMaskValue::JoinStyle); }
        if mask & 0x00000100 != 0 { value.insert(CreateGCValueMaskValue::FillStyle); }
        if mask & 0x00000200 != 0 { value.insert(CreateGCValueMaskValue::FillRule); }
        if mask & 0x00000400 != 0 { value.insert(CreateGCValueMaskValue::Tile); }
        if mask & 0x00000800 != 0 { value.insert(CreateGCValueMaskValue::Stipple); }
        if mask & 0x00001000 != 0 { value.insert(CreateGCValueMaskValue::TileStippleXOrigin); }
        if mask & 0x00002000 != 0 { value.insert(CreateGCValueMaskValue::TileStippleYOrigin); }
        if mask & 0x00004000 != 0 { value.insert(CreateGCValueMaskValue::Font); }
        if mask & 0x00008000 != 0 { value.insert(CreateGCValueMaskValue::SubwindowMode); }
        if mask & 0x00010000 != 0 { value.insert(CreateGCValueMaskValue::GraphicsExposures); }
        if mask & 0x00020000 != 0 { value.insert(CreateGCValueMaskValue::ClipXOrigin); }
        if mask & 0x00040000 != 0 { value.insert(CreateGCValueMaskValue::ClipYOrigin); }
        if mask & 0x00080000 != 0 { value.insert(CreateGCValueMaskValue::ClipMask); }
        if mask & 0x00100000 != 0 { value.insert(CreateGCValueMaskValue::DashOffset); }
        if mask & 0x00200000 != 0 { value.insert(CreateGCValueMaskValue::Dashes); }
        if mask & 0x00400000 != 0 { value.insert(CreateGCValueMaskValue::ArcMode); }
        Ok(value)
    }
}

impl Writable for HashSet<CreateGCValueMaskValue> {
    fn write(stream: &mut BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let mut value = 0u32;
        for data in data {
            let mask = match data {
                CreateGCValueMaskValue::Function => 0x00000001,
                CreateGCValueMaskValue::PlaneMask => 0x00000002,
                CreateGCValueMaskValue::Foreground => 0x00000004,
                CreateGCValueMaskValue::Background => 0x00000008,
                CreateGCValueMaskValue::LineWidth => 0x00000010,
                CreateGCValueMaskValue::LineStyle => 0x00000020,
                CreateGCValueMaskValue::CapStyle => 0x00000040,
                CreateGCValueMaskValue::JoinStyle => 0x00000080,
                CreateGCValueMaskValue::FillStyle => 0x00000100,
                CreateGCValueMaskValue::FillRule => 0x00000200,
                CreateGCValueMaskValue::Tile => 0x00000400,
                CreateGCValueMaskValue::Stipple => 0x00000800,
                CreateGCValueMaskValue::TileStippleXOrigin => 0x00001000,
                CreateGCValueMaskValue::TileStippleYOrigin => 0x00002000,
                CreateGCValueMaskValue::Font => 0x00004000,
                CreateGCValueMaskValue::SubwindowMode => 0x00008000,
                CreateGCValueMaskValue::GraphicsExposures => 0x00010000,
                CreateGCValueMaskValue::ClipXOrigin => 0x00020000,
                CreateGCValueMaskValue::ClipYOrigin => 0x00040000,
                CreateGCValueMaskValue::ClipMask => 0x00080000,
                CreateGCValueMaskValue::DashOffset => 0x00100000,
                CreateGCValueMaskValue::Dashes => 0x00200000,
                CreateGCValueMaskValue::ArcMode => 0x00400000,
            };
            value |= mask;
        }
        stream.write_value(value, order)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateGCRequest {
    pub cid: u32,
    pub drawable: u32,
    pub value_mask: HashSet<CreateGCValueMaskValue>,
    pub value: CreateGCValue,
}

impl Readable for CreateGCRequest {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        read_specified_length(stream, &mut [0; 1], 1)?;
        let length = stream.read_value::<u16>(order)? as usize - 4;
        let cid = stream.read_value(order)?;
        let drawable = stream.read_value(order)?;
        let value_mask: HashSet<_> = stream.read_value(order)?;
        let mut value = CreateGCValue::default();
        let mut buffer = vec![0; length << 2];
        read_specified_length(stream, &mut buffer[..], length << 2)?;
        let mut buffer = BufReader::new(&buffer[..]);
        if value_mask.contains(&CreateGCValueMaskValue::Function) { value.function = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::PlaneMask) { value.plane_mask = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Foreground) { value.foreground = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Background) { value.background = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::LineWidth) { value.line_width = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::LineStyle) { value.line_style = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::CapStyle) { value.cap_style = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::JoinStyle) { value.join_style = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::FillStyle) { value.fill_style = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::FillRule) { value.fill_rule = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Tile) { value.tile = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Stipple) { value.stipple = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::TileStippleXOrigin) { value.tile_stipple_x_origin = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::TileStippleYOrigin) { value.tile_stipple_y_origin = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Font) { value.font = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::SubwindowMode) { value.subwindow_mode = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::GraphicsExposures) { value.graphics_exposures = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::ClipXOrigin) { value.clip_x_origin = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::ClipYOrigin) { value.clip_y_origin = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::ClipMask) {
            value.clip_mask = match buffer.read_value(order)? {
                0 => None,
                other => Some(other),
            };
        }
        if value_mask.contains(&CreateGCValueMaskValue::DashOffset) { value.dash_offset = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::Dashes) { value.dashes = buffer.read_value(order)?; }
        if value_mask.contains(&CreateGCValueMaskValue::ArcMode) { value.arc_mode = buffer.read_value(order)?; }
        Ok(CreateGCRequest {
            cid,
            drawable,
            value_mask,
            value,
        })
    }
}

impl Writable for CreateGCRequest {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        let mut buffer = [0; 52];
        let mut value_writer = BufWriter::new(&mut buffer[..]);
        let default_value = CreateGCValue::default();
        let mut written_size = 0;
        let mut written = HashSet::new();
        if data.value.function != default_value.function {
            value_writer.write_value(data.value.function, order)?;
            written.insert(CreateGCValueMaskValue::Function);
            written_size += 1;
        }
        if data.value.plane_mask != default_value.plane_mask {
            value_writer.write_value(data.value.plane_mask, order)?;
            written.insert(CreateGCValueMaskValue::PlaneMask);
            written_size += 4;
        }
        if data.value.foreground != default_value.foreground {
            value_writer.write_value(data.value.foreground, order)?;
            written.insert(CreateGCValueMaskValue::Foreground);
            written_size += 4;
        }
        if data.value.background != default_value.background {
            value_writer.write_value(data.value.background, order)?;
            written.insert(CreateGCValueMaskValue::Background);
            written_size += 4;
        }
        if data.value.line_width != default_value.line_width {
            value_writer.write_value(data.value.line_width, order)?;
            written.insert(CreateGCValueMaskValue::LineWidth);
            written_size += 2;
        }
        if data.value.line_style != default_value.line_style {
            value_writer.write_value(data.value.line_style, order)?;
            written.insert(CreateGCValueMaskValue::LineStyle);
            written_size += 1;
        }
        if data.value.cap_style != default_value.cap_style {
            value_writer.write_value(data.value.cap_style, order)?;
            written.insert(CreateGCValueMaskValue::CapStyle);
            written_size += 1;
        }
        if data.value.join_style != default_value.join_style {
            value_writer.write_value(data.value.join_style, order)?;
            written.insert(CreateGCValueMaskValue::JoinStyle);
            written_size += 1;
        }
        if data.value.fill_style != default_value.fill_style {
            value_writer.write_value(data.value.fill_style, order)?;
            written.insert(CreateGCValueMaskValue::FillStyle);
            written_size += 1;
        }
        if data.value.fill_rule != default_value.fill_rule {
            value_writer.write_value(data.value.fill_rule, order)?;
            written.insert(CreateGCValueMaskValue::FillRule);
            written_size += 1;
        }
        if data.value.tile != default_value.tile {
            value_writer.write_value(data.value.tile, order)?;
            written.insert(CreateGCValueMaskValue::Tile);
            written_size += 4;
        }
        if data.value.stipple != default_value.stipple {
            value_writer.write_value(data.value.stipple, order)?;
            written.insert(CreateGCValueMaskValue::Stipple);
            written_size += 4;
        }
        if data.value.tile_stipple_x_origin != default_value.tile_stipple_x_origin {
            value_writer.write_value(data.value.tile_stipple_x_origin, order)?;
            written.insert(CreateGCValueMaskValue::TileStippleXOrigin);
            written_size += 2;
        }
        if data.value.tile_stipple_y_origin != default_value.tile_stipple_y_origin {
            value_writer.write_value(data.value.tile_stipple_y_origin, order)?;
            written.insert(CreateGCValueMaskValue::TileStippleYOrigin);
            written_size += 2;
        }
        if data.value.font != default_value.font {
            value_writer.write_value(data.value.font, order)?;
            written.insert(CreateGCValueMaskValue::Font);
            written_size += 4;
        }
        if data.value.subwindow_mode != default_value.subwindow_mode {
            value_writer.write_value(data.value.subwindow_mode, order)?;
            written.insert(CreateGCValueMaskValue::SubwindowMode);
            written_size += 1;
        }
        if data.value.graphics_exposures != default_value.graphics_exposures {
            value_writer.write_value(data.value.graphics_exposures, order)?;
            written.insert(CreateGCValueMaskValue::GraphicsExposures);
            written_size += 1;
        }
        if data.value.clip_x_origin != default_value.clip_x_origin {
            value_writer.write_value(data.value.clip_x_origin, order)?;
            written.insert(CreateGCValueMaskValue::ClipXOrigin);
            written_size += 2;
        }
        if data.value.clip_y_origin != default_value.clip_y_origin {
            value_writer.write_value(data.value.clip_y_origin, order)?;
            written.insert(CreateGCValueMaskValue::ClipYOrigin);
            written_size += 2;
        }
        if data.value.clip_mask != default_value.clip_mask {
            value_writer.write_value(data.value.clip_mask.unwrap_or(0), order)?;
            written.insert(CreateGCValueMaskValue::ClipMask);
            written_size += 4;
        }
        if data.value.dash_offset != default_value.dash_offset {
            value_writer.write_value(data.value.dash_offset, order)?;
            written.insert(CreateGCValueMaskValue::DashOffset);
            written_size += 2;
        }
        if data.value.dashes != default_value.dashes {
            value_writer.write_value(data.value.dashes, order)?;
            written.insert(CreateGCValueMaskValue::Dashes);
            written_size += 1;
        }
        if data.value.arc_mode != default_value.arc_mode {
            value_writer.write_value(data.value.arc_mode, order)?;
            written.insert(CreateGCValueMaskValue::ArcMode);
            written_size += 1;
        }
        value_writer.flush().map_err(|e| Error::IoError(e))?;
        drop(value_writer);
        let written_size = (written_size + 3) & !3;
        stream.write_value::<u8>(55, order)?;
        stream.write_all(&[0]).map_err(|e| Error::IoError(e))?;
        stream.write_value((written_size >> 2) as u16 + 4, order)?;
        stream.write_value(data.cid, order)?;
        stream.write_value(data.drawable, order)?;
        stream.write_value(written, order)?;
        stream.write_all(&buffer[..written_size]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateGCResponse;

impl Readable for CreateGCResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        Ok(Self)
    }
}

impl Writable for CreateGCResponse {
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        Ok(())
    }
}
