use rawspeed_bitstream_bitstreams::bitstreams::BitOrderMSB16;

use crate::bitvacuumer::{
    BitVacuumer, BitVacuumerBase, BitVacuumerUseDefaultDrainImpl,
};

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB16 {}

pub type BitVacuumerMSB16<'a, W> = BitVacuumerBase<'a, BitOrderMSB16, W>;

impl<W> BitVacuumer for BitVacuumerMSB16<'_, W> where W: std::io::Write {}

#[cfg(test)]
mod tests;
