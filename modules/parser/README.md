# oni-comb-parser

[日本語](README.ja.md)

A parser-monad combinator library for Rust. The core crate of [oni-comb-rs](../../).

## Features

- **Parsec-style recursive descent** — LL(1) by default, LL(*) with `attempt`
- **Full type class hierarchy** — Functor (`map`) / Applicative (`zip`) / Alternative (`or`) / Monad (`flat_map`)
- **Zero-cost combinator composition** — Applicative combinators are concrete types on the stack, zero heap allocation
- **Backtrack / Cut error control** — `or` recovers from `Backtrack`; `Cut` propagates. `attempt` / `cut` to control
- **Structured errors** — `ParseError` with position, expected tokens, and `.context()` labels
- **Generic Input trait** — `StrInput<'a>` for `&str`, `ByteInput<'a>` for `&[u8]`
- **`no_std` support** — `#![no_std]` with `alloc`

## Performance

- **Full JSON benchmark lead (March 18, 2026 rerun)** — `109.5 µs`, `932.1 MiB/s` on the 107KB sample, ahead of `winnow` (`178.7 µs`, `571.3 MiB/s`)
- **Generic token parsers recovered** — identifier `"foo_bar_123"` is `18.6 ns`, integer `"184467...615"` is `20.0 ns`
- **Whitespace refactor result is mixed on subset JSON** — primitive-heavy cases regressed, but object-heavy cases improved and full JSON got faster
- **Remaining hotspot** — `flat_map` still trails `winnow` / `nom` on the smallest branch-dispatch microbenchmarks
- See [benchmark details](benches/README.md) and [Japanese benchmark notes](benches/README.ja.md)

## Quickstart

```rust
use oni_comb_parser::prelude::*;

// Match 'a' or 'b'
let mut parser = char('a').or(char('b'));
let mut input = StrInput::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// Identifier: letter/_ followed by alphanumeric/_
let mut input = StrInput::new("foo_123");
let (head, tail) = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'))
    .parse_next(&mut input)
    .unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_123");

// Integer
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInput::new("42");
assert_eq!(int_parser.parse_next(&mut input).unwrap(), 42);
```

## Available Parsers

### Text Parsers

| Function | Description | Output |
|----------|-------------|--------|
| `char(c)` | Match a specific character | `char` |
| `tag(s)` | Match a specific string | `&str` |
| `satisfy(f)` | Match a character satisfying predicate | `char` |
| `take_while0(f)` | Consume 0+ matching characters | `&str` |
| `take_while1(f)` | Consume 1+ matching characters | `&str` |
| `eof()` | Match end of input | `()` |
| `whitespace0()` / `whitespace1()` | Consume ASCII whitespace | `&str` |
| `identifier()` | ASCII identifier `[a-zA-Z_][a-zA-Z0-9_]*` | `&str` |
| `integer()` | Signed integer | `i64` |
| `quoted_string()` | JSON-compliant double-quoted string (borrows when unescaped) | `Cow<'a, str>` |
| `escaped(open, close, esc, handler)` | Generic escaped string | `String` |
| `lexeme(p)` | Run parser then consume trailing whitespace | `P::Output` |
| `between(l, p, r)` | Run l, p, r and return p's value | `P::Output` |
| `recursive(f)` | Build recursive parser | `P::Output` |
| `fn_parser(f)` | Wrap function as Parser | `O` |

### Combinators (ParserExt)

| Method | Type Class | Description |
|--------|-----------|-------------|
| `.map(f)` | Functor | Transform success value |
| `.zip(p)` | Applicative | Sequence two parsers, return pair |
| `.zip_left(p)` | Applicative | Run both, keep left |
| `.zip_right(p)` | Applicative | Run both, keep right |
| `.or(p)` | Alternative | Try right if left backtracks |
| `.flat_map(f)` | Monad | Context-sensitive branching |
| `.attempt()` | — | Downgrade Cut to Backtrack |
| `.cut()` | — | Upgrade Backtrack to Cut |
| `.optional()` | — | Convert Backtrack to None |
| `.many0()` / `.many1()` | — | Repeat 0+ / 1+ times |
| `.many0_fold(init, f)` / `.many1_fold(init, f)` | — | Fold 0+ / 1+ elements (zero-allocation) |
| `.many0_into(c)` / `.many1_into(c)` | — | Collect into custom `Extend` container |
| `.sep_by0(sep)` / `.sep_by1(sep)` | — | Separated repetition |
| `.sep_by0_fold(sep, init, f)` / `.sep_by1_fold(sep, init, f)` | — | Fold separated elements (zero-allocation) |
| `.sep_by0_into(sep, c)` / `.sep_by1_into(sep, c)` | — | Collect separated elements into custom container |
| `.chainl1(op)` / `.chainr1(op)` | — | Operator associativity chains |
| `.context(label)` | — | Add error context label |
| `.map_res(f, label)` | — | Transform with fallible function |

## Input Types

| Type | Token | Slice | Use Case |
|------|-------|-------|----------|
| `StrInput<'a>` | `char` | `&'a str` | Text parsing (default) |
| `ByteInput<'a>` | `u8` | `&'a [u8]` | Binary protocol parsing |

## Build & Test

```bash
cargo build -p oni-comb-parser
cargo test -p oni-comb-parser

# Benchmarks
cargo bench -p oni-comb-parser --bench comparison
```

Detailed benchmark tables and analysis:
- [English benchmark notes](benches/README.md)
- [Japanese benchmark notes](benches/README.ja.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
