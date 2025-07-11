use super::{BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitVacuumerMSB<'a, W> = BitVacuumerBase<'a, BitOrderMSB, W>;

#[cfg(test)]
mod tests;
