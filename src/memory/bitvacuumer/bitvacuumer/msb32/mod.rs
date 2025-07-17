use super::{BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB32;

pub type BitVacuumerMSB32<'a, W> = BitVacuumerBase<'a, BitOrderMSB32, W>;

impl<W> BitVacuumer for BitVacuumerMSB32<'_, W> where W: std::io::Write {}

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB32 {}

#[cfg(test)]
mod tests;
