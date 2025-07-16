use super::{BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB16;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB16 {}

#[cfg_attr(not(test), expect(dead_code))]
pub type BitVacuumerMSB16<'a, W> = BitVacuumerBase<'a, BitOrderMSB16, W>;

impl<W> BitVacuumer for BitVacuumerMSB16<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
