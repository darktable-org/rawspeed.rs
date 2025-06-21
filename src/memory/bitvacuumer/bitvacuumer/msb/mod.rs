use super::*;

use rawspeed_memory_bitstream::bitstream::BitOrderMSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB {}

#[allow(dead_code)]
pub type BitVacuumerMSB<'a, W> = BitVacuumerBase<'a, BitOrderMSB, W>;

#[cfg(test)]
mod test;
