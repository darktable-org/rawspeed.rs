use rawspeed_bitstream_bitstreams::bitstreams::{BitOrder, BitOrderMSB16};

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl BitStreamerTraits for BitOrderMSB16 {
    const TAG: BitOrder = BitOrder::MSB16;
    const MAX_PROCESS_BYTES: usize = 4;
    type MaxProcessByteArray = [u8; 4];
}

impl BitStreamerUseDefaultCacheFillImpl for BitOrderMSB16 {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB16<'a> = BitStreamerBase<'a, BitOrderMSB16>;

#[cfg(test)]
mod tests;
