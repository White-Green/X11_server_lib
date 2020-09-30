#![allow(unused_imports)]


mod setup {
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
    use crate::setup::{ConnectionSetupInformation, read_setup, write_setup};

    #[test]
    fn read_setup_test() -> Result<(), ()> {
        let input = vec![0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            read_setup(&mut &input[..], &mut [0; 1024]).map_err(|_| ())?,
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
            read_setup(&mut &input[..], &mut [0; 1024]).map_err(|_| ())?,
            ((
                LSBFirst,
                ConnectionSetupInformation {
                    protocol_major_version: 11,
                    protocol_minor_version: 0,
                    authorization_protocol_name: String::new(),
                    authorization_protocol_data: String::new(),
                })));

        //TODO: authテストケース追加
        Ok(())
    }

    #[test]
    fn write_setup_test() -> Result<(), ()> {
        let input = ConnectionSetupInformation {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: String::new(),
            authorization_protocol_data: String::new(),
        };
        let mut stream = [0; 1024];
        let mut buffer = [0; 1024];
        write_setup(&mut &mut stream[..], &mut buffer[..], &MSBFirst, input).map_err(|_| ())?;
        assert_eq!(&stream[..12], &[0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0]);

        let input = ConnectionSetupInformation {
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: String::new(),
            authorization_protocol_data: String::new(),
        };
        write_setup(&mut &mut stream[..], &mut buffer[..], &LSBFirst, input).map_err(|_| ())?;
        assert_eq!(&stream[..12], &[0o154, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        Ok(())
    }
}

mod setup_failed {
    use crate::read_util::{Readable, Writable};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
    use crate::setup::ConnectionSetupFailed;

    #[test]
    fn read_test() -> Result<(), ()> {
        let input = [5u8, 1, 2, 3, 4, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0];
        let data = ConnectionSetupFailed::read(&mut &input[..], &MSBFirst).map_err(|_| ())?;
        assert_eq!(data,
                   ConnectionSetupFailed {
                       protocol_major_version: 1 << 8 | 2,
                       protocol_minor_version: 3 << 8 | 4,
                       reason: String::from("abcde"),
                   });

        let input = [7u8, 1, 2, 3, 4, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0];
        let data = ConnectionSetupFailed::read(&mut &input[..], &LSBFirst).map_err(|_| ())?;
        assert_eq!(data,
                   ConnectionSetupFailed {
                       protocol_major_version: 2 << 8 | 1,
                       protocol_minor_version: 4 << 8 | 3,
                       reason: String::from("abcdefg"),
                   });
        Ok(())
    }

    #[test]
    fn write_test() -> Result<(), ()> {
        let mut buffer = [0; 20];
        let data = ConnectionSetupFailed {
            protocol_major_version: 1 << 8 | 2,
            protocol_minor_version: 3 << 8 | 4,
            reason: String::from("abcde"),
        };
        ConnectionSetupFailed::write(&mut &mut buffer[..], data, &MSBFirst).map_err(|_| ())?;
        assert_eq!(&buffer[..13], &[0, 5u8, 1, 2, 3, 4, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0][..13]);

        let data = ConnectionSetupFailed {
            protocol_major_version: 2 << 8 | 1,
            protocol_minor_version: 4 << 8 | 3,
            reason: String::from("abcdefg"),
        };
        ConnectionSetupFailed::write(&mut &mut buffer[..], data, &LSBFirst).map_err(|_| ())?;
        assert_eq!(&buffer[..15], &[0, 7u8, 1, 2, 3, 4, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0][..15]);
        Ok(())
    }
}

mod setup_authenticate {
    use crate::read_util::ByteOrder::{MSBFirst, LSBFirst};
    use crate::setup::ConnectionSetupAuthenticate;
    use crate::read_util::{Readable, Writable};

    #[test]
    fn read_test() -> Result<(), ()> {
        let input = [0, 0, 0, 0, 0, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0];
        let data = ConnectionSetupAuthenticate::read(&mut &input[..], &MSBFirst).map_err(|_| ())?;
        assert_eq!(data,
                   ConnectionSetupAuthenticate {
                       reason: String::from("abcde"),
                   });

        let input = [0, 0, 0, 0, 0, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0];
        let data = ConnectionSetupAuthenticate::read(&mut &input[..], &LSBFirst).map_err(|_| ())?;
        assert_eq!(data,
                   ConnectionSetupAuthenticate {
                       reason: String::from("abcdefg"),
                   });
        Ok(())
    }

    #[test]
    fn write_test() -> Result<(), ()> {
        let mut buffer = [0; 20];
        let data = ConnectionSetupAuthenticate {
            reason: String::from("abcde"),
        };
        ConnectionSetupAuthenticate::write(&mut &mut buffer[..], data, &MSBFirst).map_err(|_| ())?;
        assert_eq!(&buffer[..13], &[2, 0, 0, 0, 0, 0, 0, 2, b'a', b'b', b'c', b'd', b'e', 0, 0, 0][..13]);

        let data = ConnectionSetupAuthenticate {
            reason: String::from("abcdefg"),
        };
        ConnectionSetupAuthenticate::write(&mut &mut buffer[..], data, &LSBFirst).map_err(|_| ())?;
        assert_eq!(&buffer[..15], &[2, 0, 0, 0, 0, 0, 2, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0][..15]);
        Ok(())
    }
}