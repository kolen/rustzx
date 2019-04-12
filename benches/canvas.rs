#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate rustzx;
use rustzx::utils::clocks::Clocks;
use rustzx::zx::machine::ZXMachine;
use rustzx::zx::screen::canvas::ZXCanvas;

fn process_clocks_benchmark(c: &mut Criterion) {
    c.bench_function("Canvas::process_clocks", |b| {
        let mut canvas = ZXCanvas::new(ZXMachine::Sinclair48K);
        b.iter(|| {
            for f in 0..ZXMachine::Sinclair48K.specs().clocks_frame {
                black_box(canvas.process_clocks(Clocks(f)));
            }
        });
    });
}

fn update_benchmark(c: &mut Criterion) {
    c.bench_function("Canvas::update", |b| {
        let mut canvas = ZXCanvas::new(ZXMachine::Sinclair48K);
        b.iter(|| {
            for offset in 0..0x1b00 {
                black_box(canvas.update(offset, 0, 0b1010_1010));
            }
        })
    });
}

criterion_group!(benches, process_clocks_benchmark, update_benchmark);
criterion_main!(benches);
