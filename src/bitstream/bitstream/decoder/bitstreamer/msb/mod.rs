use rawspeed_bitstream_bitstreams::bitstreams::{BitOrder, BitOrderMSB};

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl BitStreamerTraits for BitOrderMSB {
    const TAG: BitOrder = BitOrder::MSB;
    const MAX_PROCESS_BYTES: usize = 4;
    type MaxProcessByteArray = [u8; 4];
}

impl BitStreamerUseDefaultCacheFillImpl for BitOrderMSB {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB<'a> = BitStreamerBase<'a, BitOrderMSB>;

#[cfg(test)]
mod tests;
