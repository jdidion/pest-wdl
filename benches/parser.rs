use criterion::{criterion_group, criterion_main, Criterion};
use pest::iterators::Pair;
use pest_wdl_1::Rule;
use std::{fs, path::PathBuf};

pub fn benchmark_parser(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root.join("resources").join("test");
    let mut group = c.benchmark_group("parser");

    let strings_path = test_dir.join("strings.wdl");
    let strings_text = fs::read_to_string(strings_path.clone()).unwrap();
    group.bench_function("strings", |b| {
        b.iter(|| pest_wdl_1::parse_document(&strings_text))
    });

    let expressions_path = test_dir.join("expressions.wdl");
    let expressions_text = fs::read_to_string(expressions_path.clone()).unwrap();
    group.bench_function("expressions", |b| {
        b.iter(|| pest_wdl_1::parse_document(&expressions_text))
    });

    let comprehensive_path = test_dir.join("comprehensive.wdl");
    let comprehensive_text = fs::read_to_string(comprehensive_path.clone()).unwrap();
    group.bench_function("comprehensive", |b| {
        b.iter(|| pest_wdl_1::parse_document(&comprehensive_text))
    });
}

fn exhaust(pair: Pair<Rule>) {
    for child in pair.into_inner() {
        exhaust(child)
    }
}

pub fn benchmark_parser_exhaust(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root.join("resources").join("test");
    let comprehensive_path = test_dir.join("comprehensive.wdl");
    let comprehensive_text = fs::read_to_string(comprehensive_path.clone()).unwrap();
    c.bench_function("parser_exhaust", |b| {
        b.iter(|| {
            let doc = pest_wdl_1::parse_document(&comprehensive_text).unwrap();
            exhaust(doc)
        })
    });
}

criterion_group!(benches, benchmark_parser, benchmark_parser_exhaust);
criterion_main!(benches);
