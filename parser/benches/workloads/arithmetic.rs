use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

fn calc_parser() -> impl Parser<StrInput<'static>, Output = i64, Error = ParseError> {
    recursive(|expr| {
        let ws_int = whitespace0().zip_right(integer()).zip_left(whitespace0());
        let atom = ws_int.or(
            whitespace0()
                .zip_right(char('('))
                .zip_right(expr)
                .zip_left(char(')'))
                .zip_left(whitespace0()),
        );

        let mul_op = whitespace0()
            .zip_right(
                char('*')
                    .map(|_| (|a: i64, b: i64| a * b) as fn(i64, i64) -> i64)
                    .or(char('/').map(|_| (|a, b| a / b) as fn(i64, i64) -> i64)),
            )
            .zip_left(whitespace0());

        let add_op = whitespace0()
            .zip_right(
                char('+')
                    .map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64)
                    .or(char('-').map(|_| (|a, b| a - b) as fn(i64, i64) -> i64)),
            )
            .zip_left(whitespace0());

        let term = atom.chainl1(mul_op);
        term.chainl1(add_op)
    })
}

const EXPR_INPUTS: &[(&str, &str)] = &[
    ("single", "42"),
    ("add", "1 + 2"),
    ("mul_add", "1 + 2 * 3"),
    ("parens", "(1 + 2) * 3"),
    ("complex", "1 + 2 * (3 - 4) + 5"),
    ("deeply_nested", "(((1 + 2) * 3) - 4) / 5"),
    ("long_chain", "1 + 2 + 3 + 4 + 5 + 6 + 7 + 8"),
];

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    for (name, input) in EXPR_INPUTS {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("oni-comb", name), input, |b, input| {
            b.iter(|| {
                let mut inp = StrInput::new(black_box(input));
                calc_parser().parse_next(&mut inp)
            })
        });
    }

    group.finish();
}
