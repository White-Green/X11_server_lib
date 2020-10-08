#![allow(unused_imports)]
#![deny(dead_code)]

mod request {
    use std::io::{BufReader, BufWriter};

    use crate::read_util::{ByteOrder, Readable, Writable};
    use crate::request::get_property::GetPropertyRequest;

    #[test]
    fn read_test() {
        let input = [1, 0, 6, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6];
        let value = GetPropertyRequest::read(&mut BufReader::new(&input[..]), &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, GetPropertyRequest {
            delete: true,
            window: 2,
            property: 3,
            type_: Some(4),
            long_offset: 5,
            long_length: 6,
        });
    }

    #[test]
    fn write_test() {
        let value = GetPropertyRequest {
            delete: true,
            window: 2,
            property: 3,
            type_: Some(4),
            long_offset: 5,
            long_length: 6,
        };
        let mut buffer = [0; 24];
        GetPropertyRequest::write(&mut BufWriter::new(&mut buffer[..]), value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(buffer, [20, 1, 0, 6, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6]);
    }
}