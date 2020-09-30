use std::collections::HashSet;
/// https://www.x.org/releases/current/doc/xproto/x11protocol.html#Encoding::Connection_Setup
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
enum ImageByteOrder {
    LSBFirst,
    MSBFirst,
}

#[derive(Debug, PartialEq, Clone)]
enum BitmapFormatBitOrder {
    MostSignificant,
    LeastSignificant,
}

#[derive(Debug, PartialEq, Clone)]
struct Format {
    depth: u8,
    bits_per_pixel: u8,
    scanline_pad: u8,
}

#[derive(Debug, PartialEq, Clone)]
enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

#[derive(Debug, PartialEq, Clone)]
enum Class {
    StaticGray,
    GrayScale,
    StaticColor,
    PseudoColor,
    TrueColor,
    DirectColor,
}

#[derive(Debug, PartialEq, Clone)]
struct VisualType {
    visual_id: u32,
    class: Class,
    bits_per_rgb_value: u8,
    colormap_entries: u16,
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct Depth {
    depth: u8,
    visuals: Vec<VisualType>,
}

#[derive(Debug, PartialEq, Clone)]
enum Event {
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

#[derive(Debug, PartialEq, Clone)]
struct Screen {
    root: u32,
    default_colormap: u32,
    white_pixel: u32,
    black_pixel: u32,
    current_input_masks: HashSet<Event>,
    width_in_pixels: u16,
    height_in_pixels: u16,
    width_in_millimeters: u16,
    height_in_millimeters: u16,
    min_installed_maps: u16,
    max_installed_maps: u16,
    root_visual: u32,
    save_unders: bool,
    root_depth: u8,

}

#[derive(Debug, PartialEq, Clone)]
struct ConnectionSetupSuccess {
    protocol_major_version: u16,
    protocol_minor_version: u16,
    release_number: u32,
    resource_id_base: u32,
    resource_id_mask: u32,
    motion_buffer_size: u32,
    maximum_request_length: u16,
    image_byte_order: ImageByteOrder,
    bitmap_format_bit_order: BitmapFormatBitOrder,
    bitmap_format_scanline_unit: u8,
    bitmap_format_scanline_pad: u8,
    min_keycode: u8,
    max_keycode: u8,
    vendor: String,
    pixmap_formats: Vec<Format>,
    roots: Vec<Screen>,
}