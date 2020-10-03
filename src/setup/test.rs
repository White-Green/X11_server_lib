#![allow(unused_imports)]
#![deny(dead_code)]

mod setup {
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
    use crate::setup::{ConnectionSetupInformation, read_setup, write_setup};

    #[test]
    fn read_setup_test() {
        let input = vec![0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            read_setup(&mut &input[..], &mut [0; 1024]).unwrap(),
            ((
                MSBFirst,
                ConnectionSetupInformation {
                    protocol_major_version: 11,
                    protocol_minor_version: 0,
                    authorization_protocol_name: String::new(),
                    authorization_protocol_data: String::new(),
                })));

        let input = vec![0o154, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            read_setup(&mut &input[..], &mut [0; 1024]).unwrap(),
            ((
                LSBFirst,
                ConnectionSetupInformation {
                    protocol_major_version: 11,
                    protocol_minor_version: 0,
                    authorization_protocol_name: String::new(),
                    authorization_protocol_data: String::new(),
                })));

        //TODO: authテストケース追加
    }

    #[test]
    fn write_setup_test() {
        let input = ConnectionSetupInformation {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: String::new(),
            authorization_protocol_data: String::new(),
        };
        let mut stream = [0; 1024];
        let mut buffer = [0; 1024];
        write_setup(&mut &mut stream[..], &mut buffer[..], &MSBFirst, input).unwrap();
        assert_eq!(&stream[..12], &[0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0]);

        let input = ConnectionSetupInformation {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: String::new(),
            authorization_protocol_data: String::new(),
        };
        write_setup(&mut &mut stream[..], &mut buffer[..], &LSBFirst, input).unwrap();
        assert_eq!(&stream[..12], &[0o154, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}

mod setup_failed {
    use crate::read_util::{Readable, Writable};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
    use crate::setup::ConnectionSetupFailed;

    #[test]
    fn read_test() {
        let input = [5u8, 1, 2, 3, 4, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0];
        let data = ConnectionSetupFailed::read(&mut &input[..], &MSBFirst).unwrap();
        assert_eq!(data,
                   ConnectionSetupFailed {
                       protocol_major_version: 1 << 8 | 2,
                       protocol_minor_version: 3 << 8 | 4,
                       reason: String::from("abcde"),
                   });

        let input = [7u8, 1, 2, 3, 4, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0];
        let data = ConnectionSetupFailed::read(&mut &input[..], &LSBFirst).unwrap();
        assert_eq!(data,
                   ConnectionSetupFailed {
                       protocol_major_version: 2 << 8 | 1,
                       protocol_minor_version: 4 << 8 | 3,
                       reason: String::from("abcdefg"),
                   });
    }

    #[test]
    fn write_test() {
        let mut buffer = [0; 20];
        let data = ConnectionSetupFailed {
            protocol_major_version: 1 << 8 | 2,
            protocol_minor_version: 3 << 8 | 4,
            reason: String::from("abcde"),
        };
        ConnectionSetupFailed::write(&mut &mut buffer[..], data, &MSBFirst).unwrap();
        assert_eq!(&buffer[..13], &[0, 5u8, 1, 2, 3, 4, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0][..13]);

        let data = ConnectionSetupFailed {
            protocol_major_version: 2 << 8 | 1,
            protocol_minor_version: 4 << 8 | 3,
            reason: String::from("abcdefg"),
        };
        ConnectionSetupFailed::write(&mut &mut buffer[..], data, &LSBFirst).unwrap();
        assert_eq!(&buffer[..15], &[0, 7u8, 1, 2, 3, 4, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0][..15]);
    }
}

mod setup_authenticate {
    use crate::read_util::{Readable, Writable};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
    use crate::setup::ConnectionSetupAuthenticate;

    #[test]
    fn read_test() {
        let input = [0, 0, 0, 0, 0, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0];
        let data = ConnectionSetupAuthenticate::read(&mut &input[..], &MSBFirst).unwrap();
        assert_eq!(data,
                   ConnectionSetupAuthenticate {
                       reason: String::from("abcde"),
                   });

        let input = [0, 0, 0, 0, 0, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0];
        let data = ConnectionSetupAuthenticate::read(&mut &input[..], &LSBFirst).unwrap();
        assert_eq!(data,
                   ConnectionSetupAuthenticate {
                       reason: String::from("abcdefg"),
                   });
    }

    #[test]
    fn write_test() {
        let mut buffer = [0; 20];
        let data = ConnectionSetupAuthenticate {
            reason: String::from("abcde"),
        };
        ConnectionSetupAuthenticate::write(&mut &mut buffer[..], data, &MSBFirst).unwrap();
        assert_eq!(&buffer[..13], &[2, 0, 0, 0, 0, 0, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0][..13]);

        let data = ConnectionSetupAuthenticate {
            reason: String::from("abcdefg"),
        };
        ConnectionSetupAuthenticate::write(&mut &mut buffer[..], data, &LSBFirst).unwrap();
        assert_eq!(&buffer[..15], &[2, 0, 0, 0, 0, 0, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0][..15]);
    }
}

mod setup_success {
    use std::collections::HashSet;

    use crate::read_util::{ByteOrder, Writable};
    use crate::read_util::Readable;
    use crate::setup::{BackingStores, BitmapFormatBitOrder, Class, ConnectionSetupSuccess, Depth, Event, Format, ImageByteOrder, Screen, VisualType};

    #[test]
    fn image_byte_order_read_test() {
        let input = [0];
        let value = ImageByteOrder::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, ImageByteOrder::LSBFirst);
        let input = [1];
        let value = ImageByteOrder::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, ImageByteOrder::MSBFirst);
        let input = [2];
        ImageByteOrder::read(&mut &input[..], &ByteOrder::MSBFirst).map_err(|_| ()).expect_err("expect err");
    }

    #[test]
    fn image_byte_order_write_test() {
        let mut input = [100];
        ImageByteOrder::write(&mut &mut input[..], ImageByteOrder::LSBFirst, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(input, [0]);
        let mut input = [100];
        ImageByteOrder::write(&mut &mut input[..], ImageByteOrder::MSBFirst, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(input, [1]);
    }

    #[test]
    fn bitmap_format_bit_order_read_test() {
        let input = [0];
        let value = BitmapFormatBitOrder::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, BitmapFormatBitOrder::LeastSignificant);
        let input = [1];
        let value = BitmapFormatBitOrder::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, BitmapFormatBitOrder::MostSignificant);
        let input = [2];
        ImageByteOrder::read(&mut &input[..], &ByteOrder::MSBFirst).map_err(|_| ()).expect_err("expect err");
    }

    #[test]
    fn bitmap_format_bit_order_write_test() {
        let mut output = [100];
        BitmapFormatBitOrder::write(&mut &mut output[..], BitmapFormatBitOrder::LeastSignificant, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output, [0]);
        let mut output = [100];
        BitmapFormatBitOrder::write(&mut &mut output[..], BitmapFormatBitOrder::MostSignificant, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output, [1]);
    }

    #[test]
    fn format_read_test() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let value = Format::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Format { depth: 1, bits_per_pixel: 2, scanline_pad: 3 });
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let value = Format::read(&mut &input[..], &ByteOrder::LSBFirst).unwrap();
        assert_eq!(value, Format { depth: 1, bits_per_pixel: 2, scanline_pad: 3 });
        let input = [1, 2, 3, 4, 5, 6, 7];
        Format::read(&mut &input[..], &ByteOrder::MSBFirst).expect_err("err");
    }

    #[test]
    fn format_write_test() {
        let value = Format { depth: 1, bits_per_pixel: 2, scanline_pad: 3 };
        let mut output = [0; 16];
        Format::write(&mut &mut output[..], value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(&output[..3], &[1, 2, 3]);
        let value = Format { depth: 1, bits_per_pixel: 2, scanline_pad: 3 };
        let mut output = [0; 16];
        Format::write(&mut &mut output[..], value, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(&output[..3], &[1, 2, 3]);
    }

    #[test]
    fn backing_stores_read_test() {
        let input = [0];
        let value = BackingStores::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, BackingStores::Never);
        let input = [1];
        let value = BackingStores::read(&mut &input[..], &ByteOrder::LSBFirst).unwrap();
        assert_eq!(value, BackingStores::WhenMapped);
        let input = [2];
        let value = BackingStores::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, BackingStores::Always);
    }

    #[test]
    fn backing_stores_write_test() {
        let mut output = [0; 4];
        BackingStores::write(&mut &mut output[..], BackingStores::Never, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(output[0], 0);
        let mut output = [0; 4];
        BackingStores::write(&mut &mut output[..], BackingStores::WhenMapped, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output[0], 1);
        let mut output = [0; 4];
        BackingStores::write(&mut &mut output[..], BackingStores::Always, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(output[0], 2);
    }

    #[test]
    fn class_read_test() {
        let input = [0];
        let value = Class::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Class::StaticGray);
        let input = [1];
        let value = Class::read(&mut &input[..], &ByteOrder::LSBFirst).unwrap();
        assert_eq!(value, Class::GrayScale);
        let input = [2];
        let value = Class::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Class::StaticColor);
        let input = [3];
        let value = Class::read(&mut &input[..], &ByteOrder::LSBFirst).unwrap();
        assert_eq!(value, Class::PseudoColor);
        let input = [4];
        let value = Class::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Class::TrueColor);
        let input = [5];
        let value = Class::read(&mut &input[..], &ByteOrder::LSBFirst).unwrap();
        assert_eq!(value, Class::DirectColor);
    }

    #[test]
    fn class_write_test() {
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::StaticGray, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(output[0], 0);
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::GrayScale, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output[0], 1);
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::StaticColor, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(output[0], 2);
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::PseudoColor, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output[0], 3);
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::TrueColor, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(output[0], 4);
        let mut output = [0; 4];
        Class::write(&mut &mut output[..], Class::DirectColor, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output[0], 5);
    }

    #[test]
    fn visual_type_read_test() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
        let value = VisualType::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, VisualType {
            visual_id: 1 << 24 | 2 << 16 | 3 << 8 | 4,
            class: Class::DirectColor,
            bits_per_rgb_value: 6,
            colormap_entries: 7 << 8 | 8,
            red_mask: 9 << 24 | 10 << 16 | 11 << 8 | 12,
            green_mask: 13 << 24 | 14 << 16 | 15 << 8 | 16,
            blue_mask: 17 << 24 | 18 << 16 | 19 << 8 | 20,
        });
        let input = [0; 23];
        VisualType::read(&mut &input[..], &ByteOrder::LSBFirst).expect_err("err");
    }

    #[test]
    fn visual_type_write_test() {
        let value = VisualType {
            visual_id: 1 << 24 | 2 << 16 | 3 << 8 | 4,
            class: Class::DirectColor,
            bits_per_rgb_value: 6,
            colormap_entries: 7 << 8 | 8,
            red_mask: 9 << 24 | 10 << 16 | 11 << 8 | 12,
            green_mask: 13 << 24 | 14 << 16 | 15 << 8 | 16,
            blue_mask: 17 << 24 | 18 << 16 | 19 << 8 | 20,
        };
        let mut output = [0; 25];
        VisualType::write(&mut &mut output[..], value, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(&output[..20], &[4, 3, 2, 1, 5, 6, 8, 7, 12, 11, 10, 9, 16, 15, 14, 13, 20, 19, 18, 17]);
    }

    #[test]
    fn depth_read_test() {
        let value1 = VisualType {
            visual_id: 1,
            class: Class::StaticGray,
            bits_per_rgb_value: 2,
            colormap_entries: 3,
            red_mask: 4,
            green_mask: 5,
            blue_mask: 6,
        };
        let value2 = VisualType {
            visual_id: 10,
            class: Class::GrayScale,
            bits_per_rgb_value: 20,
            colormap_entries: 30,
            red_mask: 40,
            green_mask: 50,
            blue_mask: 60,
        };
        let mut input = vec![10u8, 1, 0, 2, 3, 3, 3, 3];
        input.resize(64, 0);
        VisualType::write(&mut &mut input[8..], value1.clone(), &ByteOrder::MSBFirst).unwrap();
        VisualType::write(&mut &mut input[32..], value2.clone(), &ByteOrder::MSBFirst).unwrap();
        let value = Depth::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Depth {
            depth: 10,
            visuals: vec![value1, value2],
        });
    }

    #[test]
    fn depth_write_test() {
        let value1 = VisualType {
            visual_id: 1,
            class: Class::StaticGray,
            bits_per_rgb_value: 2,
            colormap_entries: 3,
            red_mask: 4,
            green_mask: 5,
            blue_mask: 6,
        };
        let value2 = VisualType {
            visual_id: 10,
            class: Class::GrayScale,
            bits_per_rgb_value: 20,
            colormap_entries: 30,
            red_mask: 40,
            green_mask: 50,
            blue_mask: 60,
        };
        let value = Depth {
            depth: 10,
            visuals: vec![value1.clone(), value2.clone()],
        };
        let mut expect = vec![10u8, 1, 2, 0, 3, 3, 3, 3];
        expect.resize(54, 0);
        VisualType::write(&mut &mut expect[8..], value1, &ByteOrder::LSBFirst).unwrap();
        VisualType::write(&mut &mut expect[32..], value2, &ByteOrder::LSBFirst).unwrap();
        let mut output = vec![0u8; 54];
        Depth::write(&mut &mut output[..], value, &ByteOrder::LSBFirst).unwrap();
        assert_eq!(expect[0], output[0]);
        assert_eq!(&expect[8..], &output[8..]);
        assert_eq!(&expect[2..4], &output[2..4]);
    }

    #[test]
    fn event_read_test() {
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
        let input = [0x01, 0xff, 0xff, 0xff];
        let value = HashSet::<Event>::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        let mut expect = HashSet::new();
        for event in &VALUES {
            expect.insert(event.clone());
        }
        assert_eq!(value, expect);
        let input = [0, 0, 0, 2];
        HashSet::<Event>::read(&mut &input[..], &ByteOrder::LSBFirst).expect_err("err");
    }

    #[test]
    fn event_write_test() {
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
        let mut value = HashSet::new();
        for event in &VALUES {
            value.insert(event.clone());
        }
        let mut output = [0; 4];
        HashSet::<Event>::write(&mut &mut output[..], value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(output, [0x01, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn screen_read_test() {
        let input = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0];
        let value = Screen::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, Screen {
            root: 1,
            default_colormap: 2,
            white_pixel: 3,
            black_pixel: 4,
            current_input_masks: Default::default(),
            width_in_pixels: 5,
            height_in_pixels: 6,
            width_in_millimeters: 7,
            height_in_millimeters: 8,
            min_installed_maps: 9,
            max_installed_maps: 10,
            root_visual: 11,
            backing_stores: BackingStores::Always,
            save_unders: false,
            root_depth: 12,
            allowed_depths: vec![
                Depth {
                    depth: 8,
                    visuals: vec![
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        }],
                },
                Depth {
                    depth: 8,
                    visuals: vec![
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        },
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        }],
                }],
        });
        let input = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0];
        Screen::read(&mut &input[..], &ByteOrder::MSBFirst).expect_err("err");
    }

    #[test]
    fn screen_write_test() {
        let mut buffer = [0; 128];
        let value = Screen {
            root: 1,
            default_colormap: 2,
            white_pixel: 3,
            black_pixel: 4,
            current_input_masks: Default::default(),
            width_in_pixels: 5,
            height_in_pixels: 6,
            width_in_millimeters: 7,
            height_in_millimeters: 8,
            min_installed_maps: 9,
            max_installed_maps: 10,
            root_visual: 11,
            backing_stores: BackingStores::Always,
            save_unders: false,
            root_depth: 12,
            allowed_depths: vec![
                Depth {
                    depth: 8,
                    visuals: vec![
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        }],
                },
                Depth {
                    depth: 8,
                    visuals: vec![
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        },
                        VisualType {
                            visual_id: 0,
                            class: Class::StaticGray,
                            bits_per_rgb_value: 8,
                            colormap_entries: 0,
                            red_mask: 0xff_00_00,
                            green_mask: 0x00_ff_00,
                            blue_mask: 0x00_00_ff,
                        }],
                }],
        };
        Screen::write(&mut &mut buffer[..], value, &ByteOrder::MSBFirst).unwrap();
        let expect = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0];
        assert_eq!(buffer, expect);
    }

    #[test]
    fn read_test() {
        let input = [0, 0, 11, 0, 0, 0, 45, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 5, 0, 5, 2, 2, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, b't', b'e', b's', b't', b'A', 0, 0, 0,
            8, 24, 3, 0, 0, 0, 0, 0,
            8, 24, 5, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0
        ];
        let value = ConnectionSetupSuccess::read(&mut &input[..], &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, ConnectionSetupSuccess {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            release_number: 1,
            resource_id_base: 2,
            resource_id_mask: 3,
            motion_buffer_size: 4,
            maximum_request_length: 5,
            image_byte_order: ImageByteOrder::LSBFirst,
            bitmap_format_bit_order: BitmapFormatBitOrder::LeastSignificant,
            bitmap_format_scanline_unit: 6,
            bitmap_format_scanline_pad: 7,
            min_keycode: 8,
            max_keycode: 9,
            vendor: "testA".to_string(),
            pixmap_formats: vec![
                Format {
                    depth: 8,
                    bits_per_pixel: 24,
                    scanline_pad: 3,
                },
                Format {
                    depth: 8,
                    bits_per_pixel: 24,
                    scanline_pad: 5,
                }],
            roots: vec![
                Screen {
                    root: 1,
                    default_colormap: 2,
                    white_pixel: 3,
                    black_pixel: 4,
                    current_input_masks: Default::default(),
                    width_in_pixels: 5,
                    height_in_pixels: 6,
                    width_in_millimeters: 7,
                    height_in_millimeters: 8,
                    min_installed_maps: 9,
                    max_installed_maps: 10,
                    root_visual: 11,
                    backing_stores: BackingStores::Always,
                    save_unders: false,
                    root_depth: 12,
                    allowed_depths: vec![
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        },
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                },
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        }],
                },
                Screen {
                    root: 1,
                    default_colormap: 2,
                    white_pixel: 3,
                    black_pixel: 4,
                    current_input_masks: Default::default(),
                    width_in_pixels: 5,
                    height_in_pixels: 6,
                    width_in_millimeters: 7,
                    height_in_millimeters: 8,
                    min_installed_maps: 9,
                    max_installed_maps: 10,
                    root_visual: 11,
                    backing_stores: BackingStores::Always,
                    save_unders: false,
                    root_depth: 12,
                    allowed_depths: vec![
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        },
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                },
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        }],
                }],
        });
        let input = [0, 0, 11, 0, 0, 0, 45, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 5, 0, 5, 2, 2, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, b't', b'e', b's', b't', b'A', 0, 0, 0,
            8, 24, 3, 0, 0, 0, 0, 0,
            8, 24, 5, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0
        ];
        ConnectionSetupSuccess::read(&mut &input[..], &ByteOrder::MSBFirst).expect_err("err");
    }

    #[test]
    fn write_test() {
        let value = ConnectionSetupSuccess {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            release_number: 1,
            resource_id_base: 2,
            resource_id_mask: 3,
            motion_buffer_size: 4,
            maximum_request_length: 5,
            image_byte_order: ImageByteOrder::LSBFirst,
            bitmap_format_bit_order: BitmapFormatBitOrder::LeastSignificant,
            bitmap_format_scanline_unit: 6,
            bitmap_format_scanline_pad: 7,
            min_keycode: 8,
            max_keycode: 9,
            vendor: "testA".to_string(),
            pixmap_formats: vec![
                Format {
                    depth: 8,
                    bits_per_pixel: 24,
                    scanline_pad: 3,
                },
                Format {
                    depth: 8,
                    bits_per_pixel: 24,
                    scanline_pad: 5,
                }],
            roots: vec![
                Screen {
                    root: 1,
                    default_colormap: 2,
                    white_pixel: 3,
                    black_pixel: 4,
                    current_input_masks: Default::default(),
                    width_in_pixels: 5,
                    height_in_pixels: 6,
                    width_in_millimeters: 7,
                    height_in_millimeters: 8,
                    min_installed_maps: 9,
                    max_installed_maps: 10,
                    root_visual: 11,
                    backing_stores: BackingStores::Always,
                    save_unders: false,
                    root_depth: 12,
                    allowed_depths: vec![
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        },
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                },
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        }],
                },
                Screen {
                    root: 1,
                    default_colormap: 2,
                    white_pixel: 3,
                    black_pixel: 4,
                    current_input_masks: Default::default(),
                    width_in_pixels: 5,
                    height_in_pixels: 6,
                    width_in_millimeters: 7,
                    height_in_millimeters: 8,
                    min_installed_maps: 9,
                    max_installed_maps: 10,
                    root_visual: 11,
                    backing_stores: BackingStores::Always,
                    save_unders: false,
                    root_depth: 12,
                    allowed_depths: vec![
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        },
                        Depth {
                            depth: 8,
                            visuals: vec![
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                },
                                VisualType {
                                    visual_id: 0,
                                    class: Class::StaticGray,
                                    bits_per_rgb_value: 8,
                                    colormap_entries: 0,
                                    red_mask: 0xff_00_00,
                                    green_mask: 0x00_ff_00,
                                    blue_mask: 0x00_00_ff,
                                }],
                        }],
                }],
        };
        let mut buffer = [0; 320];
        ConnectionSetupSuccess::write(&mut &mut buffer[..], value, &ByteOrder::MSBFirst).unwrap();
        let expect = [1, 0, 0, 11, 0, 0, 0, 78, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 5, 0, 5, 2, 2, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, b't', b'e', b's', b't', b'A', 0, 0, 0,
            8, 24, 3, 0, 0, 0, 0, 0,
            8, 24, 5, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 0, 0, 11, 2, 0, 12, 2,
            8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0,
            8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0xff, 0, 0, 0, 0
        ];
        assert_eq!(buffer, expect);
    }
}