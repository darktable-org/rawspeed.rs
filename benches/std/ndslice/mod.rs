use criterion::{
    AxisScale, Bencher, BenchmarkId, Criterion, PlotConfiguration, Throughput,
    criterion_group, criterion_main,
};
use rawspeed_std::coord_common::{
    ColOffset, CoordOffset2D, Dimensions2D, RowCount, RowLength, RowOffset,
    RowPitch,
};
use rawspeed_std_ndslice::{
    array2dref::Array2DRef, offsetarray2dref::OffsetArray2DRef,
};

#[derive(Debug)]
struct DataSerialDependency<T> {
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

type T = u8;

#[inline(never)]
fn run_bench(input: OffsetArray2DRef<'_, T>) -> T {
    let mut serial = DataSerialDependency::new();
    for row in input.rows() {
        for col in row.cols() {
            serial.serialize(*col);
        }
    }
    serial.finalize()
}

fn prepare_and_run_bench(
    b: &mut Bencher<'_>,
    p: &(Dimensions2D<core::num::NonZero<usize>>, CoordOffset2D),
) {
    let dims = p.0;
    let num_elts = dims.row_count().get() * dims.row_len().get();
    let storage = vec![0; num_elts];
    let view = Array2DRef::new(
        storage.as_slice(),
        dims.row_len(),
        RowPitch::new(dims.row_len().val()),
    );
    let view = OffsetArray2DRef::new(view, p.1);
    b.iter(|| run_bench(core::hint::black_box(view)));
}

fn bench_config(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    dims: Dimensions2D<core::num::NonZero<usize>>,
    off: CoordOffset2D,
) {
    let num_elts = dims.row_count().get() * dims.row_len().get();
    let num_bytes = size_of::<T>() * num_elts;
    group.throughput(Throughput::ElementsAndBytes {
        elements: num_elts.try_into().unwrap(),
        bytes: num_bytes.try_into().unwrap(),
    });
    group.bench_with_input(
        BenchmarkId::from_parameter(num_bytes),
        &(dims, off),
        |b: &mut Bencher<'_>,
         p: &(Dimensions2D<core::num::NonZero<usize>>, CoordOffset2D)| {
            prepare_and_run_bench(b, p);
        },
    );
}

fn enumerate_quadrants(c: &mut Criterion) {
    let sizes = || -> Box<dyn Iterator<Item = _>> {
        if true {
            let sizes = [16].into_iter();
            Box::new(sizes)
        } else {
            let sizes = core::iter::successors(Some(1_usize), |&prev| {
                prev.checked_mul(2)
            })
            .take_while(|s| *s <= 16 * 1024);
            Box::new(sizes)
        }
    };

    for with_row_offset in [false, true] {
        for with_col_offset in [false, true] {
            let quadrants = {
                if !with_row_offset && !with_col_offset {
                    "I"
                } else if !with_row_offset && with_col_offset {
                    "I_II"
                } else if with_row_offset && !with_col_offset {
                    "I_IV"
                } else if with_row_offset && with_col_offset {
                    "I_II_IV_III"
                } else {
                    unreachable!()
                }
            };
            let mut group = c.benchmark_group(format!("iter/{quadrants}"));
            group.plot_config(
                PlotConfiguration::default()
                    .summary_scale(AxisScale::Logarithmic),
            );
            for unit in sizes() {
                let dims = Dimensions2D::new(
                    RowLength::new(core::num::NonZero::new(2 * unit).unwrap()),
                    RowCount::new(core::num::NonZero::new(2 * unit).unwrap()),
                );
                let unit = unit.try_into().unwrap();
                let row_offset =
                    RowOffset::new(if with_row_offset { unit } else { 0 });
                let col_offset =
                    ColOffset::new(if with_col_offset { unit } else { 0 });
                let off = CoordOffset2D::new(row_offset, col_offset);
                bench_config(&mut group, dims, off);
            }
            group.finish();
        }
    }
}

criterion_group!(benches, enumerate_quadrants);
criterion_main!(benches);
