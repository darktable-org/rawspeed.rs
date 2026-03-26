use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;

use crate::bitstreamposition::BitstreamPosition;

type BitOrder = BitOrderMSB16;

#[test]
fn test() {
    let pos = BitstreamPosition::<BitOrder>::new(6, 15);
    assert_eq!(pos.mcu_index(), 6);
    assert_eq!(pos.bit_index(), 15);
}
