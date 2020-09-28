mod read_specified_length {
    use crate::read_util::{Collect, read_specified_length};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};

    #[test]
    fn read_specified_length_test() -> Result<(), ()> {
        let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut buffer = [0; 16];
        read_specified_length(&mut &input[..], &mut buffer[..], 10).map_err(|_e| ())?;
        assert_eq!(buffer, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn read_specified_length_test_1() {
        let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut buffer = [0; 10];
        let _ = read_specified_length(&mut &input[..], &mut buffer, 12);
    }
}

mod collect {
    use crate::read_util::{ByteOrder, Collect};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};

    #[test]
    fn collect_u8_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<u8>>::collect(&MSBFirst, &data[0..1]), 1);
        assert_eq!(<ByteOrder as Collect<u8>>::collect(&LSBFirst, &data[1..2]), 2);
    }

    #[test]
    fn collect_u16_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<u16>>::collect(&MSBFirst, &data[0..2]), 1 << 8 | 2);
        assert_eq!(<ByteOrder as Collect<u16>>::collect(&LSBFirst, &data[2..4]), 4 << 8 | 3);
    }

    #[test]
    fn collect_u32_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<u32>>::collect(&MSBFirst, &data[0..4]), 1 << 24 | 2 << 16 | 3 << 8 | 4);
        assert_eq!(<ByteOrder as Collect<u32>>::collect(&LSBFirst, &data[4..8]), 8 << 24 | 7 << 16 | 6 << 8 | 5);
    }

    #[test]
    fn collect_i8_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<i8>>::collect(&MSBFirst, &data[0..1]), 1);
        assert_eq!(<ByteOrder as Collect<i8>>::collect(&LSBFirst, &data[1..2]), 2);
    }

    #[test]
    fn collect_i16_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<i16>>::collect(&MSBFirst, &data[0..2]), 1 << 8 | 2);
        assert_eq!(<ByteOrder as Collect<i16>>::collect(&LSBFirst, &data[2..4]), 4 << 8 | 3);
    }

    #[test]
    fn collect_i32_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        assert_eq!(<ByteOrder as Collect<i32>>::collect(&MSBFirst, &data[0..4]), 1 << 24 | 2 << 16 | 3 << 8 | 4);
        assert_eq!(<ByteOrder as Collect<i32>>::collect(&LSBFirst, &data[4..8]), 8 << 24 | 7 << 16 | 6 << 8 | 5);
    }
}