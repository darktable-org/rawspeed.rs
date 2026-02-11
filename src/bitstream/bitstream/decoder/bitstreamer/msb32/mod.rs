use rawspeed_bitstream_bitstreams::bitstreams::{BitOrder, BitOrderMSB32};

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl BitStreamerTraits for BitOrderMSB32 {
    const TAG: BitOrder = BitOrder::MSB32;
    const MAX_PROCESS_BYTES: usize = 4;
    type MaxProcessByteArray = [u8; 4];
}

impl BitStreamerUseDefaultCacheFillImpl for BitOrderMSB32 {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB32<'a> = BitStreamerBase<'a, BitOrderMSB32>;

#[cfg(test)]
mod tests;
