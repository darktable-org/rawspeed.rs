use super::BitStreamerBase;
use super::BitStreamerTraits;
use super::BitStreamerUseDefaultCacheFillImpl;

use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_memory_bitstream::bitstream::BitOrderMSB;

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
