use super::{BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB {}

pub type BitVacuumerMSB<'a, W> = BitVacuumerBase<'a, BitOrderMSB, W>;

impl<W> BitVacuumer for BitVacuumerMSB<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
