//! Measure Fluent lookup times with `cargo bench -p warp_i18n`.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use warp_i18n::{FluentValue, Locale, args_from, init, render, set_locale};

fn bench_lookup(c: &mut Criterion) {
    init(Locale::ZhCn).expect("warp_i18n init");

    let mut group = c.benchmark_group("warp_i18n_lookup");

    set_locale(Locale::ZhCn);
    group.bench_function("plain_zh", |b| {
        b.iter(|| black_box(render(black_box("ui-button-ok"), None)));
    });

    set_locale(Locale::En);
    group.bench_function("plain_en", |b| {
        b.iter(|| black_box(render(black_box("ui-button-ok"), None)));
    });

    set_locale(Locale::ZhCn);
    group.bench_function("plural_zh", |b| {
        b.iter(|| {
            let args = args_from([("n", FluentValue::from(3))]);
            black_box(render(black_box("tabs-close-confirm"), Some(&args)))
        });
    });

    set_locale(Locale::ZhCn);
    group.bench_function("fallback_to_en", |b| {
        b.iter(|| black_box(render(black_box("core-fallback-only"), None)));
    });

    group.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
