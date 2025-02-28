use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let num_elements: usize = 1_000_000_000;
    println!("Initializing data for {num_elements} items");

    let data = &mut vec![0u32; num_elements][..];

    for (idx, val) in data.iter_mut().enumerate() {
        *val = idx.try_into().unwrap();
    }

    c.bench_function("fib 20", |b| b.iter(|| {
        
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);