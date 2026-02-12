use rawspeed_bitstream_bitstreams::bitstreams::{BitOrderLSB, BitStreamTraits};

use crate::derive_mcu_bytesize::derive_mcu_bytesize;

#[test]
fn mcu_size_test() {
    type T = BitOrderLSB;
    let mcu_size = derive_mcu_bytesize::<T>();
    assert_eq!(
        mcu_size,
        size_of::<<T as BitStreamTraits>::MCUByteArrayType>()
    );
}
