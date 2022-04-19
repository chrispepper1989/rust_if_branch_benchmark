use criterion::{black_box, criterion_group, criterion_main, Criterion};
const VALUES: &'static [&'static str] = &["Sorry no beer for you", "Not in the USA", "I will need to check you", "sure you can drink!"];

fn get_drinking_message_via_if(_age: i32) -> &'static str {
    if _age < 18 {
        return  &VALUES[0];
    }
    else if _age < 21 {
        return  &VALUES[1];
    }
    else if _age < 25 {
        return  &VALUES[2];
    }
    return  &VALUES[3];
}
fn get_drinking_message_via_logical(age: i32) -> &'static str{
    let value: usize =  (age < 18) as usize + (age < 21) as usize  + (age < 25) as usize;
    return &VALUES[value];
}

fn get_drinking_message(age: i32)
{
    get_drinking_message_via_logical(age);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get_drinking_message", |b| b.iter(|| get_drinking_message(black_box(20))));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);