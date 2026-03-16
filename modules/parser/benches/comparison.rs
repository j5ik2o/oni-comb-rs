use criterion::{criterion_group, criterion_main};

mod impls;
mod workloads;

criterion_group!(
    benches,
    workloads::token::register,
    workloads::json::register,
    workloads::arithmetic::register,
);
criterion_main!(benches);
