# oni-comb-rs (v2/reboot)

[![Workflow Status](https://github.com/j5ik2o/oni-comb-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/oni-comb-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/oni-comb-parser-rs.svg)](https://crates.io/crates/oni-comb-parser-rs)
[![docs.rs](https://docs.rs/oni-comb-parser-rs/badge.svg)](https://docs.rs/oni-comb-parser-rs)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/j5ik2o/oni-comb-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![dependency status](https://deps.rs/repo/github/j5ik2o/oni-comb-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/oni-comb-rs)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-APACHE2.0-blue.svg)](https://opensource.org/licenses/apache-2-0)

[日本語](README.ja.md)

A parser-monad combinator library for Rust (**v2/reboot**).

The old v1 design based on `Rc<dyn Fn>` has been replaced with **trait + concrete combinator types** (`Map`, `Zip`, `Or`, `FlatMap`, etc.). It provides the full Functor / Applicative / Alternative / Monad hierarchy while minimizing dynamic dispatch and heap allocation.

## Quickstart

```rust
use oni_comb_parser::prelude::*;

// Match 'a' or 'b'
let mut parser = char('a').or(char('b'));
let mut input = StrInput::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// Identifier: starts with letter/_, followed by alphanumeric/_
let mut ident = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'));
let mut input = StrInput::new("foo_bar_123");
let (head, tail) = ident.parse_next(&mut input).unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_bar_123");

// Integer
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInput::new("42");
assert_eq!(int_parser.parse_next(&mut input).unwrap(), 42);
```

## Design Highlights

- **Parsec-style recursive descent parser** — LL(1) by default, extensible to LL(\*) with `attempt`. `cut` commits to a branch for better error reporting. `flat_map` enables context-sensitive branching
- **Parser monad** — full Functor (`map`) / Applicative (`zip`) / Alternative (`or`) / Monad (`flat_map`) hierarchy
- **Zero-cost combinator composition** — Applicative combinators are built on the stack as concrete types with zero heap allocation. `flat_map` with same-type branches is also zero-cost
- **Backtrack / Cut error control** — `or` recovers only from `Backtrack`; `Cut` propagates as-is. `attempt` downgrades Cut→Backtrack, `cut` upgrades Backtrack→Cut
- **Boxed recursion** — only the recursive knot uses `Box<dyn Parser>`; non-recursive parts stay as concrete types

### Type Class Hierarchy and Cost

| Operation | Type Class | Rust Type | Cost |
|-----------|-----------|-----------|------|
| `p.map(f)` | Functor | `Map<P, F>` | Zero |
| `p1.zip(p2)` | Applicative | `Zip<P1, P2>` | Zero |
| `p1.zip_left(p2)` | Applicative | `ZipLeft<P1, P2>` | Zero |
| `p1.zip_right(p2)` | Applicative | `ZipRight<P1, P2>` | Zero |
| `p1.or(p2)` | Alternative | `Or<P1, P2>` | Zero |
| `p.flat_map(f)` same-type branch | Monad | `FlatMap<P, F>` | Zero |
| `p.flat_map(f)` heterogeneous branch | Monad | `FlatMap<P, F>` + `Box<dyn Parser>` | 1 Box |

`.flat_map()` is available on all parsers, but **for maximum performance, prefer Applicative combinators (`zip`, `map`, `or`)** and reserve `flat_map` for context-sensitive branching. This is the same recommendation as in Haskell's Parsec.

### Why Applicative-First?

In Rust, when a `flat_map` closure returns different parser types, type erasure via `Box<dyn Parser>` is required, incurring heap allocation + dynamic dispatch. The old v1 and pom built all combinators with `Rc<dyn Fn>`, which benchmarks show to be 3–39x slower than v2.

In contrast, `zip` (Applicative) is built on the stack as a concrete type like `Zip<Char, Tag>`, allowing the compiler to perform monomorphization → inlining → LLVM optimization end-to-end, achieving performance close to hand-written recursive descent parsers.

```rust
// Applicative: structure known at compile time → inlinable
let parser = char('a').zip(char('b'));   // Zip<Char, Char> — concrete type

// Monad (same type): no Box needed, zero cost
let parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|n| match n {
    '1' => tag("one"),
    _ => tag("other"),
});

// Monad (heterogeneous): type erasure via Box<dyn Parser>
let parser = satisfy(|c: char| c == 'c' || c == 't')
    .flat_map(|c| -> Box<dyn Parser<StrInput<'_>, Output = &str, Error = String>> {
        match c {
            'c' => Box::new(tag("har")),
            _ => Box::new(take_while1(|c: char| c.is_ascii_digit())),
        }
    });
```

## Available Parsers

### Text Parsers (`text` module)

| Function | Description | Return Type |
|----------|-------------|-------------|
| `char(c)` | Match a specific character | `char` |
| `tag(s)` | Match a specific string | `&str` |
| `satisfy(f)` | Match a character satisfying a predicate | `char` |
| `take_while0(f)` | Consume 0 or more characters satisfying a predicate | `&str` |
| `take_while1(f)` | Consume 1 or more characters satisfying a predicate | `&str` |
| `eof()` | Match end of input | `()` |
| `whitespace0()` | Consume 0 or more ASCII whitespace characters | `&str` |
| `whitespace1()` | Consume 1 or more ASCII whitespace characters | `&str` |
| `identifier()` | ASCII identifier (`[a-zA-Z_][a-zA-Z0-9_]*`) | `&str` |
| `integer()` | Signed integer | `i64` |
| `quoted_string()` | Double-quoted string (JSON-compliant escaping) | `String` |
| `quoted_string_cow()` | Zero-copy quoted_string (borrows when no escapes) | `Cow<'a, str>` |
| `escaped(open, close, esc, handler)` | Generic escaped string parser | `String` |
| `lexeme(p)` | Run parser then consume trailing whitespace | `P::Output` |
| `between(l, p, r)` | Run `l`, `p`, `r` in sequence and return `p`'s value | `P::Output` |
| `recursive(f)` | Build a recursive parser (closure receives recursive reference) | `P::Output` |
| `fn_parser(f)` | Wrap a function pointer as a `Parser` (ideal for vtable-free recursion) | `O` |

### Combinators (`ParserExt` method chain)

| Method | Type Class | Description |
|--------|-----------|-------------|
| `.map(f)` | Functor | Transform the success value |
| `.zip(p)` | Applicative | Apply two parsers sequentially, return a pair |
| `.zip_left(p)` | Applicative | Run both, return only the left value (= terminated) |
| `.zip_right(p)` | Applicative | Run both, return only the right value (= preceded) |
| `.or(p)` | Alternative | Try right if left returns Backtrack |
| `.flat_map(f)` | Monad | Dynamically select next parser based on first result |
| `.attempt()` | — | Downgrade Cut to Backtrack (make rewindable) |
| `.cut()` | — | Upgrade Backtrack to Cut (prevent backtracking via `or`) |
| `.optional()` | — | Convert Backtrack to `None` |
| `.many0()` | — | Repeat 0 or more times |
| `.many1()` | — | Repeat 1 or more times |
| `.sep_by0(sep)` | — | Repeat 0 or more times with separator |
| `.sep_by1(sep)` | — | Repeat 1 or more times with separator |
| `.chainl1(op)` | — | Left-associative binary operator chain |
| `.chainr1(op)` | — | Right-associative binary operator chain |

## Benchmarks

Comparison benchmarks against other libraries are included, using Criterion.rs.

### Compared Libraries

| Library | Design |
|---------|--------|
| **winnow** | Fastest class. `Parser` trait + `parse_next(&mut I)` — closest design to oni-comb-rs |
| **nom** | De facto standard. Function pointer based |
| **chumsky** | Error recovery focused. Trait-based combinators |
| **pom** | Operator overloading centric. Similar design to old v1 |

### Feature Comparison

| Feature | oni-comb | winnow | nom | chumsky | pom |
|---------|:--------:|:------:|:---:|:-------:|:---:|
| **Method chain API** (`p1.zip(p2)`) | o | o | x | o | x |
| **Parser monad** (full Functor/Applicative/Monad hierarchy) | o | x | x | x | o |
| **Zero heap allocation in Applicative composition** | o | o | o | x | x |
| **Zero-cost flat_map for same type** | o | o | o | x | x |
| **Structured errors** (position, expected tokens) | o | o | △ | o | x |
| **Explicit Backtrack / Cut control** | o | o | o | x | x |
| **`.context()` labeling** | o | o | △ | o | x |
| **`recursive()` helper** | o | x | x | o | x |
| **`chainl1` / `chainr1`** (operator associativity) | o | x | x | x | x |
| **`sep_by` / `between`** | o | o | o | o | o |
| **`no_std` support** (with `alloc`) | o | o | o | x | x |

- o = supported, △ = partial (requires additional support such as VerboseError), x = unsupported or not possible by design

**oni-comb-rs positioning**: Combines winnow/nom-level zero-cost performance with chumsky-level method chain ergonomics. The only library that also provides the Monad layer (`flat_map`) along with `chainl1`/`recursive()`.

*All figures below are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint).*

### Token Workload Results (Identifier) (mean)

As input grows longer, oni-comb-rs's `TakeWhile` byte scanning becomes more effective. chumsky 0.12 dramatically improved from v0.9 (~54x faster) and is now competitive on short inputs, though still ~2x slower on medium/long inputs.

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 16.6 ns | 15.5 ns | 14.9 ns | 17.1 ns | 66.3 ns |
| `"foo_bar_123"` (11B) | 38.9 ns | 21.7 ns | 33.4 ns | 83.8 ns | 230 ns |
| `"longIdentifier..."` (28B) | 81.7 ns | 34.2 ns | 82.7 ns | 132.0 ns | 266.5 ns |

### Token Workload Results (Integer) (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 8.8 ns | 3.8 ns | 3.8 ns | 77.2 ns | 17.2 ns |
| `"9999999"` (7B) | 20.3 ns | 5.7 ns | 5.8 ns | 136 ns | 32.3 ns |
| `"184467...615"` (20B) | 62.4 ns | 24.1 ns | 23.4 ns | 253 ns | 86.2 ns |

### flat_map Workload Results

#### Same-type branch (digit → tag, no Box needed) (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"1one"` | 7.2 ns | 2.7 ns | 2.4 ns | 49.7 ns | 69.4 ns |
| `"3three"` | 5.9 ns | 2.6 ns | 2.4 ns | 51.9 ns | 94.7 ns |

Improved from 8.3ns → 7.2ns through ParseError introduction + `#[inline]`. chumsky 0.12 improved from ~930ns to ~50ns.

#### Heterogeneous branch (`Box<dyn Parser>` / dynamic dispatch) (mean)

| Input | oni-comb | winnow | nom\* | chumsky | pom |
|-------|----------|--------|-------|---------|-----|
| `"c:hello"` | 30.9 ns | 20.1 ns | 3.7 ns | 25.4 ns | 163.7 ns |
| `"i:42"` | 23.8 ns | 18.0 ns | 3.0 ns | 19.7 ns | 111.0 ns |

\* nom's `Parser` trait is not dyn-compatible, so it uses manual two-stage parsing (no Box).

#### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| `"x"` | 4.3 ns | 4.2 ns | ≈0% (within margin) |
| `"foo_bar_123"` | 26.0 ns | 25.9 ns | ≈0% (within margin) |
| `"longIdentifier..."` | 64.9 ns | 64.7 ns | ≈0% (within margin) |

### JSON Subset (oni-comb only) (mean)

| Input | Time |
|-------|------|
| `null` | 8.5 ns |
| `42` | 83.4 ns |
| `"hello world"` | 143.8 ns |
| `[1, 2, 3]` | 517.6 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 663.5 ns |

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 155 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 443 ns |
| `(((1 + 2) * 3) - 4) / 5` | 905 ns |

### Full JSON Benchmark (107KB)

Measured on the same machine (100 samples) after adding the `pom` implementation to `json_full.rs`.

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| **oni-comb** | **193.4 µs** | **527.8** |
| winnow | 206.5 µs | 494.4 |
| nom | 262.8 µs | 388.5 |
| chumsky | 495.6 µs | 206.0 |
| pom | 7.56 ms | 13.5 |

Using `fn_parser` function recursion + `peek_byte` leading-byte dispatch + `quoted_string_cow` zero-copy, oni-comb outperforms winnow by 1.07x, nom by 1.36x, chumsky by 2.56x, and pom by 39.1x.

### Summary

- **Outperforms winnow in throughput** — 1.07x faster on 107KB JSON (`fn_parser` + `peek_byte` dispatch + zero-copy strings)
- **Competitive with nom on medium-to-long inputs** — comparable at 11B and 28B identifier
- **3–39x faster than pom** — demonstrates the gap vs. old v1-equivalent `Rc<dyn Fn>` design
- **chumsky 0.12 dramatically improved** — identifier "x": 918ns -> 17.1ns (~54x faster than v0.9). Now competitive on short inputs, but still ~2x slower on medium/long inputs
- **zip ≒ flat_map (same type)** — monadic composition is zero-cost thanks to concrete combinator type design
- **~83% cumulative improvement across 3 optimization rounds** — ParseError introduction (~12%) + `#[inline]` (~17%) + zero-copy + fn recursion (~77%)
- **2-5% improvement on JSON/arithmetic workloads** — ongoing minor improvements across all workloads
- **Zero heap allocation for Applicative / same-type flat_map** — verified 0 bytes / 0 blocks with dhat
- See [`modules/parser/benches/README.md`](modules/parser/benches/README.md) for detailed analysis

### Running Benchmarks

```bash
# Comparison benchmarks
cargo bench -p oni-comb-parser --bench comparison

# JSON / arithmetic benchmarks
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# Allocation measurement
cargo bench -p oni-comb-parser --bench alloc_count
```

## Crates

| Crate | Description |
|-------|-------------|
| [oni-comb-parser](modules/parser/README.md) | Core parser combinator library |
| [oni-comb-crond](modules/crond/README.md) | Cron expression parser & scheduler |
| [oni-comb-uri](modules/uri/README.md) | RFC 3986 URI parser (zero-copy, URN support) |

## Build & Test

```bash
# Build
cargo build

# Run all tests
cargo test -p oni-comb-parser

# Run specific test
cargo test -p oni-comb-parser -- test_name
```

## Roadmap

| MS | Name | Status | Content |
|----|------|--------|---------|
| 1 | Core | **Done** | Input, Fail, PResult, Parser, ParserExt, StrInput |
| 2 | Primitive | **Done** | eof, char, tag, satisfy, take_while0/1, peek |
| 3 | Combinators | **Done** | map, zip, zip_left, zip_right, between, or, attempt, cut, optional, many0/1, sep_by0/1, chainl1/r1, flat_map/and_then |
| 4 | Text module | **Done** | whitespace0/1, identifier, integer, quoted_string, escaped, lexeme. Validated with JSON subset and URI tokenizer tests |
| 5 | Recursive | **Done** | `recursive()` helper (`Rc<UnsafeCell<Box<dyn Parser>>>`). Validated with arithmetic + parentheses tests |
| 6 | Error reporting | **Done** | `ParseError` (position, expected tokens, context), `or` merging, `.context()` combinator |
| 7 | Benchmark | **Done** | 5-library comparison (token/flat_map), JSON subset, arithmetic, zip vs flat_map, dhat. Completed optimization cycle with ~12% improvement from ParseError introduction |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
