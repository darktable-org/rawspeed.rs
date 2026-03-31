#[macro_export]
macro_rules! test_bitstreamer_rewind {
    ($BitOrder:ty, $BitStreamer:ident) => {
        #[test]
        fn rewind() -> Result<(), Box<dyn core::error::Error>>
        where
            $BitOrder: BitStreamTraits,
            for<'a> $BitStreamer<'a>: $crate::bitstreamer::BitStream,
        {
            const MAX_BITS_PER_OP: u32 = 32;
            const INPUT_LEN: usize = 32;
            assert!(8 * INPUT_LEN >= usize::try_from(2 * u64::BITS).unwrap());
            assert!(
                INPUT_LEN
                    >= 2 * size_of::<
                        <$BitOrder as BitStreamTraits>::MCUByteArrayType,
                    >()
            );

            let input: [u8; INPUT_LEN] =
                core::array::from_fn(|i| u8::try_from(1 + i).unwrap());
            let input_bitcount = u32::try_from(8 * input.len()).unwrap();
            let mut base_bs: $BitStreamer<'_> =
                $BitStreamer::new(input.as_slice().try_into().unwrap());
            for restart_bit_pos in (0..=input_bitcount).step_by(8) {
                if restart_bit_pos > 0 {
                    base_bs.fill(8)?;
                    base_bs.skip_bits_no_fill(8);
                }
                for fill_level in (0..=MAX_BITS_PER_OP).step_by(8) {
                    let mut bs = base_bs;
                    if fill_level != 0 {
                        bs.fill(fill_level)?;
                    }
                    let position = bs.get_bitstream_position();
                    let mut bs_orig = bs;
                    let mut bs_restarted: $BitStreamer<'_> =
                        $BitStreamer::new_with_position(
                            input.as_slice().try_into().unwrap(),
                            position,
                        )?;
                    for _ in (restart_bit_pos..input_bitcount)
                        .step_by(MAX_BITS_PER_OP.try_into()?)
                    {
                        bs_orig.fill(MAX_BITS_PER_OP)?;
                        bs_restarted.fill(MAX_BITS_PER_OP)?;
                        assert_eq!(
                            bs_orig.peek_bits_no_fill(MAX_BITS_PER_OP),
                            bs_restarted.peek_bits_no_fill(MAX_BITS_PER_OP)
                        );
                        bs_orig.skip_bits_no_fill(MAX_BITS_PER_OP);
                        bs_restarted.skip_bits_no_fill(MAX_BITS_PER_OP);
                    }
                }
            }
            Ok(())
        }
    };
}
