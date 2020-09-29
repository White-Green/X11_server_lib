mod read_specified_length {
    use crate::read_util::{Encoding, read_specified_length};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};

    #[test]
    fn read_specified_length_test() -> Result<(), ()> {
        let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut buffer = [0; 16];
        let len = read_specified_length(&mut &input[..], &mut buffer[..], 10).map_err(|_| ())?;
        assert_eq!(len, 10);
        assert_eq!(buffer, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0]);
        Ok(())
    }

    #[test]
    fn read_specified_length_test_1() -> Result<(), ()> {
        let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut buffer = [0; 10];
        let len = read_specified_length(&mut &input[..], &mut buffer, 12).map_err(|_| ())?;
        assert_eq!(len, 10);
        Ok(())
    }
}

mod collect {
    use crate::read_util::{ByteOrder, Encoding};
    use crate::read_util::ByteOrder::{LSBFirst, MSBFirst};

    #[test]
    fn collect_u8_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<u8>(&data[0..1]), 1);
        assert_eq!(LSBFirst.decode::<u8>(&data[1..2]), 2);
        MSBFirst.encode::<(u8)>(1, &mut output[0..1]);
        LSBFirst.encode::<(u8)>(2, &mut output[1..2]);
        assert_eq!(&output[0..2], &data[0..2]);
    }

    #[test]
    fn collect_u16_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<u16>(&data[0..2]), 1 << 8 | 2);
        assert_eq!(LSBFirst.decode::<u16>(&data[2..4]), 4 << 8 | 3);
        MSBFirst.encode::<(u16)>(1 << 8 | 2, &mut output[0..2]);
        LSBFirst.encode::<(u16)>(4 << 8 | 3, &mut output[2..4]);
        assert_eq!(&output[0..4], &data[0..4]);
    }

    #[test]
    fn collect_u32_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<u32>(&data[0..4]), 1 << 24 | 2 << 16 | 3 << 8 | 4);
        assert_eq!(LSBFirst.decode::<u32>(&data[4..8]), 8 << 24 | 7 << 16 | 6 << 8 | 5);
        MSBFirst.encode::<(u32)>(1 << 24 | 2 << 16 | 3 << 8 | 4, &mut output[0..4]);
        LSBFirst.encode::<(u32)>(8 << 24 | 7 << 16 | 6 << 8 | 5, &mut output[4..8]);
        assert_eq!(&output[0..8], &data[0..8]);
    }

    #[test]
    fn collect_i8_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<i8>(&data[0..1]), 1);
        assert_eq!(LSBFirst.decode::<i8>(&data[1..2]), 2);
        MSBFirst.encode::<(i8)>(1, &mut output[0..1]);
        LSBFirst.encode::<(i8)>(2, &mut output[1..2]);
        assert_eq!(&output[0..2], &data[0..2]);
    }

    #[test]
    fn collect_i16_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<i16>(&data[0..2]), 1 << 8 | 2);
        assert_eq!(LSBFirst.decode::<i16>(&data[2..4]), 4 << 8 | 3);
        MSBFirst.encode::<(i16)>(1 << 8 | 2, &mut output[0..2]);
        LSBFirst.encode::<(i16)>(4 << 8 | 3, &mut output[2..4]);
        assert_eq!(&output[0..4], &data[0..4]);
    }

    #[test]
    fn collect_i32_test() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut output = [0; 16];
        assert_eq!(MSBFirst.decode::<i32>(&data[0..4]), 1 << 24 | 2 << 16 | 3 << 8 | 4);
        assert_eq!(LSBFirst.decode::<i32>(&data[4..8]), 8 << 24 | 7 << 16 | 6 << 8 | 5);
        MSBFirst.encode::<(i32)>(1 << 24 | 2 << 16 | 3 << 8 | 4, &mut output[0..4]);
        LSBFirst.encode::<(i32)>(8 << 24 | 7 << 16 | 6 << 8 | 5, &mut output[4..8]);
        assert_eq!(&output[0..8], &data[0..8]);
    }
}