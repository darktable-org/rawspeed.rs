use super::{BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderMSB16;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB16 {}

#[allow(dead_code)]
pub type BitVacuumerMSB16<'a, W> = BitVacuumerBase<'a, BitOrderMSB16, W>;

#[cfg(test)]
mod test;
