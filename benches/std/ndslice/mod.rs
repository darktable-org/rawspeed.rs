use criterion::{
    Bencher, BenchmarkId, Criterion, Throughput, criterion_group,
    criterion_main,
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

fn bench_quadrant(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    dims: Dimensions2D<core::num::NonZero<usize>>,
    off: CoordOffset2D,
) {
    let quadrants = match (*off.row(), *off.col()) {
        (row, col) if (row == 0 && col == 0) => "I",
        (row, col) if (row == 0 && col != 0) => "I_II",
        (row, col) if (row != 0 && col == 0) => "I_IV",
        (row, col) if (row != 0 && col != 0) => "I_II_IV_III",
        (_, _) => unreachable!(),
    };
    let num_elts = dims.row_count().get() * dims.row_len().get();
    group.throughput(Throughput::ElementsAndBytes {
        elements: num_elts.try_into().unwrap(),
        bytes: (size_of::<T>() * num_elts).try_into().unwrap(),
    });
    group.bench_with_input(
        BenchmarkId::from_parameter(quadrants.to_owned()),
        &(dims, off),
        |b: &mut Bencher<'_>,
         p: &(Dimensions2D<core::num::NonZero<usize>>, CoordOffset2D)| {
            prepare_and_run_bench(b, p);
        },
    );
}

fn enumerate_quadrants(c: &mut Criterion) {
    let unit = 512;
    let dims = Dimensions2D::new(
        RowLength::new(core::num::NonZero::new(2 * unit).unwrap()),
        RowCount::new(core::num::NonZero::new(2 * unit).unwrap()),
    );

    let mut group = c.benchmark_group("iter");
    for row_offset in [0, unit] {
        let row_offset = RowOffset::new(row_offset.try_into().unwrap());
        for col_offset in [0, unit] {
            let col_offset = ColOffset::new(col_offset.try_into().unwrap());
            let off = CoordOffset2D::new(row_offset, col_offset);
            bench_quadrant(&mut group, dims, off);
        }
    }
    group.finish();
}

criterion_group!(benches, enumerate_quadrants);
criterion_main!(benches);
