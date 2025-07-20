use super::BitStreamerBase;
use super::BitStreamerTraits;
use super::BitStreamerUseDefaultCacheFillImpl;

use rawspeed_bitstream_bitstreams::bitstreams::BitOrder;
use rawspeed_bitstream_bitstreams::bitstreams::BitOrderLSB;

impl BitStreamerTraits for BitOrderLSB {
    const TAG: BitOrder = BitOrder::LSB;
    const MAX_PROCESS_BYTES: usize = 4;
    type MaxProcessByteArray = [u8; 4];
}

impl BitStreamerUseDefaultCacheFillImpl for BitOrderLSB {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitStreamerLSB<'a> = BitStreamerBase<'a, BitOrderLSB>;

#[cfg(test)]
mod tests;
