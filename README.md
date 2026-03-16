# oni-comb-rs

[日本語](README.ja.md)

A parser-monad combinator library for Rust (v2 reboot).

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

In Rust, when a `flat_map` closure returns different parser types, type erasure via `Box<dyn Parser>` is required, incurring heap allocation + dynamic dispatch. The old v1 and pom built all combinators with `Rc<dyn Fn>`, which benchmarks show to be 3–30x slower than v2.

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

As input grows longer, oni-comb-rs's `TakeWhile` byte scanning becomes more effective, significantly outpacing nom.

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"x"` (1B) | 14.9 ns | 15.2 ns | 13.2 ns | 66.4 ns | 894 ns |
| `"foo_bar_123"` (11B) | 26.7 ns | 21.4 ns | 37.8 ns | 199 ns | 1,144 ns |
| `"longIdentifier..."` (28B) | 44.1 ns | 33.6 ns | 82.2 ns | 269 ns | 1,447 ns |

### Token Workload Results (Integer) (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 3.6 ns | 2.3 ns | 2.7 ns | 72 ns | 907 ns |
| `"9999999"` (7B) | 8.2 ns | 5.2 ns | 5.3 ns | 133 ns | 995 ns |
| `"184467...615"` (20B) | 25.9 ns | 22.6 ns | 22.7 ns | 264 ns | 1,256 ns |

### flat_map Workload Results

#### Same-type branch (digit → tag, no Box needed) (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"1one"` | 6.1 ns | 2.6 ns | 2.4 ns | 70 ns | 892 ns |
| `"3three"` | 4.8 ns | 2.6 ns | 2.4 ns | 94 ns | 929 ns |

Improved from 8.3ns → 6.1ns (~26% cumulative improvement) through ParseError introduction + `#[inline]`.

#### Heterogeneous branch (`Box<dyn Parser>` / dynamic dispatch) (mean)

| Input | oni-comb | winnow | nom\* | pom | chumsky |
|-------|----------|--------|-------|-----|---------|
| `"c:hello"` | 21.5 ns | 19.3 ns | 3.9 ns | 164 ns | 1,052 ns |
| `"i:42"` | 19.8 ns | 18.6 ns | 2.8 ns | 109 ns | 972 ns |

\* nom's `Parser` trait is not dyn-compatible, so it uses manual two-stage parsing (no Box).

#### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| `"x"` | 4.8 ns | 4.8 ns | 0% (within margin) |
| `"foo_bar_123"` | 17.7 ns | 17.7 ns | 0% (within margin) |
| `"longIdentifier..."` | 31.2 ns | 31.1 ns | 0% (within margin) |

### JSON Subset (oni-comb only) (mean)

| Input | Time |
|-------|------|
| `null` | 15 ns |
| `42` | 86 ns |
| `"hello world"` | 147 ns |
| `[1, 2, 3]` | 536 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 693 ns |

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 156 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 440 ns |
| `(((1 + 2) * 3) - 4) / 5` | 931 ns |

### Full JSON Benchmark (107KB — chumsky bench compatible)

Measured on the same machine (100 samples):

| Library | Mean | Median | p90 | p95 | StdDev | Throughput (mean) |
|---------|------|--------|-----|-----|--------|-------------------|
| **oni-comb** | **109.6 µs** | **109.4 µs** | **112.7 µs** | **113.8 µs** | **2.10 µs** | **977 MB/s** |
| winnow | 159.3 µs | 159.8 µs | 161.8 µs | 162.3 µs | 2.46 µs | 672 MB/s |
| nom | 283.2 µs | 282.7 µs | 286.6 µs | 287.9 µs | 2.26 µs | 378 MB/s |

Using `fn_parser` function recursion + `peek_byte` leading-byte dispatch + `quoted_string_cow` zero-copy, oni-comb outperforms winnow by 1.45x. All 3 libraries show stable StdDev ~2µs.

### Summary

- **Outperforms winnow in throughput** — 1.43x faster on 107KB JSON (`fn_parser` + `peek_byte` dispatch + zero-copy strings)
- **Outperforms nom on medium-to-long inputs** — 28% faster at 11B, 46% faster at 28B (identifier)
- **3–30x faster than pom** — demonstrates the gap vs. old v1-equivalent `Rc<dyn Fn>` design
- **30–200x faster than chumsky**
- **zip ≒ flat_map (same type)** — monadic composition is zero-cost thanks to concrete combinator type design
- **~83% cumulative improvement across 3 optimization rounds** — ParseError introduction (~12%) + `#[inline]` (~17%) + zero-copy + fn recursion (~77%)
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
| [oni-comb-parser](modules/parser/) | Core parser combinator library |
| [oni-comb-crond](modules/crond/) | Cron expression parser & scheduler |
| [oni-comb-uri](modules/uri/) | RFC 3986 URI parser (zero-copy, URN support) |

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
