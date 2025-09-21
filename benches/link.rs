use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use link2aws::{Arn, ArnOwned, ArnParts};

/// Load known good ARNs from test data file.
fn load_test_arns() -> Vec<ArnOwned> {
    serde_json::from_str::<serde_json::Value>(include_str!("../tests/data/aws.json"))
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(arn_str, _)| Arn::new(arn_str).ok())
        .map(|arn| arn.to_owned())
        .collect()
}

/// Benchmark [`ArnParts::link()`].
fn bench_arn_parts_link(c: &mut Criterion) {
    let test_arns = load_test_arns();
    c.bench_with_input(
        BenchmarkId::new("all_arns", test_arns.len()),
        &test_arns,
        |b, arns| {
            b.iter(|| {
                for arn in arns {
                    black_box(arn.link());
                }
            })
        },
    );
}

criterion_group!(benches, bench_arn_parts_link);
criterion_main!(benches);
