use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB;

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl<ByteArray> BitStreamerTraits<ByteArray> for BitOrderMSB {
    type ByteArray = ByteArray;
}

type T = BitOrderMSB;

impl BitStreamerUseDefaultCacheFillImpl for T {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB<'a> = BitStreamerBase<'a, T>;

#[cfg(test)]
mod tests;
