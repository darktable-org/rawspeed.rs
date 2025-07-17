use super::{BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB {}

pub type BitVacuumerMSB<'a, W> = BitVacuumerBase<'a, BitOrderMSB, W>;

impl<W> BitVacuumer for BitVacuumerMSB<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
