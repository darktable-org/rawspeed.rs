use rawspeed_std_ndslice::array2drefmut::Array2DRefMut;

pub trait RawDemuxer {
    fn decode(&self, output: &mut Array2DRefMut<'_, u16>)
    -> Result<(), String>;
}
