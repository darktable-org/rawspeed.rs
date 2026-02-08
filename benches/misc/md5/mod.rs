use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
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
    static L1D: usize = 512 * KIB;

    let sizes =
        core::iter::successors(Some(1_usize), |&prev| prev.checked_mul(2))
            .take_while(|s| *s <= L1D);
    let sizes = [0].into_iter().chain(sizes);

    let mut group = c.benchmark_group("md5");
    for size in sizes {
        let input = vec![0; size];
        group.throughput(Throughput::Bytes(size.try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            input.as_slice(),
            |b, input| b.iter(|| md5_slice(input)),
        );
    }
    group.finish();
}

criterion_group!(benches, md5_benchmark);
criterion_main!(benches);
