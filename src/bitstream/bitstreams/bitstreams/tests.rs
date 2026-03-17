#[macro_export]
macro_rules! test_bitstream_packing_size_predictions {
    ($BitOrder:ty) => {
        #[test]
        fn bitstream_packing_eltcount_prediction() {
            use $crate::bitstreams::{
                MaximalPackedElementCount, predict_bitstream_bytelen,
            };
            for item_packed_bitlen in 1..=32 {
                for num_items in 0..=(u8::MAX.into()) {
                    let predicted_bytelen = predict_bitstream_bytelen::<
                        $BitOrder,
                    >(
                        num_items, item_packed_bitlen
                    )
                    .try_into()
                    .unwrap();
                    let parsed = MaximalPackedElementCount::new::<$BitOrder>(
                        predicted_bytelen,
                        item_packed_bitlen,
                    );
                    assert_eq!(parsed.bytelen, predicted_bytelen);
                    assert!(parsed.item_count >= num_items.try_into().unwrap());
                }
            }
        }
        #[test]
        fn bitstream_packing_bytelen_prediction() {
            use $crate::bitstreams::{
                MaximalPackedElementCount, predict_bitstream_bytelen,
            };
            for item_packed_bitlen in 1..=32 {
                for bytelen in 0..=(u8::MAX.into()) {
                    let parsed = MaximalPackedElementCount::new::<$BitOrder>(
                        bytelen,
                        item_packed_bitlen,
                    );
                    let reencoded_bytelen =
                        predict_bitstream_bytelen::<$BitOrder>(
                            parsed.item_count.try_into().unwrap(),
                            item_packed_bitlen,
                        )
                        .try_into()
                        .unwrap();
                    assert!(parsed.bytelen >= reencoded_bytelen);
                }
            }
        }
    };
}
