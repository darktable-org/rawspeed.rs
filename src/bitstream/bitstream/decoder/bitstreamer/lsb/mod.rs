use rawspeed_bitstream_bitstreams::bitstreams::{BitOrder, BitOrderLSB};

use crate::bitstreamer::{
    BitStreamByteSequenceDefaultReader, BitStreamerBase, BitStreamerTraits,
    BitStreamerUseDefaultCacheFillImpl,
};

impl BitStreamerTraits for BitOrderLSB {
    const TAG: BitOrder = BitOrder::LSB;
    type MaxProcessByteArray = [u8; 4];
}

type T = BitOrderLSB;

impl BitStreamerUseDefaultCacheFillImpl for T {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerLSB<'a, R = BitStreamByteSequenceDefaultReader<'a, T>> =
    BitStreamerBase<'a, T, R>;

#[cfg(test)]
mod tests;
