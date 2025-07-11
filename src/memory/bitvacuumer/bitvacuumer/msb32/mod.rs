use super::{BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB32;

#[cfg_attr(not(test), expect(dead_code))]
pub type BitVacuumerMSB32<'a, W> = BitVacuumerBase<'a, BitOrderMSB32, W>;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB32 {}

#[cfg(test)]
mod tests;
