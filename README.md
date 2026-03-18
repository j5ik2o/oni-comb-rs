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

<p align="center">
  <img src="images/icon.png" alt="oni-comb-rs mascot" width="420">
</p>

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

*All figures below are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint).*

### Token Workload Results (Identifier) (mean)

In the March 18, 2026 rerun, oni-comb recovers the generic identifier path enough to edge out `winnow` on the medium/long ASCII cases shown below, while chumsky 0.12 remains dramatically better than older releases but still trails on non-trivial inputs.

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 15.0 ns | 15.2 ns | 15.1 ns | 17.5 ns | 68.2 ns |
| `"foo_bar_123"` (11B) | 18.6 ns | 20.3 ns | 33.2 ns | 86.9 ns | 202.0 ns |
| `"longIdentifier..."` (28B) | 30.2 ns | 33.2 ns | 86.5 ns | 132.7 ns | 269.7 ns |

### Token Workload Results (Integer) (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 2.6 ns | 2.8 ns | 2.8 ns | 70.8 ns | 20.7 ns |
| `"9999999"` (7B) | 5.2 ns | 5.4 ns | 5.4 ns | 132.3 ns | 29.5 ns |
| `"184467...615"` (20B) | 20.0 ns | 23.1 ns | 22.7 ns | 273.1 ns | 100.1 ns |

### flat_map Workload Results

#### Same-type branch (digit → tag, no Box needed) (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"1one"` | 5.7 ns | 2.6 ns | 3.2 ns | 72.8 ns | 76.2 ns |
| `"3three"` | 4.1 ns | 2.7 ns | 2.4 ns | 51.5 ns | 95.0 ns |

Improved from the older 8ns class through ParseError cleanup, cross-crate inlining, and the later generic token fast-path pass. The remaining gap is now mostly branch-dispatch overhead.

#### Heterogeneous branch (`Box<dyn Parser>` / dynamic dispatch) (mean)

| Input | oni-comb | winnow | nom\* | chumsky | pom |
|-------|----------|--------|-------|---------|-----|
| `"c:hello"` | 20.7 ns | 19.2 ns | 4.6 ns | 41.4 ns | 307.5 ns |
| `"i:42"` | 18.3 ns | 17.7 ns | 2.8 ns | 24.2 ns | 114.2 ns |

\* nom's `Parser` trait is not dyn-compatible, so it uses manual two-stage parsing (no Box).

#### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| `"x"` | 2.3 ns | 1.9 ns | same envelope |
| `"foo_bar_123"` | 6.7 ns | 6.6 ns | same envelope |
| `"longIdentifier..."` | 15.6 ns | 15.4 ns | same envelope |

### JSON Subset (oni-comb only) (mean)

| Input | Time |
|-------|------|
| `null` | 11.5 ns |
| `42` | 88.7 ns |
| `"hello world"` | 129.1 ns |
| `[1, 2, 3]` | 494.3 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 596.5 ns |

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 151 ns |
| `1 + 2 * 3` | 265 ns |
| `(1 + 2) * 3` | 429 ns |
| `(((1 + 2) * 3) - 4) / 5` | 912 ns |

### Full JSON Benchmark (107KB)

Rerun on March 18, 2026 on the same machine (100 samples) after updating the benchmark baseline from `winnow` 0.7 to 1.0.0.
Measurement machine: Mac mini (Mac16,11), Apple M4 Pro (14 cores: 10P + 4E), 64 GB RAM, macOS 26.3.1, arm64.

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| **oni-comb** | **109.5 µs** | **932.1** |
| winnow | 178.7 µs | 571.3 |
| nom | 282.8 µs | 360.9 |
| chumsky | 561.0 µs | 181.9 |
| pom | 7.69 ms | 13.3 |

On this rerun, oni-comb widens the full-JSON benchmark lead further. It delivers 1.63x the throughput of `winnow` 1.0.0, 2.58x that of nom, 5.12x that of chumsky, and 70.2x that of pom.

### Summary

- **oni-comb now leads the full-JSON macro benchmark by a wider margin** — 932.1 MiB/s vs winnow's 571.3 MiB/s
- **oni-comb stays well ahead of nom, chumsky, and pom on full JSON** — 2.58x faster than nom, 5.12x faster than chumsky, and 70.2x faster than pom
- **Generic identifier / integer parsers are no longer the clearest remaining gap** — the current rerun puts oni-comb ahead of winnow on the 11B identifier and 20B integer cases shown above
- **chumsky 0.12 dramatically improved** — short identifiers are still in the same ballpark as oni-comb, but medium/long inputs still trail substantially
- **flat_map remains the clearest microbenchmark gap** — especially same-type branch dispatch against winnow / nom
- **zip ≒ flat_map (same type)** — concrete combinator types still keep monadic composition in the same performance envelope
- **The whitespace refactor is mixed on JSON subset** — primitive-heavy cases regressed, but object parsing improved to ~596.5ns and full JSON moved in the right direction
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
