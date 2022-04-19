use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
mod agefunctions;
use crate::agefunctions::get_drinking_message_via_if;
use crate::agefunctions::get_drinking_message_via_logical;

fn bench_both(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get Drink Message");
    for i in 15..31 {
        group.bench_with_input(BenchmarkId::new("Via Logical", i), &i, 
            |b, i| b.iter(|| get_drinking_message_via_logical(*i)));
        group.bench_with_input(BenchmarkId::new("Via If Statement", i), &i, 
            |b, i| b.iter(|| get_drinking_message_via_if(*i)));
    }
    group.finish();
}


criterion_group!(benches, bench_both);
criterion_main!(benches);