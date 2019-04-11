#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::{black_box, BatchSize};
use std::time::Duration;

extern crate rustzx;
use rustzx::emulator::Emulator;
use rustzx::settings::RustzxSettings;

fn emulation_benchmark(c: &mut Criterion) {
    c.bench_function("emulate", |b| {
        b.iter_batched(
            || {
                let settings = RustzxSettings::new();
                Emulator::new(&settings)
            },
            |mut emulator: Emulator| {
                for _ in 0..5 {
                    black_box(emulator.emulate_frames(Duration::from_secs(1)));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, emulation_benchmark);
criterion_main!(benches);
