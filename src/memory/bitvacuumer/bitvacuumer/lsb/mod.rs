use super::{BitVacuumerBase, BitVacuumerUseDefaultDrainImpl};

use rawspeed_memory_bitstream::bitstream::BitOrderLSB;

impl BitVacuumerUseDefaultDrainImpl for BitOrderLSB {}

#[allow(dead_code)]
pub type BitVacuumerLSB<'a, W> = BitVacuumerBase<'a, BitOrderLSB, W>;

#[cfg(test)]
mod test;
