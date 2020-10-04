#![allow(unused_imports)]
#![deny(dead_code)]

mod request {
    use std::io::{BufReader, BufWriter};

    use crate::read_util::{ByteOrder, Readable, Writable};
    use crate::request::query_extension::QueryExtensionRequest;

    #[test]
    fn read_test() {
        let input = [0, 0, 4, 0, 5, 0, 0, b't', b'e', b's', b't', b'A', 0, 0, 0];
        let value = QueryExtensionRequest::read(&mut BufReader::new(&input[..]), &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, QueryExtensionRequest {
            name: String::from("testA"),
        });
    }

    #[test]
    fn write_test() {
        let value = QueryExtensionRequest {
            name: String::from("testA"),
        };
        let mut buffer = [0; 16];
        QueryExtensionRequest::write(&mut BufWriter::new(&mut buffer[..]), value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(buffer, [98, 0, 0, 4, 0, 5, 0, 0, b't', b'e', b's', b't', b'A', 0, 0, 0]);
    }
}

mod response {
    use std::io::{BufReader, BufWriter};

    use crate::read_util::{ByteOrder, Readable, Writable};
    use crate::request::query_extension::QueryExtensionResponse;

    #[test]
    fn read_test() {
        let input = [1, 0, 0, 1, 0, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let value = QueryExtensionResponse::read(&mut BufReader::new(&input[..]), &ByteOrder::MSBFirst).unwrap();
        assert_eq!(value, QueryExtensionResponse {
            sequence_number: 1,
            present: true,
            major_opcode: 2,
            first_event: 3,
            first_error: 4,
        });
    }

    #[test]
    fn write_test() {
        let value = QueryExtensionResponse {
            sequence_number: 1,
            present: true,
            major_opcode: 2,
            first_event: 3,
            first_error: 4,
        };
        let mut buffer = [0; 32];
        QueryExtensionResponse::write(&mut BufWriter::new(&mut buffer[..]), value, &ByteOrder::MSBFirst).unwrap();
        assert_eq!(buffer, [1, 0, 0, 1, 0, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}