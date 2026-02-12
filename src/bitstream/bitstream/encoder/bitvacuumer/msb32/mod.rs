use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB32;

use crate::bitvacuumer::{
    BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl,
};

pub type BitVacuumerMSB32<'a, W> = BitVacuumerBase<'a, BitOrderMSB32, W>;

impl<W> BitVacuumer for BitVacuumerMSB32<'_, W> where W: std::io::Write {}

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB32 {}

#[cfg(test)]
mod tests;
