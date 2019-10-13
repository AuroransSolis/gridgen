#[macro_use] extern crate criterion;

use criterion::{Criterion, black_box};

mod auro;
mod jetp;

use auro::gen_grid_bit_magic;
use jetp::create_grid;

use std::convert::TryInto;

const SIZES: [usize; 5] = [10, 50, 51, 100, 128];

fn bench_gen_grid_bit_magic(c: &mut Criterion) {
    for &size in SIZES.iter() {
        let id = format!("Generate grid {} (auro)", size);
        c.bench_function(id.as_str(), move |b| {
            b.iter(|| {
                black_box(gen_grid_bit_magic(size));
            })
        });
    }
}

fn bench_create_grid(c: &mut Criterion) {
    for &size in SIZES.iter() {
        let id = format!("Generate grid {} (jetp)", size);
        let size = size.try_into().unwrap();
        c.bench_function(id.as_str(), move |b| {
            b.iter(|| {
                black_box(create_grid(size));
            })
        });
    }
}

criterion_group!{
    name = bench_gen_grid;
    config = Criterion::default();
    targets = bench_gen_grid_bit_magic, bench_create_grid
}

criterion_main!{bench_gen_grid}
