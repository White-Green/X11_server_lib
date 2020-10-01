/// https://www.x.org/releases/current/doc/xproto/x11protocol.html#Encoding::Connection_Setup
use std::collections::HashSet;
use std::io::{Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};

mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionSetupInformation {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name: String,
    pub authorization_protocol_data: String,
}

pub fn read_setup(stream: &mut impl Read, buffer: &mut [u8]) -> Result<(ByteOrder, ConnectionSetupInformation)> {
    assert!(buffer.len() >= 10);
    read_specified_length(stream, buffer, 2)?;
    let order = match buffer[0] {
        0o102 => { ByteOrder::MSBFirst }
        0o154 => { ByteOrder::LSBFirst }
        _ => { return Err(Error::InvalidValue("byte order")); }
    };
    read_specified_length(stream, buffer, 10)?;
    let protocol_major_version = order.decode(&buffer[0..2]);
    let protocol_minor_version = order.decode(&buffer[2..4]);
    let authorization_protocol_name_length: u16 = order.decode(&buffer[4..6]);
    let authorization_protocol_data_length: u16 = order.decode(&buffer[6..8]);
    let name_length = authorization_protocol_name_length as usize;
    let data_length = authorization_protocol_data_length as usize;

    let mut name_total_length = (((-1isize ^ 3) as usize) & name_length) + ((name_length << 1 | name_length << 2) & 4);
    let mut name = Vec::with_capacity(name_total_length);
    while name_total_length > 0 {
        let read = read_specified_length(stream, buffer, name_total_length)?;
        name.extend_from_slice(&buffer[..read]);
        name_total_length -= read;
    }
    let authorization_protocol_name = std::str::from_utf8(&name[..name_length])
        .map_err(|e| Error::StringError(e))?
        .to_string();

    let mut data_total_length = (((-1isize ^ 3) as usize) & data_length) + ((data_length << 1 | data_length << 2) & 4);
    let mut data = Vec::with_capacity(data_total_length);
    while data_total_length > 0 {
        let read = read_specified_length(stream, buffer, data_total_length)?;
        data.extend_from_slice(&buffer[..read]);
        data_total_length -= read;
    }
    let authorization_protocol_data = std::str::from_utf8(&buffer[..data_length])
        .map_err(|e| Error::StringError(e))?
        .to_string();

    let information = ConnectionSetupInformation {
        protocol_major_version,
        protocol_minor_version,
        authorization_protocol_name,
        authorization_protocol_data,
    };

    Ok((order, information))
}

pub fn write_setup(stream: &mut impl Write, buffer: &mut [u8], order: &ByteOrder, info: ConnectionSetupInformation) -> Result<()> {
    assert!(buffer.len() >= 12);
    buffer[0] =
        match order {
            ByteOrder::MSBFirst => { 0o102 }
            ByteOrder::LSBFirst => { 0o154 }
        };
    order.encode(info.protocol_major_version, &mut buffer[2..4]);
    order.encode(info.protocol_minor_version, &mut buffer[4..6]);
    let name_len = info.authorization_protocol_name.as_bytes().len();
    assert!(name_len <= u16::MAX as usize);
    order.encode(name_len as u16, &mut buffer[6..8]);
    let data_len = info.authorization_protocol_data.as_bytes().len();
    assert!(data_len <= u16::MAX as usize);
    order.encode(data_len as u16, &mut buffer[8..10]);
    stream.write(&buffer[..12]).map_err(|e| Error::IoError(e))?;
    stream.write(info.authorization_protocol_name.as_bytes()).map_err(|e| Error::IoError(e))?;
    stream.write(&buffer[..((!name_len).wrapping_add(1)) & 0b11]).map_err(|e| Error::IoError(e))?;
    stream.write(info.authorization_protocol_data.as_bytes()).map_err(|e| Error::IoError(e))?;
    stream.write(&buffer[..((!data_len).wrapping_add(1)) & 0b11]).map_err(|e| Error::IoError(e))?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConnectionSetupFailed {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub reason: String,
}

impl Readable for ConnectionSetupFailed {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {//最初のopcodeは読まない
        let n = stream.read_value::<u8>(order)? as usize;
        let protocol_major_version = stream.read_value(order)?;
        let protocol_minor_version = stream.read_value(order)?;
        let len = stream.read_value::<u16>(order)? as usize;
        let mut buffer = vec![0; len << 2];
        read_specified_length(stream, &mut buffer[..], len << 2)?;
        let reason = std::str::from_utf8(&buffer[..n]).map_err(|e| Error::StringError(e))?;
        let reason = reason.to_string();
        Ok(ConnectionSetupFailed {
            protocol_major_version,
            protocol_minor_version,
            reason,
        })
    }
}

impl Writable for ConnectionSetupFailed {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {//最初のopcodeも送る
        stream.write_value(0u8, order)?;
        stream.write_value(data.reason.len() as u8, order)?;
        stream.write_value(data.protocol_major_version, order)?;
        stream.write_value(data.protocol_minor_version, order)?;
        let len = data.reason.len() as u16;
        let q = (!len).wrapping_add(1) & 3;
        stream.write_value((len + q) >> 2, order)?;
        stream.write(data.reason.as_bytes()).map_err(|e| Error::IoError(e))?;
        let buf = vec![0; q as usize];
        stream.write(&buf[..]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConnectionSetupAuthenticate {
    pub reason: String,
}

impl Readable for ConnectionSetupAuthenticate {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {//最初のopcodeは読まない
        read_specified_length(stream, &mut [0; 5], 5)?;
        let len = stream.read_value::<u16>(order)? as usize;
        let mut len = len << 2;
        let mut buffer = vec![0; len];
        read_specified_length(stream, &mut buffer[..], len)?;
        while len > 0 && buffer[len - 1] == 0 { len -= 1; }
        let reason = std::str::from_utf8(&buffer[..len]).map_err(|e| Error::StringError(e))?;
        let reason = reason.to_string();//ここreasonの長さがわからないから適当に末尾の0消すぐらいでやってる
        Ok(ConnectionSetupAuthenticate {
            reason,
        })
    }
}

impl Writable for ConnectionSetupAuthenticate {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {//最初のopcodeも送る
        stream.write_value(2u8, order)?;
        stream.write(&[0; 5]).map_err(|e| Error::IoError(e))?;
        let len = data.reason.len() as u16;
        let q = (!len).wrapping_add(1) & 3;
        stream.write_value((len + q) >> 2, order)?;
        stream.write(data.reason.as_bytes()).map_err(|e| Error::IoError(e))?;
        for _ in 0..q {
            stream.write_value(0u8, order)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImageByteOrder {
    LSBFirst,
    MSBFirst,
}

impl Readable for ImageByteOrder {
    fn read(stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        let mut buffer = [0];
        read_specified_length(stream, &mut buffer[..], 1)?;
        match buffer[0] {
            0 => { Ok(Self::LSBFirst) }
            1 => { Ok(Self::MSBFirst) }
            _ => { Err(Error::InvalidValue("ImageByteOrder")) }
        }
    }
}

impl Writable for ImageByteOrder {
    fn write(stream: &mut impl Write, data: Self, _order: &ByteOrder) -> Result<()> {
        let buffer = [
            match data {
                ImageByteOrder::LSBFirst => { 0 }
                ImageByteOrder::MSBFirst => { 1 }
            }
        ];
        match stream.write(&buffer[..]) {
            Ok(i) if i == 1 => { Ok(()) }
            Err(e) => { Err(Error::IoError(e)) }
            _ => { Err(Error::UnknownError) }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BitmapFormatBitOrder {
    LeastSignificant,
    MostSignificant,
}

impl Readable for BitmapFormatBitOrder {
    fn read(stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        let mut buffer = [0];
        read_specified_length(stream, &mut buffer[..], 1)?;
        match buffer[0] {
            0 => { Ok(Self::LeastSignificant) }
            1 => { Ok(Self::MostSignificant) }
            _ => { Err(Error::InvalidValue("BitmapFormatBitOrder")) }
        }
    }
}

impl Writable for BitmapFormatBitOrder {
    fn write(stream: &mut impl Write, data: Self, _order: &ByteOrder) -> Result<()> {
        let buffer = [
            match data {
                BitmapFormatBitOrder::LeastSignificant => { 0 }
                BitmapFormatBitOrder::MostSignificant => { 1 }
            }
        ];
        match stream.write(&buffer[..]) {
            Ok(i)if i == 1 => { Ok(()) }
            Err(e) => { Err(Error::IoError(e)) }
            _ => { Err(Error::UnknownError) }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}

impl Readable for Format {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        let depth = stream.read_value(order)?;
        let bits_per_pixel = stream.read_value(order)?;
        let scanline_pad = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 5][..], 5)?;
        Ok(Format {
            depth,
            bits_per_pixel,
            scanline_pad,
        })
    }
}

impl Writable for Format {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value(data.depth, order)?;
        stream.write_value(data.bits_per_pixel, order)?;
        stream.write_value(data.scanline_pad, order)?;
        stream.write(&[0; 5][..]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

impl Readable for BackingStores {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(BackingStores::Never),
            1 => Ok(BackingStores::WhenMapped),
            2 => Ok(BackingStores::Always),
            _ => Err(Error::UnknownError),
        }
    }
}

impl Writable for BackingStores {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        let value = match data {
            BackingStores::Never => 0,
            BackingStores::WhenMapped => 1,
            BackingStores::Always => 2,
        };
        stream.write_value::<u8>(value, order)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Class {
    StaticGray,
    GrayScale,
    StaticColor,
    PseudoColor,
    TrueColor,
    DirectColor,
}

impl Readable for Class {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            0 => Ok(Class::StaticGray),
            1 => Ok(Class::GrayScale),
            2 => Ok(Class::StaticColor),
            3 => Ok(Class::PseudoColor),
            4 => Ok(Class::TrueColor),
            5 => Ok(Class::DirectColor),
            _ => Err(Error::UnknownError),
        }
    }
}

impl Writable for Class {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        let value = match data {
            Class::StaticGray => 0,
            Class::GrayScale => 1,
            Class::StaticColor => 2,
            Class::PseudoColor => 3,
            Class::TrueColor => 4,
            Class::DirectColor => 5,
        };
        stream.write_value::<u8>(value, order)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VisualType {
    pub visual_id: u32,
    pub class: Class,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}

impl Readable for VisualType {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        let visual_id = stream.read_value(order)?;
        let class = stream.read_value(order)?;
        let bits_per_rgb_value = stream.read_value(order)?;
        let colormap_entries = stream.read_value(order)?;
        let red_mask = stream.read_value(order)?;
        let green_mask = stream.read_value(order)?;
        let blue_mask = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 4], 4)?;
        Ok(VisualType {
            visual_id,
            class,
            bits_per_rgb_value,
            colormap_entries,
            red_mask,
            green_mask,
            blue_mask,
        })
    }
}

impl Writable for VisualType {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value(data.visual_id, order)?;
        stream.write_value(data.class, order)?;
        stream.write_value(data.bits_per_rgb_value, order)?;
        stream.write_value(data.colormap_entries, order)?;
        stream.write_value(data.red_mask, order)?;
        stream.write_value(data.green_mask, order)?;
        stream.write_value(data.blue_mask, order)?;
        stream.write(&[0; 4][..]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<VisualType>,
}

impl Readable for Depth {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        let depth = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 1], 1)?;
        let n = stream.read_value::<u16>(order)? as usize;
        read_specified_length(stream, &mut [0; 4], 4)?;
        let mut visuals = Vec::with_capacity(n);
        for _ in 0..n {
            visuals.push(stream.read_value(order)?);
        }
        Ok(Depth {
            depth,
            visuals,
        })
    }
}

impl Writable for Depth {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value(data.depth, order)?;
        stream.write(&[0; 1]).map_err(|e| Error::IoError(e))?;
        stream.write_value(data.visuals.len() as u16, order)?;
        stream.write(&[0; 4]).map_err(|e| Error::IoError(e))?;
        for visual in data.visuals {
            stream.write_value(visual, order)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event {
    //TODO:別のとこで定義したほうがいいので移動させる
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState,
    Exposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    OwnerGrabButton,
}

impl Readable for HashSet<Event> {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        const VALUES: [Event; 25] = [
            Event::KeyPress,
            Event::KeyRelease,
            Event::ButtonPress,
            Event::ButtonRelease,
            Event::EnterWindow,
            Event::LeaveWindow,
            Event::PointerMotion,
            Event::PointerMotionHint,
            Event::Button1Motion,
            Event::Button2Motion,
            Event::Button3Motion,
            Event::Button4Motion,
            Event::Button5Motion,
            Event::ButtonMotion,
            Event::KeymapState,
            Event::Exposure,
            Event::VisibilityChange,
            Event::StructureNotify,
            Event::ResizeRedirect,
            Event::SubstructureNotify,
            Event::SubstructureRedirect,
            Event::FocusChange,
            Event::PropertyChange,
            Event::ColormapChange,
            Event::OwnerGrabButton,
        ];
        let value = stream.read_value::<u32>(order)?;
        if value & 0xFE000000 != 0 { return Err(Error::InvalidValue("Set of Event")); }
        let count = {
            let value = (value & 0x55555555) + ((value >> 1) & 0x55555555);
            let value = (value & 0x33333333) + ((value >> 2) & 0x33333333);
            let value = (value & 0x0F0F0F0F) + ((value >> 4) & 0x0F0F0F0F);
            let value = (value & 0x00FF00FF) + ((value >> 8) & 0x00FF00FF);
            (value & 0x0000FFFF) + ((value >> 16) & 0x0000FFFF)
        };
        let mut result = HashSet::with_capacity(count as usize);
        for i in 0..VALUES.len() {
            if ((value >> i) & 1) == 1 {
                result.insert(VALUES[i].clone());
            }
        }
        Ok(result)
    }
}

impl Writable for HashSet<Event> {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()> {
        const VALUES: [Event; 25] = [
            Event::KeyPress,
            Event::KeyRelease,
            Event::ButtonPress,
            Event::ButtonRelease,
            Event::EnterWindow,
            Event::LeaveWindow,
            Event::PointerMotion,
            Event::PointerMotionHint,
            Event::Button1Motion,
            Event::Button2Motion,
            Event::Button3Motion,
            Event::Button4Motion,
            Event::Button5Motion,
            Event::ButtonMotion,
            Event::KeymapState,
            Event::Exposure,
            Event::VisibilityChange,
            Event::StructureNotify,
            Event::ResizeRedirect,
            Event::SubstructureNotify,
            Event::SubstructureRedirect,
            Event::FocusChange,
            Event::PropertyChange,
            Event::ColormapChange,
            Event::OwnerGrabButton,
        ];
        let mut value: u32 = 0;
        for i in 0..VALUES.len() {
            if data.contains(&VALUES[i]) {
                value |= 1 << i;
            }
        }
        stream.write_value(value, order)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Screen {
    pub root: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: HashSet<Event>,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub save_unders: bool,
    pub root_depth: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConnectionSetupSuccess {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: ImageByteOrder,
    pub bitmap_format_bit_order: BitmapFormatBitOrder,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub vendor: String,
    pub pixmap_formats: Vec<Format>,
    pub roots: Vec<Screen>,
}