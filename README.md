# oni-comb-rs (v3/reboot)

[![Workflow Status](https://github.com/j5ik2o/oni-comb-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/oni-comb-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/oni-comb-parser-rs.svg)](https://crates.io/crates/oni-comb-parser-rs)
[![docs.rs](https://docs.rs/oni-comb-parser-rs/badge.svg)](https://docs.rs/oni-comb-parser-rs)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/j5ik2o/oni-comb-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![dependency status](https://deps.rs/repo/github/j5ik2o/oni-comb-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/oni-comb-rs)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-APACHE2.0-blue.svg)](https://opensource.org/licenses/apache-2-0)

[日本語](README.ja.md)

<p align="center">
  <img src="images/icon.png" alt="oni-comb-rs mascot" width="420">
</p>

A parser-monad combinator library for Rust (**v3/reboot**).

The old v1 design based on `Rc<dyn Fn>` has been replaced with **trait + concrete combinator types** (`Map`, `Zip`, `Or`, `FlatMap`, etc.). It provides the full Functor / Applicative / Alternative / Monad hierarchy while minimizing dynamic dispatch and heap allocation.

## Quickstart

```rust
use oni_comb_parser::prelude::*;

// Match 'a' or 'b'
let mut parser = char('a').or(char('b'));
let mut input = StrInputStream::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// Identifier: starts with letter/_, followed by alphanumeric/_
let mut ident = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'));
let mut input = StrInputStream::new("foo_bar_123");
let (head, tail) = ident.parse_next(&mut input).unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_bar_123");

// Integer
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInputStream::new("42");
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

In Rust, when a `flat_map` closure returns different parser types, type erasure via `Box<dyn Parser>` is required, incurring heap allocation + dynamic dispatch. The old v1 and pom built all combinators with `Rc<dyn Fn>`, which benchmarks show to be 3–39x slower than v3.

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
    .flat_map(|c| -> Box<dyn Parser<StrInputStream<'_>, Output = &str, Error = String>> {
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
| `quoted_string()` | Double-quoted string (JSON-compliant escaping, borrows when unescaped) | `Cow<'a, str>` |
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
| `.many0_fold(init, f)` | — | Fold 0+ elements (zero-allocation) |
| `.many1_fold(init, f)` | — | Fold 1+ elements (zero-allocation) |
| `.many0_into(container)` | — | Collect 0+ elements into custom `Extend` container |
| `.many1_into(container)` | — | Collect 1+ elements into custom `Extend` container |
| `.sep_by0(sep)` | — | Repeat 0 or more times with separator |
| `.sep_by1(sep)` | — | Repeat 1 or more times with separator |
| `.sep_by0_fold(sep, init, f)` | — | Fold 0+ separated elements (zero-allocation) |
| `.sep_by1_fold(sep, init, f)` | — | Fold 1+ separated elements (zero-allocation) |
| `.sep_by0_into(sep, container)` | — | Collect 0+ separated elements into custom container |
| `.sep_by1_into(sep, container)` | — | Collect 1+ separated elements into custom container |
| `.chainl1(op)` | — | Left-associative binary operator chain |
| `.chainr1(op)` | — | Right-associative binary operator chain |
| `.context(label)` | — | Add error context label |
| `.map_res(f, label)` | — | Transform with fallible function |

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

*The token / JSON subset / arithmetic figures below are from the March 21, 2026 reruns of `cargo bench -p oni-comb-parser --bench comparison` and `cargo bench -p oni-comb-parser --bench comparison -- json`. All figures are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint). The full JSON section below is the separate March 21, 2026 `json_full` rerun after the latest `take_while*` hot-path cleanup.*

### Token Workload Results (Identifier) (mean)

In the March 21, 2026 rerun, oni-comb stays very close to `winnow` on the medium/long ASCII cases shown below, while chumsky 0.12 remains dramatically better than older releases but still trails on non-trivial inputs.

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 14.6 ns | 15.5 ns | 14.0 ns | 16.6 ns | 67.7 ns |
| `"foo_bar_123"` (11B) | 20.0 ns | 20.5 ns | 33.3 ns | 85.0 ns | 205.1 ns |
| `"longIdentifier..."` (28B) | 33.8 ns | 33.6 ns | 83.1 ns | 133.4 ns | 272.5 ns |

### Token Workload Results (Integer) (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 3.1 ns | 2.8 ns | 2.6 ns | 74.3 ns | 21.7 ns |
| `"9999999"` (7B) | 6.8 ns | 6.2 ns | 5.2 ns | 133.0 ns | 29.6 ns |
| `"184467...615"` (20B) | 22.8 ns | 22.7 ns | 22.5 ns | 252.5 ns | 95.1 ns |

### flat_map Workload Results

#### Same-type branch (digit → tag, no Box needed) (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"1one"` | 10.6 ns | 2.4 ns | 2.6 ns | 49.1 ns | 69.9 ns |
| `"3three"` | 10.4 ns | 2.6 ns | 2.7 ns | 52.0 ns | 95.2 ns |

The latest rerun is notably slower than the earlier March 18 snapshot. The remaining gap is still mostly branch-dispatch overhead, but the current same-type flat_map path is back in the ~10ns class rather than the ~5ns class.

#### Heterogeneous branch (`Box<dyn Parser>` / dynamic dispatch) (mean)

| Input | oni-comb | winnow | nom\* | chumsky | pom |
|-------|----------|--------|-------|---------|-----|
| `"c:hello"` | 24.2 ns | 20.1 ns | 3.7 ns | 25.8 ns | 166.3 ns |
| `"i:42"` | 21.9 ns | 17.2 ns | 2.7 ns | 18.7 ns | 109.5 ns |

\* nom's `Parser` trait is not dyn-compatible, so it uses manual two-stage parsing (no Box).

#### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| `"x"` | 3.1 ns | 2.4 ns | flat_map faster |
| `"foo_bar_123"` | 8.4 ns | 7.9 ns | flat_map slightly faster |
| `"longIdentifier..."` | 18.8 ns | 18.0 ns | flat_map slightly faster |

### JSON Subset (oni-comb only) (mean)

| Input | Time |
|-------|------|
| `null` | 16.5 ns |
| `42` | 89.7 ns |
| `"hello world"` | 138.1 ns |
| `[1, 2, 3]` | 505.2 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 661.3 ns |

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 166 ns |
| `1 + 2 * 3` | 299 ns |
| `(1 + 2) * 3` | 487 ns |
| `(((1 + 2) * 3) - 4) / 5` | 1,044 ns |

### Full JSON Benchmark (107KB)

Rerun on March 21, 2026 on the same machine (100 samples).
Measurement machine: Mac mini (Mac16,11), Apple M4 Pro (14 cores: 10P + 4E), 64 GB RAM, macOS 26.3.1, arm64.

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| oni-comb | 671.7 µs | 152.0 |
| **winnow** | **176.0 µs** | **580.0** |
| nom | 286.1 µs | 356.8 |
| chumsky | 493.9 µs | 206.7 |
| pom | 7.88 ms | 13.0 |

On this rerun, `winnow` still leads the full-JSON benchmark, followed by `nom` and `chumsky`. oni-comb improves to 152.0 MiB/s after the latest `take_while*` hot-path cleanup and remains well ahead of `pom`, but the realistic 107KB payload is still slower than the top three parsers.

### Summary

- **`winnow` still leads the full-JSON macro benchmark** — 580.0 MiB/s, with `nom` at 356.8 MiB/s and `chumsky` at 206.7 MiB/s; oni-comb now reaches 152.0 MiB/s
- **The latest `take_while*` hot-path cleanup recovered JSON throughput** — subset inputs improved to `null`: 16.5ns and object-heavy cases to ~661ns / ~1.38µs, while the 107KB full-JSON run improved to 671.7µs
- **Generic identifier / integer parsers remain competitive, but the latest `comparison` rerun is tighter** — oni-comb is still close to winnow on the 11B identifier and effectively tied with winnow / nom on the 20B integer case
- **chumsky 0.12 dramatically improved** — short identifiers are still in the same ballpark as oni-comb (`"x"`: 16.6ns vs 14.6ns), but medium/long inputs still trail substantially (`"foo_bar_123"`: 85.0ns vs 20.0ns)
- **flat_map remains the clearest microbenchmark gap** — especially same-type branch dispatch against winnow / nom
- **zip ≒ flat_map (same type)** — concrete combinator types still keep monadic composition in the same performance envelope
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
| 1 | Core | **Done** | InputStream, Fail, PResult, Parser, ParserExt, StrInputStream |
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
