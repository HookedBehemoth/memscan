use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::mem::size_of;

const ELEMENTS: usize = 128 * 1024;

macro_rules! def_bench {
    ($kind:ident, $ty:ty, $name:ident, $trait:ident) => {
        fn $name(c: &mut Criterion) {
            let mut group =
                c.benchmark_group(concat!(stringify!($kind), ": find_next ", stringify!($ty)));
            group
                .sample_size(10)
                .measurement_time(std::time::Duration::new(1, 0))
                .warm_up_time(std::time::Duration::new(0, 200000));
            unsafe {
                for i in 0..6 {
                    let element_size = std::mem::size_of::<$ty>();
                    let element_count = (ELEMENTS << (i * 2)) / element_size;
                    println!("elements: {element_count}");
                    let mut haystack: Vec<$ty> = (0..element_count).map(|_| 0 as $ty).collect();
                    haystack[element_count / 2] = 1 as $ty;
                    let haystack = std::slice::from_raw_parts(
                        haystack.as_ptr() as *const u8,
                        haystack.len() * size_of::<$ty>(),
                    );
                    group.bench_with_input(
                        BenchmarkId::new("None", element_count * element_size),
                        &(<$ty>::MAX, haystack),
                        |b, &(needle, haystack)| {
                            b.iter(|| {
                                black_box($trait::find_next(
                                    black_box(needle as $ty),
                                    black_box(haystack),
                                ))
                            })
                        },
                    );
                    group.bench_with_input(
                        BenchmarkId::new("inclusive", element_count * element_size),
                        &(<$ty>::MAX, haystack),
                        |b, &(needle, haystack)| {
                            b.iter(|| {
                                let two = needle / needle + needle / needle;
                                black_box($trait::find_inclusive_range(
                                    black_box(two),
                                    black_box(needle as $ty),
                                    black_box(haystack),
                                ))
                            })
                        },
                    );
                    group.bench_with_input(
                        BenchmarkId::new("exclusive", element_count * element_size),
                        &(<$ty>::MAX, haystack),
                        |b, &(needle, haystack)| {
                            b.iter(|| {
                                let two = needle / needle + needle / needle;
                                black_box($trait::find_exclusive_range(
                                    black_box(two),
                                    black_box(needle as $ty),
                                    black_box(haystack),
                                ))
                            })
                        },
                    );
                }
            }
            group.finish();
        }
    };
}

macro_rules! def_group {
    ($name:ident, $trait:ident) => {
        def_bench!($name, u8, u8_benchmark, $trait);
        def_bench!($name, u16, u16_benchmark, $trait);
        def_bench!($name, u32, u32_benchmark, $trait);
        def_bench!($name, u64, u64_benchmark, $trait);
        def_bench!($name, f32, f32_benchmark, $trait);
        def_bench!($name, f64, f64_benchmark, $trait);

        criterion_group!(
            $name,
            u8_benchmark,
            u16_benchmark,
            u32_benchmark,
            u64_benchmark,
            f32_benchmark,
            f64_benchmark,
        );
    };
}

mod avx2 {
    use super::*;

    use memscan::search::avx2::*;

    def_group!(avx2, Needle);
}

mod sse2 {
    use super::*;

    use memscan::search::sse42::*;

    def_group!(sse2, Needle);
}

mod primitive {
    use super::*;

    use memscan::search::primitive::*;

    def_group!(primitive, Needle);
}

criterion_main!(avx2::avx2, sse2::sse2, primitive::primitive);
