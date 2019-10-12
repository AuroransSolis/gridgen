#[macro_use] extern crate criterion;

use criterion::{Criterion, black_box};

mod main;

use main::gen_grid_bit_magic;

const SIZES: [usize; 5] = [10, 50, 51, 100, 128];

fn bench_gen_grid_bit_magic(c: &mut Criterion) {
    for &size in SIZES.iter() {
        let id = format!("Generate grid {} (my method)", size);
        c.bench_function(id.as_str(), move |b| {
            b.iter(|| {
                black_box(gen_grid_bit_magic(size));
            })
        });
    }
}

criterion_group!{
    name = bench_gen_grid;
    config = Criterion::default();
    targets = bench_gen_grid_bit_magic
}

criterion_main!{bench_gen_grid}
