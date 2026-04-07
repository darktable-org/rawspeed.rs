use criterion::{
    AxisScale, Bencher, BenchmarkId, Criterion, PlotConfiguration, Throughput,
    criterion_group, criterion_main,
};
use rawspeed_bitstream_bitstream_decoder::bitstreamer::{
    BitStream, BitStreamerBase, BitStreamerTraits,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamFlowTrait;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderLSB, BitOrderMSB, BitOrderMSB16, BitOrderMSB32, BitOrderTrait,
    BitStreamTraits, MaximalPackedElementCount,
};
use rawspeed_common_bitseq::bitseq::BitSeq;

trait BitOrderName {
    #[must_use]
    fn name() -> &'static str;
}

impl BitOrderName for BitOrderLSB {
    #[inline]
    fn name() -> &'static str {
        "LSB"
    }
}

impl BitOrderName for BitOrderMSB {
    #[inline]
    fn name() -> &'static str {
        "MSB"
    }
}

impl BitOrderName for BitOrderMSB16 {
    #[inline]
    fn name() -> &'static str {
        "MSB16"
    }
}

impl BitOrderName for BitOrderMSB32 {
    #[inline]
    fn name() -> &'static str {
        "MSB32"
    }
}

#[derive(Debug)]
struct DataSerialDependency<T>
where
    T: Default + Clone + Copy,
    core::num::Wrapping<T>: core::ops::Add<Output = core::num::Wrapping<T>>,
{
    val: T,
}

impl<T> DataSerialDependency<T>
where
    T: Default + Clone + Copy,
    core::num::Wrapping<T>: core::ops::Add<Output = core::num::Wrapping<T>>,
{
    #[inline]
    #[must_use]
    fn new() -> Self {
        Self {
            val: Default::default(),
        }
    }

    #[inline]
    fn serialize(&mut self, rhs: T) {
        let val = core::num::Wrapping(self.val) + core::num::Wrapping(rhs);
        self.val = val.0;
    }

    #[inline]
    #[must_use]
    const fn finalize(self) -> T {
        self.val
    }
}

#[inline(never)]
fn run_bench<BitOrder>(
    input: &[u8],
    item_count: u64,
    item_packed_bitlen: u32,
) -> u64
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits,
    <BitOrder as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    for<'a> BitStreamerBase<'a, BitOrder>: TryFrom<&'a [u8]> + BitStream,
    for<'a> <BitStreamerBase<'a, BitOrder> as TryFrom<&'a [u8]>>::Error:
        core::fmt::Debug,
    BitOrder: BitStreamerTraits<[u8; 4]>,
    for<'d> BitSeq<u64>:
        From<BitSeq<<BitStreamerBase<'d, BitOrder> as BitStream>::T>>,
{
    let mut bs = BitStreamerBase::<BitOrder>::try_from(input).unwrap();
    let mut serial = DataSerialDependency::new();
    for _ in 0..item_count {
        bs.fill(item_packed_bitlen).unwrap();
        let item: BitSeq<u64> = bs.peek_bits_no_fill(item_packed_bitlen).into();
        serial.serialize(item.zext());
        bs.skip_bits_no_fill(item_packed_bitlen);
    }
    serial.finalize()
}

fn benchmark<BitOrder>(c: &mut Criterion)
where
    BitOrder: Clone + Copy + BitOrderTrait + BitStreamTraits + BitOrderName,
    <BitOrder as BitStreamTraits>::StreamFlow: BitStreamFlowTrait<u64>,
    for<'a> BitStreamerBase<'a, BitOrder>: TryFrom<&'a [u8]> + BitStream,
    for<'a> <BitStreamerBase<'a, BitOrder> as TryFrom<&'a [u8]>>::Error:
        core::fmt::Debug,
    BitOrder: BitStreamerTraits<[u8; 4]>,
    for<'d> BitSeq<u64>:
        From<BitSeq<<BitStreamerBase<'d, BitOrder> as BitStream>::T>>,
{
    static KIB: usize = 1024;

    let sizes: Box<dyn Iterator<Item = _>> = if true {
        let sizes = [4 * KIB].into_iter();
        Box::new(sizes)
    } else {
        let sizes =
            core::iter::successors(Some(1_usize), |&prev| prev.checked_mul(2))
                .take_while(|s| *s <= 2 * KIB * KIB);
        Box::new(sizes)
    };

    let mut group = c.benchmark_group(<BitOrder as BitOrderName>::name());
    group.plot_config(
        PlotConfiguration::default().summary_scale(AxisScale::Logarithmic),
    );
    for size in sizes {
        for item_packed_bitlen in 1..=32 {
            let p = MaximalPackedElementCount::new::<BitOrder>(
                size,
                item_packed_bitlen,
            );
            group.throughput(Throughput::ElementsAndBytes {
                elements: p.item_count,
                bytes: p.bytelen.try_into().unwrap(),
            });
            group.bench_with_input(
                BenchmarkId::from_parameter(format!(
                    "{size}/{item_packed_bitlen:02}",
                )),
                &p,
                |b: &mut Bencher<'_>, p: &MaximalPackedElementCount| {
                    let input = core::hint::black_box(vec![0; size]);
                    b.iter(|| {
                        run_bench::<BitOrder>(
                            input.as_slice(),
                            p.item_count,
                            item_packed_bitlen,
                        )
                    });
                },
            );
        }
    }
    group.finish();
}

criterion_group!(lsb, benchmark::<BitOrderLSB>);
criterion_group!(msb, benchmark::<BitOrderMSB>);
criterion_group!(msb16, benchmark::<BitOrderMSB16>);
criterion_group!(msb32, benchmark::<BitOrderMSB32>);
criterion_main!(lsb, msb, msb16, msb32);
