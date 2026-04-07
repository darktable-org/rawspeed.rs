use rawspeed_bitstream_bitstreams::bitstreams::BitOrderLSB;

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl<ByteArray> BitStreamerTraits<ByteArray> for BitOrderLSB {
    type ByteArray = ByteArray;
}

type T = BitOrderLSB;

impl BitStreamerUseDefaultCacheFillImpl for T {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerLSB<'a> = BitStreamerBase<'a, T>;

#[cfg(test)]
mod tests;
