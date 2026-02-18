use criterion::{
    AxisScale, Bencher, BenchmarkId, Criterion, PlotConfiguration, Throughput,
    criterion_group, criterion_main,
};
use rawspeed_bitstream_bitstream_decoder::bitstreamer::{
    BitStreamerBase, BitStreamerCacheFillImpl, BitStreamerReplenisher,
    BitStreamerReplenisherStorage, BitStreamerTraits,
};
use rawspeed_bitstream_bitstreamcache::bitstreamcache::BitStreamCache;
use rawspeed_bitstream_bitstreams::bitstreams::{
    BitOrderLSB, BitOrderMSB, BitOrderMSB16, BitOrderMSB32, BitOrderTrait,
    BitStreamTraits,
};
use rawspeed_bitstream_bitstreamslice::bitstreamslice::BitStreamSliceConstraints;
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
fn run_bench<'a, BitOrder>(input: &'a [u8], item_count: u64, item_packed_bitlen: u32) -> u64
where
    BitOrder: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits
        + BitStreamSliceConstraints,
    BitStreamerBase<'a, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
    BitStreamerReplenisherStorage<'a, BitOrder>: BitStreamerReplenisher<'a, BitOrder>,
    <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
    BitSeq<u64>: From<
        BitSeq<<<BitOrder as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage>,
    >,
{
    let input = input.try_into().unwrap();
    let mut bs = BitStreamerBase::<BitOrder>::new(input);
    let mut serial = DataSerialDependency::new();
    for _ in 0..item_count {
        bs.fill(item_packed_bitlen).unwrap();
        let item = bs.peek_bits_no_fill(item_packed_bitlen);
        serial.serialize(item.zext());
        bs.skip_bits_no_fill(item_packed_bitlen);
    }
    serial.finalize()
}

struct MaximalPackedElementCount {
    bytelen: usize,
    item_count: u64,
}

impl MaximalPackedElementCount {
    #[inline]
    #[must_use]
    fn new<BitOrder>(bytelen: usize, item_packed_bitlen: u32) -> Self
    where
        BitOrder: BitOrderTrait + BitStreamerTraits,
    {
        let chunk_bytelen =
            size_of::<<BitOrder as BitStreamerTraits>::MaxProcessByteArray>();
        let chunk_bytelen = u64::try_from(chunk_bytelen).unwrap();
        let chunk_bitlen = chunk_bytelen.checked_mul(8).unwrap();
        let bytelen = u64::try_from(bytelen).unwrap();
        let num_chunks = bytelen.checked_div(chunk_bytelen).unwrap();
        let usable_bytelen = num_chunks.checked_mul(chunk_bytelen).unwrap();
        let usable_bytelen = usable_bytelen.try_into().unwrap();
        let num_bits = num_chunks.checked_mul(chunk_bitlen).unwrap();
        let item_count =
            num_bits.checked_div(item_packed_bitlen.into()).unwrap();
        Self {
            bytelen: usable_bytelen,
            item_count,
        }
    }
}

fn benchmark<BitOrder>(c: &mut Criterion)
where
    BitOrder: BitOrderTrait + BitStreamerTraits + BitOrderName,
    BitOrder: Clone
        + Copy
        + BitOrderTrait
        + BitStreamTraits
        + BitStreamerTraits
        + BitStreamSliceConstraints,
    for<'a> BitStreamerBase<'a, BitOrder>: BitStreamerCacheFillImpl<BitOrder>,
    for<'a> BitStreamerReplenisherStorage<'a, BitOrder>: BitStreamerReplenisher<'a, BitOrder>,
    <BitOrder as BitStreamTraits>::StreamFlow: Default + BitStreamCache,
    BitSeq<u64>: From<
        BitSeq<<<BitOrder as BitStreamTraits>::StreamFlow as BitStreamCache>::Storage>,
    >,
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
