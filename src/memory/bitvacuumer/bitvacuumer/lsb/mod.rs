use super::{BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderLSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderLSB {}

pub type BitVacuumerLSB<'a, W> = BitVacuumerBase<'a, BitOrderLSB, W>;

impl<W> BitVacuumer for BitVacuumerLSB<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
