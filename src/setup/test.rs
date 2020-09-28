use crate::read_util::ByteOrder::{MSBFirst, LSBFirst};
use crate::setup::{ConnectionSetupInformation, read_setup};

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