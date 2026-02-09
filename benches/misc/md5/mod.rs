use criterion::{
    AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput,
    criterion_group, criterion_main,
};
use rawspeed_misc_md5::md5::{self, MD5};

#[inline(never)]
fn md5_slice(input: &[u8]) -> md5::MD5State {
    let mut hasher = MD5::default();
    hasher.extend(input);
    hasher.flush()
}

fn md5_benchmark(c: &mut Criterion) {
    static KIB: usize = 1024;

    let sizes: Box<dyn Iterator<Item = _>> = if true {
        let sizes = [4 * KIB].into_iter();
        Box::new(sizes)
    } else {
        let sizes =
            core::iter::successors(Some(1_usize), |&prev| prev.checked_mul(2))
                .take_while(|s| *s <= 2 * KIB * KIB);
        let sizes = [0].into_iter().chain(sizes);
        Box::new(sizes)
    };

    let mut group = c.benchmark_group("md5");
    group.plot_config(
        PlotConfiguration::default().summary_scale(AxisScale::Logarithmic),
    );
    for size in sizes {
        group.throughput(Throughput::Bytes(size.try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, size| {
                let input = core::hint::black_box(vec![0; *size]);
                b.iter(|| md5_slice(input.as_slice()));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, md5_benchmark);
criterion_main!(benches);
