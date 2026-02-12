use rawspeed_bitstream_bitstreams::bitstreams::BitOrderLSB;

use crate::bitvacuumer::{
    BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl,
};

impl BitVacuumerUseDefaultDrainImpl for BitOrderLSB {}

pub type BitVacuumerLSB<'a, W> = BitVacuumerBase<'a, BitOrderLSB, W>;

impl<W> BitVacuumer for BitVacuumerLSB<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
