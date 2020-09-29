use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};
use crate::setup::{ConnectionSetupInformation, read_setup, write_setup};

#[test]
fn read_setup_test() {
    let input = vec![0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(
        read_setup(&mut &input[..], &mut [0; 1024]).map_err(|_| ()),
        Ok((
            MSBFirst,
            ConnectionSetupInformation {
                protocol_major_version: 11,
                protocol_minor_version: 0,
                authorization_protocol_name: String::new(),
                authorization_protocol_data: String::new(),
            })));

    let input = vec![0o154, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(
        read_setup(&mut &input[..], &mut [0; 1024]).map_err(|_| ()),
        Ok((
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
fn write_setup_test() -> Result<(), ()> {
    let input = ConnectionSetupInformation {
        protocol_major_version: 11,
        protocol_minor_version: 0,
        authorization_protocol_name: String::new(),
        authorization_protocol_data: String::new(),
    };
    let mut stream = [0; 1024];
    let mut buffer = [0; 1024];
    write_setup(&mut &mut stream[..], &mut buffer[..], &MSBFirst, input).map_err(|_|())?;
    assert_eq!(&stream[..12], &[0o102, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0]);

    let input = ConnectionSetupInformation {
        protocol_major_version: 11,
        protocol_minor_version: 0,
        authorization_protocol_name: String::new(),
        authorization_protocol_data: String::new(),
    };
    write_setup(&mut &mut stream[..], &mut buffer[..], &LSBFirst, input).map_err(|_|())?;
    assert_eq!(&stream[..12], &[0o154, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}
