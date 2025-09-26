use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use oni_comb_parser::core::{ParseError, ParseResult, ParseState, Parser};
use pom::parser::*;

fn generate_input(len: usize) -> Vec<u8> {
    let pattern = b"0123456789";
    let mut data = Vec::with_capacity(len);
    while data.len() < len {
        let remaining = len - data.len();
        if remaining >= pattern.len() {
            data.extend_from_slice(pattern);
        } else {
            data.extend_from_slice(&pattern[..remaining]);
        }
    }
    data
}

fn generate_csv_input(count: usize) -> Vec<u8> {
    let mut data = Vec::new();
    for i in 0..count {
        if i > 0 {
            data.push(b',');
        }
        let value = (i % 10_000) as u32;
        let s = value.to_string();
        data.extend_from_slice(s.as_bytes());
    }
    data
}

fn oni_comb_digit<'a>() -> Parser<'a, u8, u8> {
    Parser::new(|_: &'a [u8], state: ParseState<'a, u8>| {
        let remaining = state.input();
        match remaining.first() {
            Some(&value) if value.is_ascii_digit() => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, value, 1)
            }
            Some(_) => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(remaining),
                "expected digit",
            )),
            None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                None,
                "unexpected end of input",
            )),
        }
    })
}

fn oni_comb_sum_digits(input: &[u8]) -> usize {
    let parser = oni_comb_digit()
        .many1()
        .map(|digits| digits.iter().map(|d| (d - b'0') as usize).sum::<usize>());

    match parser.parse(input) {
        ParseResult::Success { value, state, .. } => {
            if let Some(state) = state {
                if state.len() != 0 {
                    panic!("oni-comb parser did not consume entire input");
                }
            }
            value
        }
        ParseResult::Failure { error, .. } => panic!("oni-comb parser failed: {}", error.message),
    }
}

fn oni_comb_csv_sum(input: &[u8]) -> usize {
    fn parser<'a>() -> Parser<'a, u8, usize> {
        Parser::new(|_: &'a [u8], state: ParseState<'a, u8>| {
            let slice = state.input();
            if slice.is_empty() {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(slice),
                    "empty input",
                ));
            }

            let mut offset = 0usize;
            let mut current = 0usize;
            let mut sum = 0usize;
            let mut has_digit = false;

            while offset < slice.len() {
                let b = slice[offset];
                if b.is_ascii_digit() {
                    has_digit = true;
                    current = current.saturating_mul(10) + (b - b'0') as usize;
                    offset += 1;
                } else if b == b',' {
                    if !has_digit {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            state.current_offset() + offset,
                            Some(&slice[offset..]),
                            "expected digit before comma",
                        ));
                    }
                    sum += current;
                    current = 0;
                    has_digit = false;
                    offset += 1;
                } else {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        state.current_offset() + offset,
                        Some(&slice[offset..]),
                        "unexpected character",
                    ));
                }
            }

            if !has_digit {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset() + offset,
                    None,
                    "trailing separator",
                ));
            }

            sum += current;
            let consumed = offset;
            let next_state = state.advance_by(consumed);
            ParseResult::successful_with_state(next_state, sum, consumed)
        })
    }

    match parser().parse(input) {
        ParseResult::Success { value, state, .. } => {
            if let Some(state) = state {
                if state.len() != 0 {
                    panic!("oni-comb CSV parser did not consume entire input");
                }
            }
            value
        }
        ParseResult::Failure { error, .. } => {
            panic!("oni-comb CSV parser failed: {}", error.message)
        }
    }
}

fn nom_sum_digits(input: &[u8]) -> usize {
    match nom::bytes::complete::take_while1::<_, _, nom::error::Error<&[u8]>>(|c: u8| {
        c.is_ascii_digit()
    })(input)
    {
        Ok((rest, digits)) => {
            if !rest.is_empty() {
                panic!("nom parser did not consume entire input");
            }
            digits.iter().map(|d| (d - b'0') as usize).sum()
        }
        Err(err) => panic!("nom parser failed: {err:?}"),
    }
}

fn nom_csv_sum(input: &[u8]) -> usize {
    use nom::{
        bytes::complete::{tag, take_while1},
        error::Error,
        multi::separated_list1,
    };

    let mut parser = separated_list1::<_, _, _, Error<&[u8]>, _, _>(
        tag(b","),
        take_while1(|c: u8| c.is_ascii_digit()),
    );

    match parser(input) {
        Ok((rest, parts)) => {
            if !rest.is_empty() {
                panic!("nom CSV parser did not consume entire input");
            }
            parts
                .into_iter()
                .map(|digits| {
                    digits
                        .iter()
                        .fold(0usize, |acc, d| acc * 10 + (d - b'0') as usize)
                })
                .sum()
        }
        Err(err) => panic!("nom CSV parser failed: {err:?}"),
    }
}

fn pom_sum_digits(input: &[u8]) -> usize {
    let digit = one_of(b"0123456789");
    let parser = digit.repeat(1..).map(|bytes| {
        bytes
            .into_iter()
            .map(|d| (d - b'0') as usize)
            .sum::<usize>()
    });

    match parser.parse_at(input, 0) {
        Ok((value, pos)) => {
            if pos != input.len() {
                panic!("pom parser did not consume entire input");
            }
            value
        }
        Err(err) => panic!("pom parser failed: {err:?}"),
    }
}

fn pom_csv_sum(input: &[u8]) -> usize {
    let digit = one_of(b"0123456789");
    let number = digit.repeat(1..).map(|bytes| {
        bytes
            .into_iter()
            .fold(0usize, |acc, d| acc * 10 + (d - b'0') as usize)
    });
    let comma = sym(b',');
    let parser = list(number, comma).map(|values| values.into_iter().sum::<usize>());

    match parser.parse_at(input, 0) {
        Ok((value, pos)) => {
            if pos != input.len() {
                panic!("pom CSV parser did not consume entire input");
            }
            value
        }
        Err(err) => panic!("pom CSV parser failed: {err:?}"),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_digits");
    for &size in &[1_024usize, 16_384, 131_072] {
        let data = generate_input(size);
        group.bench_with_input(BenchmarkId::new("oni-comb", size), &data, |b, data| {
            b.iter(|| {
                let result = oni_comb_sum_digits(black_box(data));
                black_box(result);
            });
        });
        group.bench_with_input(BenchmarkId::new("nom", size), &data, |b, data| {
            b.iter(|| {
                let result = nom_sum_digits(black_box(data));
                black_box(result);
            });
        });
        group.bench_with_input(BenchmarkId::new("pom", size), &data, |b, data| {
            b.iter(|| {
                let result = pom_sum_digits(black_box(data));
                black_box(result);
            });
        });
    }
    group.finish();

    let mut csv_group = c.benchmark_group("csv_numbers");
    for &count in &[256usize, 4_096, 32_768] {
        let data = generate_csv_input(count);
        csv_group.bench_with_input(BenchmarkId::new("oni-comb", count), &data, |b, data| {
            b.iter(|| {
                let result = oni_comb_csv_sum(black_box(data));
                black_box(result);
            });
        });
        csv_group.bench_with_input(BenchmarkId::new("nom", count), &data, |b, data| {
            b.iter(|| {
                let result = nom_csv_sum(black_box(data));
                black_box(result);
            });
        });
        csv_group.bench_with_input(BenchmarkId::new("pom", count), &data, |b, data| {
            b.iter(|| {
                let result = pom_csv_sum(black_box(data));
                black_box(result);
            });
        });
    }
    csv_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
