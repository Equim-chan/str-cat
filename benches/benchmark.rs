use std::hint::black_box;
use std::net::Ipv4Addr;
use str_cat::str_cat;

use criterion::{criterion_group, criterion_main, Criterion};

fn str_cat_vs_format(c: &mut Criterion) {
    let mut g = c.benchmark_group("literal string");
    g.bench_function("str_cat", |b| {
        b.iter(|| {
            let s = str_cat!(
                black_box("Hello"),
                black_box(" "),
                black_box("World"),
                black_box("!"),
            );
            assert_eq!(s, "Hello World!");
        })
    });
    g.bench_function("format", |b| {
        b.iter(|| {
            let s = format!(
                "{}{}{}{}",
                black_box("Hello"),
                black_box(" "),
                black_box("World"),
                black_box("!"),
            );
            assert_eq!(s, "Hello World!");
        })
    });
    g.finish();

    let mut g = c.benchmark_group("str+int");
    g.bench_function("str_cat", |b| {
        b.iter(|| {
            let s = str_cat!(black_box("Number: "), black_box(202302_u64).to_string(),);
            assert_eq!(s, "Number: 202302");
        })
    });
    g.bench_function("format", |b| {
        b.iter(|| {
            let s = format!("{}{}", black_box("Number: "), black_box(202302_u64),);
            assert_eq!(s, "Number: 202302");
        })
    });
    g.finish();

    let mut g = c.benchmark_group("display");
    g.bench_function("str_cat", |b| {
        b.iter(|| {
            let s = str_cat!(
                black_box(true).to_string(),
                black_box(202302_u64).to_string(),
                black_box(2.02302_f64).to_string(),
                black_box(Ipv4Addr::LOCALHOST).to_string(),
            );
            assert_eq!(s, "true2023022.02302127.0.0.1");
        })
    });
    g.bench_function("format", |b| {
        b.iter(|| {
            let s = format!(
                "{}{}{}{}",
                black_box(true),
                black_box(202302_u64),
                black_box(2.02302_f64),
                black_box(Ipv4Addr::LOCALHOST),
            );
            assert_eq!(s, "true2023022.02302127.0.0.1");
        })
    });
    g.finish();
}

criterion_group!(benches, str_cat_vs_format);
criterion_main!(benches);
