#[macro_export]
macro_rules! test_lsb {
    ($Impl:ty) => {
        #[test]
        fn read_in_1byte_chunks() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            for i in 1_u8..=255 {
                assert_eq!(reader.get_pos(), usize::from(i) - 1);
                assert_eq!(reader.peek_input(), Ok([i]));
                reader.mark_num_bytes_as_consumed(1);
            }
        }

        #[test]
        fn read_in_2byte_chunks() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            for i in 0_u8..=126 {
                assert_eq!(reader.get_pos(), 2 * usize::from(i));
                assert_eq!(reader.peek_input(), Ok([1 + 2 * i, 2 + 2 * i]));
                reader.mark_num_bytes_as_consumed(2);
            }
        }

        #[test]
        fn read_in_2byte_chunks_with_1byte_step() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            for i in 0_u8..=253 {
                assert_eq!(reader.get_pos(), usize::from(i));
                assert_eq!(reader.peek_input(), Ok([1 + i, 2 + i]));
                reader.mark_num_bytes_as_consumed(1);
            }
        }

        #[test]
        fn get_remaining_size_test() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            assert_eq!(reader.get_remaining_size(), 255);
            assert_eq!(reader.peek_input(), Ok([1]));
            assert_eq!(reader.get_remaining_size(), 255);
            reader.mark_num_bytes_as_consumed(1);
            assert_eq!(reader.get_remaining_size(), 254);
            assert_eq!(reader.peek_input(), Ok([2]));
            assert_eq!(reader.get_remaining_size(), 254);
        }

        #[test]
        fn partial_ov_handling_test() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            assert_eq!(reader.get_pos(), 0);
            assert_eq!(reader.get_remaining_size(), 255);
            reader.mark_num_bytes_as_consumed(255 - 4 - 2);

            assert_eq!(reader.get_pos(), 255 - 4 - 2);
            assert_eq!(reader.get_remaining_size(), 4 + 2);
            assert_eq!(
                reader.peek_input(),
                Ok([255 - 5, 255 - 4, 255 - 3, 255 - 2])
            );
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255 - 2);
            assert_eq!(reader.get_remaining_size(), 2);
            assert_eq!(reader.peek_input(), Ok([255 - 1, 255, 0, 0]));
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255 + 2);
            // assert_eq!(reader.get_remaining_size(), ???); // Ooops, overflow...
            assert_eq!(reader.peek_input(), Ok([0, 0, 0, 0]));
            reader.mark_num_bytes_as_consumed(4);
        }

        #[test]
        fn ov_handling_test() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            assert_eq!(reader.get_pos(), 0);
            assert_eq!(reader.get_remaining_size(), 255);
            reader.mark_num_bytes_as_consumed(255 - 4);

            assert_eq!(reader.get_pos(), 255 - 4);
            assert_eq!(reader.get_remaining_size(), 4);
            assert_eq!(
                reader.peek_input(),
                Ok([255 - 3, 255 - 2, 255 - 1, 255])
            );
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255);
            assert_eq!(reader.get_remaining_size(), 0);
            assert_eq!(reader.peek_input(), Ok([0, 0, 0, 0]));
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255 + 4);
            // assert_eq!(reader.get_remaining_size(), ???); // Ooops, overflow...
            assert_eq!(reader.peek_input(), Ok([0, 0, 0, 0]));
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255 + 2 * 4);
            // assert_eq!(reader.get_remaining_size(), ???); // Ooops, overflow...
            assert_eq!(reader.peek_input(), Ok([0, 0, 0, 0]));
            reader.mark_num_bytes_as_consumed(4);

            assert_eq!(reader.get_pos(), 255 + 3 * 4);
            // assert_eq!(reader.get_remaining_size(), ???); // Ooops, overflow...
            reader.peek_input().unwrap_err();
        }

        #[test]
        fn rewind() {
            let input: [u8; 255] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let mut reader = <$Impl>::new(input.as_slice().try_into().unwrap());
            for prerewind_pos in 0..=255 {
                reader.mark_num_bytes_as_consumed(prerewind_pos);
                reader = reader.rewind();
                for i in 1_u8..=255 {
                    assert_eq!(reader.get_pos(), usize::from(i) - 1);
                    assert_eq!(reader.peek_input(), Ok([i]));
                    reader.mark_num_bytes_as_consumed(1);
                }
            }
        }
    };
}
