use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB32;
use rawspeed_bitstream_bitstreams::bitstreams::BitStreamTraits;

use crate::derive_mcu_bytesize::derive_mcu_bytesize;

#[test]
fn mcu_size_test() {
    type T = BitOrderMSB32;
    let mcu_size = derive_mcu_bytesize::<T>();
    assert_eq!(
        mcu_size,
        size_of::<<T as BitStreamTraits>::MCUByteArrayType>()
    );
}
