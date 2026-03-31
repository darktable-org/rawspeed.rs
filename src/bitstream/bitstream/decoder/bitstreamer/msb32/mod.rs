use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB32;

use crate::bitstreamer::{
    BitStreamByteSequenceDefaultReader, BitStreamerBase, BitStreamerTraits,
    BitStreamerUseDefaultCacheFillImpl,
};

impl<ByteArray> BitStreamerTraits<ByteArray> for BitOrderMSB32 {
    type ByteArray = ByteArray;
}

type T = BitOrderMSB32;

impl BitStreamerUseDefaultCacheFillImpl for T {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerMSB32<'a, R = BitStreamByteSequenceDefaultReader<'a, T>> =
    BitStreamerBase<'a, T, R>;

#[cfg(test)]
mod tests;
