use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;

use crate::bitstreamer::{
    BitStreamerBase, BitStreamerTraits, BitStreamerUseDefaultCacheFillImpl,
};

impl<ByteArray> BitStreamerTraits<ByteArray> for BitOrderMSB16 {
    type ByteArray = ByteArray;
}

type T = BitOrderMSB16;

impl BitStreamerUseDefaultCacheFillImpl for T {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB16<'a> = BitStreamerBase<'a, T>;

#[cfg(test)]
mod tests;
