# Benchmarks

[日本語](README.ja.md)

Performance comparison of oni-comb-rs v2 against other libraries (winnow, nom, chumsky, pom).

## How to Run

```bash
# Run comparison benchmarks (token / JSON subset / arithmetic)
cargo bench -p oni-comb-parser --bench comparison

# Run specific comparison groups
cargo bench -p oni-comb-parser --bench comparison -- identifier
cargo bench -p oni-comb-parser --bench comparison -- integer
cargo bench -p oni-comb-parser --bench comparison -- flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# Run the standalone full JSON benchmark (107KB sample.json)
cargo bench -p oni-comb-parser --bench json_full

# Compile check only (no measurements)
cargo bench -p oni-comb-parser --bench comparison -- --test
cargo bench -p oni-comb-parser --bench json_full -- --test

# Heap allocation measurement
cargo bench -p oni-comb-parser --bench alloc_count
```

## Benchmark Targets

| Target | Focus | Notes |
|--------|-------|-------|
| `comparison` | Token microbenchmarks, JSON subset, arithmetic | Supports Criterion filters such as `identifier`, `integer`, `flat_map`, `zip_vs`, `json`, `arithmetic` |
| `json_full` | Full parse ranking on 107KB `sample.json` | Separate harness for macro throughput on a realistic JSON payload |
| `alloc_count` | Heap allocation counts with `dhat-rs` | Measures both token workloads and the full JSON parse |

## Included Results

| Result Section | Source target |
|----------------|---------------|
| Token workloads (`identifier`, `integer`, `flat_map`, `zip_vs_flat_map`) | `comparison` |
| JSON subset and arithmetic | `comparison` |
| Full JSON benchmark on 107KB `sample.json` | `json_full` |
| Heap allocation measurement | `alloc_count` |

## Benchmark Groups (`comparison` target)

| Group | Description | Libraries |
|-------|-------------|-----------|
| `token/identifier` | Identifier parsing (`satisfy` + `take_while0`) | 5 libraries |
| `token/integer` | Integer parsing (`take_while1` + parse) | 5 libraries |
| `token/flat_map_same_type` | flat_map same-type branch (digit → tag) | 5 libraries |
| `token/flat_map_boxed` | flat_map heterogeneous branch (`Box<dyn Parser>` etc.) | 5 libraries |
| `token/zip_vs_flat_map` | Direct comparison of zip and flat_map | oni-comb only |
| `json` | JSON subset parsing (null/int/string/array/object) | oni-comb only |
| `arithmetic` | Arithmetic + parentheses (recursive + chainl1) | oni-comb only |

## Results and Analysis

Measurement environment:
- Mac mini (Mac16,11)
- Apple M4 Pro, 14 cores (10 Performance + 4 Efficiency)
- Memory: 64 GB
- macOS 26.3.1
- Architecture: arm64

The `comparison` tables below were rerun on March 21, 2026 on the machine above.
All figures are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint).
The full JSON section later in this document is the separate `json_full` rerun from March 21, 2026 after the latest `take_while*` hot-path cleanup.

### Token Workload — Identifier (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 14.6 ns | 15.5 ns | 14.0 ns | 16.6 ns | 67.7 ns |
| `"foo"` (3B) | 15.4 ns | 15.9 ns | 15.7 ns | 27.5 ns | 84.3 ns |
| `"foo_bar_123"` (11B) | 20.0 ns | 20.5 ns | 33.3 ns | 85.0 ns | 205.1 ns |
| `"_private"` (8B) | 19.3 ns | 20.0 ns | 24.8 ns | 58.1 ns | 145.2 ns |
| `"longIdent..."` (28B) | 33.8 ns | 33.6 ns | 83.1 ns | 133.4 ns | 272.5 ns |

### Token Workload — Integer (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"0"` | 2.3 ns | 2.1 ns | 2.0 ns | 69.0 ns | 21.1 ns |
| `"42"` | 3.1 ns | 2.8 ns | 2.6 ns | 74.3 ns | 21.7 ns |
| `"9999999"` | 6.8 ns | 6.2 ns | 5.2 ns | 133.0 ns | 29.6 ns |
| `"184467...615"` (20B) | 22.8 ns | 22.7 ns | 22.5 ns | 252.5 ns | 95.1 ns |

### flat_map Same-Type Branch (digit → tag) (mean)

| Library | "1one" | "2two" | "3three" |
|---------|--------|--------|----------|
| winnow | 2.4 ns | 2.4 ns | 2.6 ns |
| nom | 2.6 ns | 2.6 ns | 2.7 ns |
| **oni-comb** | **10.6 ns** | **10.7 ns** | **10.4 ns** |
| chumsky | 49.1 ns | 49.5 ns | 52.0 ns |
| pom | 69.9 ns | 70.1 ns | 95.2 ns |

**Effect of MS6 ParseError introduction:**
- Old (format!-based): 8.3 / 7.8 / 6.9 ns
- New (ParseError): 7.3 / 7.3 / 6.0 ns
- **~12% improvement**. Eliminating `format!` reduced error-path code generation weight.

### flat_map Heterogeneous Branch (Box\<dyn Parser\>) (mean)

| Library | "c:hello" | "i:42" |
|---------|-----------|--------|
| nom | 3.7 ns | 2.7 ns |
| winnow | 20.1 ns | 17.2 ns |
| **oni-comb** | **24.2 ns** | **21.9 ns** |
| chumsky | 25.8 ns | 18.7 ns |
| pom | 166.3 ns | 109.5 ns |

### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| "x" | 3.1 ns | 2.4 ns | flat_map faster |
| "foo" | 3.4 ns | 3.3 ns | ≈0% (within margin) |
| "foo_bar_123" | 8.4 ns | 7.9 ns | flat_map slightly faster |
| "_private" | 6.4 ns | 6.2 ns | ≈0% (within margin) |
| "longIdent..." | 18.8 ns | 18.0 ns | flat_map slightly faster |

**zip and flat_map (same type) still remain in the same envelope, but the whole pair moved upward versus the previous snapshot.** The concrete combinator type design still avoids a large structural flat_map penalty, yet both variants are currently slower than the earlier March 18 rerun.

### JSON Subset (oni-comb only) (mean)

| Input | Time | byte/ns |
|-------|------|---------|
| `null` (4B) | 16.5 ns | 0.24 |
| `42` (2B) | 89.7 ns | 0.02 |
| `"hello world"` (13B) | 138.1 ns | 0.09 |
| `[1, 2, 3]` (9B) | 505.2 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 529.1 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 661.3 ns | 0.08 |
| `{"a":1,...,"h":8}` (64B) | 1,379 ns | 0.05 |

**Observations:**
- Compared with the earlier March 21 snapshot before the generic `take_while*` hot-path cleanup, every JSON subset input shown here improved. Primitive-heavy cases recovered to `null`: ~16.5ns, `integer`: ~89.7ns, `string`: ~138.1ns.
- Object-heavy cases also recovered: `object` is now ~661ns and `object_large` ~1.38µs, which is a visible win for the shared separator / whitespace paths.
- `array_3` and `array_mixed` improved to ~505ns and ~529ns respectively, so the current subset benchmark now shows the latest cleanup paying off across the whole mini-suite.

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 159 ns |
| `1 + 2` | 249 ns |
| `1 + 2 * 3` | 282 ns |
| `(1 + 2) * 3` | 451 ns |
| `1 + 2 * (3 - 4) + 5` | 670 ns |
| `(((1 + 2) * 3) - 4) / 5` | 972 ns |
| `1 + 2 + ... + 8` | 810 ns |

**Observations:**
- A later March 21, 2026 rerun after the `recursive()` runtime refactor lowered the single-integer case from ~169ns to ~159ns by removing `Box<dyn Parser>` and steady-state `Option` checks from the recursive hot path.
- Parenthesized and nested cases also improved across the board, but each extra recursion layer still adds roughly ~190-220ns, so recursive indirection plus `whitespace0` remain material costs.
- The 8-term addition chain is now ~0.81µs, which still suggests the `chainl1` loop itself is not the main bottleneck.

### Full JSON Benchmark (107KB sample.json)

This section corresponds to `cargo bench -p oni-comb-parser --bench json_full`.

Same-machine rerun on March 21, 2026 using the same 107KB JSON file (100 samples).

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| oni-comb | 671.7 µs | 152.0 |
| **winnow** | **176.0 µs** | **580.0** |
| nom | 286.1 µs | 356.8 |
| chumsky | 493.9 µs | 206.7 |
| pom | 7,880 µs | 13.0 |

**On this rerun, `winnow` still leads the full-JSON benchmark, followed by `nom` and `chumsky`. oni-comb improves to 152.0 MiB/s after the latest `take_while*` hot-path cleanup and still beats `pom`, but the realistic 107KB payload remains slower than the top three parsers.** The latest gain mainly comes from removing per-token checkpoint/reset churn in generic scanning paths:
- Function recursion via `fn_parser` (eliminates `recursive()`'s `Box<dyn Parser>` vtable)
- Leading-byte dispatch via `peek_byte` (eliminates `or` chain linear scanning)
- Zero-copy strings via `quoted_string` (unescaped strings use `&str` slices)
- Zero-copy number parsing via `take_while1`
- ASCII fast-path token access in `StrInputStream`
- Lower-overhead generic `take_while*` loops that avoid per-token checkpoint/reset churn in whitespace and separator handling

During the later `improve-recursive-runtime` rerun on March 21, 2026, `json_full/oni-comb` measured about **685.7 µs / 148.9 MiB/s**. That recovered most of an earlier thunk-prototype regression (~707µs), but it still did not beat the 671.7µs snapshot above, so `recursive()` is only part of the full-JSON bottleneck story.

### Heap Allocation Measurement (dhat-rs)

#### Token Workload

```
dhat: Total:     0 bytes in 0 blocks
dhat: At t-gmax: 0 bytes in 0 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

Identifier / integer / flat_map same-type all show **0 blocks**.
The parser combinator infrastructure (`fn_parser`, `tag`, `char`, `whitespace0`, `take_while1`, `satisfy`, `zip`, `map`, `or`, etc.) is **completely zero-allocation**.

#### Full JSON Parse (107KB sample.json)

```
dhat: Total:     335,647 bytes in 743 blocks
dhat: At t-gmax: 218,047 bytes in 470 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

Allocation source breakdown:

| Source | Bytes | Description |
|--------|-------|-------------|
| `Vec` grow (array/object element collection) | 312,512 | `Vec::push` growth for JSON arrays `[]` / objects `{}` |
| `quoted_string` slow path | 23,135 | Only escaped strings construct `Cow::Owned(String)` |

**The parser combinator infrastructure itself has zero allocations.** All allocations come from AST construction:
- `Vec<Json>` / `Vec<(Cow, Json)>` — array/object element collection (unavoidable)
- `Cow::Owned` — only for escaped strings (unescaped strings use `Cow::Borrowed(&str)` for zero-copy)
- `fn_parser`, `tag`, `char`, `whitespace0`, `take_while1`, `quoted_string` (fast path), `peek_byte` — all zero-allocation

## Optimization Cycle Record

The tables below are historical snapshots captured during earlier optimization steps. They explain where speedups came from, but they are not directly comparable to the March 21, 2026 rerun above.

### MS6 ParseError Introduction Effect (mean)

| Workload | Old (String/format!) | New (ParseError) | Improvement |
|----------|---------------------|-------------------|-------------|
| flat_map "1one" | 8.3 ns | 7.3 ns | -12% |
| flat_map "2two" | 7.8 ns | 7.3 ns | -6% |
| flat_map "3three" | 6.9 ns | 6.0 ns | -13% |

**Analysis**: The `format!` macro's `String` allocation code was hindering LLVM optimization. `ParseError::expected_char(pos, c)` only constructs a struct without using `format!`, reducing error-path code generation and also benefiting success-path inlining.

### #[inline] Addition Effect (mean)

| Workload | Old | New | Improvement |
|----------|-----|-----|-------------|
| identifier "x" (1B) | 18.4 ns | 14.9 ns | -19% |
| identifier "foo" (3B) | 19.6 ns | 17.8 ns | -9% |
| identifier "_private" (8B) | 26.2 ns | 25.1 ns | -4% |
| flat_map "1one" | 7.3 ns | 6.1 ns | -16% |
| flat_map "2two" | 7.3 ns | 6.2 ns | -15% |
| flat_map "3three" | 6.0 ns | 4.8 ns | -20% |

**Analysis**: Added `#[inline]` to all `parse_next` implementations. Shorter inputs benefit more (15-20% improvement). Identifier "x" now matches winnow (14.9 vs 15.2 ns). Cross-crate inlining was promoted, allowing LLVM to eliminate function call overhead.

### Zero-Copy + fn Recursion + Byte Dispatch Effect (mean)

| Step | oni-comb | Throughput | Improvement |
|------|----------|-----------|-------------|
| Before (recursive + or chain) | 640 µs | 159 MB/s | — |
| + `quoted_string` zero-copy | 486 µs | 210 MB/s | -24% |
| + number zero-copy | 477 µs | 214 MB/s | -2% |
| + `fn_parser` recursion + `peek_byte` dispatch | **109 µs** | **937 MB/s** | **-77%** |

**Analysis**: The biggest impact came from `fn_parser` + `peek_byte` dispatch. `recursive()`'s `Box<dyn Parser>` vtable indirect calls were occurring tens of thousands of times across all JSON nodes, whereas `fn_parser` uses normal function calls (inlinable). `peek_byte` leading-byte dispatch also eliminated `or` chain checkpoint/reset cycles.

### Remaining Bottlenecks

1. **`recursive()` is still heavy**: after the owner/ref split + typed thunk runtime change, a single integer in the arithmetic benchmark is down to ~159ns, but it is still far from `fn_parser`-style recursion (~3ns). The remaining cost is now the shared runtime indirection plus surrounding whitespace handling, not the old `Box<dyn Parser>` vtable itself.
2. **`flat_map` is still costlier than the best parsers**: same-type branches improved to 5.7 / 5.5 / 4.1ns, but still trail winnow and nom; heterogeneous boxed branches are now near winnow, but nom remains far ahead.

### Generic InputStream Refactoring Effect (InputStream trait generification)

Introduced `Token`/`Slice` associated types to `InputStream` trait and moved `satisfy`, `take_while0/1`, `take`, `take_while_n_m`, `eof` to generic `primitive/` module. Also added `ByteInputStream<'a>` for `&[u8]` parsing.

**Impact on token-level parsers using generic primitives (`satisfy` + `take_while0`):**

| Input | Before | After | Change | Cause |
|-------|--------|-------|--------|-------|
| identifier `"x"` (1B) | 18.4 ns | 16.6 ns | -10% | Within margin |
| identifier `"foo"` (3B) | 19.6 ns | 21.1 ns | +8% | Per-token overhead |
| identifier `"foo_bar_123"` (11B) | 28.1 ns | 38.9 ns | +38% | Per-token overhead |
| identifier `"_private"` (8B) | 26.2 ns | 42.2 ns | +61% | Per-token overhead |
| identifier `"longIdent..."` (28B) | 44.4 ns | 81.7 ns | +84% | Per-token overhead |
| integer `"42"` (2B) | 3.6 ns | 8.8 ns | +144% | Per-token overhead |
| integer `"9999999"` (7B) | 8.2 ns | 20.3 ns | +148% | Per-token overhead |

**Root cause**: The old `text/` implementations iterated `remaining.chars()` once and called `advance(consumed)` at the end. The generic `primitive/` implementations call `peek_token()` + `next_token()` per token, each of which recomputes `&self.src[self.offset..]` and calls `.chars().next()`. This is the cost of genericity — the `InputStream` trait cannot expose a batch character iterator.

**Later recovery (March 21, 2026 hot-path pass)**: the generic path removed per-token checkpoint/reset churn from `take_while*` by gating consumption with `peek_token()` before `next_token()`. The JSON subset measurements above reflect that recovery immediately, even though the token tables in this document were not rerun as part of this pass.

**Unaffected workloads** (use `as_str().chars()` directly or `fn_parser`):

| Workload | Before | After | Change |
|----------|--------|-------|--------|
| JSON `null` | 15.0 ns | 8.5 ns | -43% (noise/cache) |
| JSON `object_large` | 1,492 ns | 1,427 ns | -4% |
| arithmetic `single` | 156 ns | 155 ns | ≈0% |
| arithmetic `complex` | 639 ns | 628 ns | -2% |
| flat_map same-type `"1one"` | 7.3 ns | 7.2 ns | ≈0% |

**Mitigation**: Text-specific parsers (`identifier`, `integer`, `tag`, `whitespace`, `quoted_string`) remain in `text/` with direct `as_str().chars()` access, preserving their performance. Only code using the generic `primitive::satisfy`/`primitive::take_while0` from the prelude is affected.

**zip ≒ flat_map still holds** (post-generification):

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| "x" | 2.3 ns | 1.9 ns | same envelope |
| "foo" | 2.7 ns | 2.5 ns | same envelope |
| "foo_bar_123" | 6.7 ns | 6.6 ns | same envelope |
| "_private" | 5.3 ns | 6.3 ns | same envelope |
| "longIdent..." | 15.6 ns | 15.4 ns | same envelope |

## Overall Assessment

- **`winnow` still leads the macro benchmark** — 580.0 MiB/s on the 107KB JSON rerun, with `nom` at 356.8 MiB/s and `chumsky` at 206.7 MiB/s; oni-comb now reaches 152.0 MiB/s
- **The latest `take_while*` hot-path cleanup recovered both subset JSON and full JSON** — `null` fell to 16.5ns, `object_large` to ~1.38µs, and the 107KB full-JSON run improved to 671.7µs
- **Token-level generic parsers remain competitive, but the latest rerun is less flattering than the previous snapshot** — identifier 11B: oni-comb 20.0ns vs winnow 20.5ns / nom 33.3ns; integer 20B: oni-comb 22.8ns vs winnow 22.7ns / nom 22.5ns
- **chumsky 0.12 remains dramatically better than older releases** — short identifiers are still in the same ballpark as oni-comb (`"x"`: 16.6ns vs 14.6ns), but medium/long inputs still trail substantially (`"foo_bar_123"`: 85.0ns vs 20.0ns)
- **flat_map still has a measurable gap to the best parsers** — especially same-type branch dispatch (`"1one"`: oni-comb 10.6ns vs winnow 2.4ns / nom 2.6ns)
- **zip and flat_map stay in the same envelope** — validates the concrete combinator type design without a structural flat_map penalty
- **Zero heap allocation for Applicative combinators**
