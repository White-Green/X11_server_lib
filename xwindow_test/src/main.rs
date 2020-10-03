use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use xwindow::Error;
use xwindow::read_util::WritableWrite;
use xwindow::setup::{BackingStores, BitmapFormatBitOrder, Class, ConnectionSetupFailed, ConnectionSetupResponse, ConnectionSetupSuccess, Depth, Format, ImageByteOrder, read_setup, Screen, VisualType};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    let (mut stream, addr) = listener.accept().unwrap();
    println!("{}", addr);
    let mut buffer = [0; 1024];
    match read_setup(&mut stream, &mut buffer) {
        Ok((order, info)) => {
            println!("{:#?}", order);
            println!("{:#?}", info);
            let failed_data = ConnectionSetupFailed {
                protocol_major_version: 11,
                protocol_minor_version: 0,
                reason: String::from("こんにちは！このディスプレイはまだ使えません！ "),
            };
            let success_data = ConnectionSetupSuccess {
                protocol_major_version: 11,
                protocol_minor_version: 0,
                release_number: 1,
                resource_id_base: 0,
                resource_id_mask: 0x1f_ff_ff_ff,
                motion_buffer_size: u32::MAX,
                maximum_request_length: u16::MAX,
                image_byte_order: ImageByteOrder::LSBFirst,
                bitmap_format_bit_order: BitmapFormatBitOrder::LeastSignificant,
                bitmap_format_scanline_unit: 4,
                bitmap_format_scanline_pad: 0,
                min_keycode: 0,
                max_keycode: 100,
                vendor: "test".to_string(),
                pixmap_formats: vec![],
                roots: vec![Screen {
                    root: 0,
                    default_colormap: 0,
                    white_pixel: 0,
                    black_pixel: 0,
                    current_input_masks: Default::default(),
                    width_in_pixels: 1,
                    height_in_pixels: 1,
                    width_in_millimeters: 1,
                    height_in_millimeters: 1,
                    min_installed_maps: 0,
                    max_installed_maps: 0,
                    root_visual: 0,
                    backing_stores: BackingStores::Never,
                    save_unders: false,
                    root_depth: 8,
                    allowed_depths: vec![
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::TrueColor,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        }],
                }],
            };
            let response = ConnectionSetupResponse::Success(success_data);
            match stream.write_value(response, &order) {
                Ok(()) => {
                    println!("failed OK!");
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            };
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    };
    loop {
        let length = stream.read(&mut buffer).unwrap();
        if length == 0 {
            println!("closed");
            return;
        }
        let mut hex = String::new();
        let mut asc = String::new();
        for c in &buffer[..length] {
            hex.push_str(&format!("{:02x} ", *c));
            match char::try_from(*c as u32) {
                Ok(c) if c.is_ascii() && !c.is_ascii_control() => {
                    asc.push_str(&format!("{}  ", c));
                }
                _ => {
                    asc.push_str(&format!("?  "));
                }
            }
        }
        println!("{}", hex);
        println!("{}", asc);
    }
}