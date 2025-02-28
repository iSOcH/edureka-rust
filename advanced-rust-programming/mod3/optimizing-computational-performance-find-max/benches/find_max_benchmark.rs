use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

use optimizing_computational_performance_find_max::{single_threaded::*, multi_threaded::Threaded, FindMax};

pub fn criterion_benchmark(c: &mut Criterion) {
    let num_elements: usize = 1_000_000_000;
    println!("Initializing data for {num_elements} items");

    let data = &mut vec![0u32; num_elements][..];

    for (idx, val) in data.iter_mut().enumerate() {
        *val = idx.try_into().unwrap();
    }

    let mut group = c.benchmark_group("find_max");
    group
        .sample_size(20)
        .measurement_time(Duration::from_secs(60));

    let subjects: Vec<Box<dyn FindMax>> = vec![
        Box::new(SingleThreadedNaive),
        Box::new(StdLib),
        Box::new(SimdArgmaxCrate),
        Box::new(SimdPulpCrate),
        Box::new(Threaded::new(2, Box::new(SimdArgmaxCrate))),
        Box::new(Threaded::new(4, Box::new(SimdArgmaxCrate))),
        Box::new(Threaded::new(8, Box::new(SimdArgmaxCrate))),
        Box::new(Threaded::new(16, Box::new(SimdArgmaxCrate))),
        Box::new(Threaded::new(2, Box::new(SimdPulpCrate))),
        Box::new(Threaded::new(4, Box::new(SimdPulpCrate))),
        Box::new(Threaded::new(8, Box::new(SimdPulpCrate))),
        Box::new(Threaded::new(16, Box::new(SimdPulpCrate))),
    ];

    for sub in subjects {
        group.bench_function(&format!("{sub:?}"), |b| b.iter(|| {
            sub.find_max(&data);
        }));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);