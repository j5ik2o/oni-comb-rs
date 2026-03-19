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

Measured on March 18, 2026 on the machine above, after refreshing the benchmark baseline to `winnow` 1.0.0.
All figures are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint).

### Token Workload — Identifier (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 15.0 ns | 15.2 ns | 15.1 ns | 17.5 ns | 68.2 ns |
| `"foo"` (3B) | 14.7 ns | 16.2 ns | 16.5 ns | 29.4 ns | 86.5 ns |
| `"foo_bar_123"` (11B) | 18.6 ns | 20.3 ns | 33.2 ns | 86.9 ns | 202.0 ns |
| `"_private"` (8B) | 18.2 ns | 19.7 ns | 29.3 ns | 60.1 ns | 146.4 ns |
| `"longIdent..."` (28B) | 30.2 ns | 33.2 ns | 86.5 ns | 132.7 ns | 269.7 ns |

### Token Workload — Integer (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"0"` | 2.2 ns | 2.3 ns | 1.9 ns | 72.0 ns | 20.8 ns |
| `"42"` | 2.6 ns | 2.8 ns | 2.8 ns | 70.8 ns | 20.7 ns |
| `"9999999"` | 5.2 ns | 5.4 ns | 5.4 ns | 132.3 ns | 29.5 ns |
| `"184467...615"` (20B) | 20.0 ns | 23.1 ns | 22.7 ns | 273.1 ns | 100.1 ns |

### flat_map Same-Type Branch (digit → tag) (mean)

| Library | "1one" | "2two" | "3three" |
|---------|--------|--------|----------|
| winnow | 2.6 ns | 2.6 ns | 2.7 ns |
| nom | 3.2 ns | 2.7 ns | 2.4 ns |
| **oni-comb** | **5.7 ns** | **5.5 ns** | **4.1 ns** |
| chumsky | 72.8 ns | 51.9 ns | 51.5 ns |
| pom | 76.2 ns | 69.0 ns | 95.0 ns |

**Effect of MS6 ParseError introduction:**
- Old (format!-based): 8.3 / 7.8 / 6.9 ns
- New (ParseError): 7.3 / 7.3 / 6.0 ns
- **~12% improvement**. Eliminating `format!` reduced error-path code generation weight.

### flat_map Heterogeneous Branch (Box\<dyn Parser\>) (mean)

| Library | "c:hello" | "i:42" |
|---------|-----------|--------|
| nom | 4.6 ns | 2.8 ns |
| winnow | 19.2 ns | 17.7 ns |
| **oni-comb** | **20.7 ns** | **18.3 ns** |
| chumsky | 41.4 ns | 24.2 ns |
| pom | 307.5 ns | 114.2 ns |

### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| "x" | 2.3 ns | 1.9 ns | flat_map slightly faster |
| "foo" | 2.7 ns | 2.5 ns | flat_map slightly faster |
| "foo_bar_123" | 6.7 ns | 6.6 ns | ≈0% (within margin) |
| "_private" | 5.3 ns | 6.3 ns | zip slightly faster |
| "longIdent..." | 15.6 ns | 15.4 ns | ≈0% (within margin) |

**zip and flat_map (same type) remain in the same low-single-digit / low-teen nanosecond envelope.** The concrete combinator type design still avoids any structural flat_map penalty.

### JSON Subset (oni-comb only) (mean)

| Input | Time | byte/ns |
|-------|------|---------|
| `null` (4B) | 11.5 ns | 0.35 |
| `42` (2B) | 88.7 ns | 0.02 |
| `"hello world"` (13B) | 129.1 ns | 0.10 |
| `[1, 2, 3]` (9B) | 494.3 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 499.4 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 596.5 ns | 0.08 |
| `{"a":1,...,"h":8}` (64B) | 1,259 ns | 0.05 |

**Observations:**
- The whitespace-boundary refactor is a mixed result on tiny subset inputs. Primitive-heavy cases regressed (`null`: ~11.5ns, `integer`: ~88.7ns, `string`: ~129.1ns), which suggests the new helper structure adds enough combinator overhead to matter at this scale.
- Object-heavy cases improved (`object`: ~596.5ns, `object_large`: ~1.26µs), which suggests reducing repeated delimiter/member whitespace scans helps once member boundaries dominate the work.
- `array_mixed` is effectively flat while `array_3` regressed slightly. On the subset benchmark, the whitespace cleanup does not uniformly win, but it does shift cost away from object parsing hot paths.

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 151 ns |
| `1 + 2` | 240 ns |
| `1 + 2 * 3` | 265 ns |
| `(1 + 2) * 3` | 429 ns |
| `1 + 2 * (3 - 4) + 5` | 618 ns |
| `(((1 + 2) * 3) - 4) / 5` | 912 ns |
| `1 + 2 + ... + 8` | 762 ns |

**Observations:**
- ~151ns for a single integer is still quite heavy. This is due to `recursive()`'s indirect calls via `Rc<UnsafeCell<Box<dyn Parser>>>` + `whitespace0` overhead.
- Each level of parenthesis nesting still adds roughly ~180-200ns, consistent with one additional `Box<dyn Parser>` recursion stage.
- The 8-term addition chain remains ~0.76µs, which suggests the `chainl1` loop itself is not the main bottleneck.

### Full JSON Benchmark (107KB sample.json)

This section corresponds to `cargo bench -p oni-comb-parser --bench json_full`.

Same-machine rerun on March 18, 2026 using the same 107KB JSON file (100 samples), after updating the benchmark dependency from `winnow` 0.7 to 1.0.0.

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| **oni-comb** | **109.5 µs** | **932.1** |
| winnow | 178.7 µs | 571.3 |
| nom | 282.8 µs | 360.9 |
| chumsky | 561.0 µs | 181.9 |
| pom | 7.69 ms | 13.3 |

**On this rerun, oni-comb extends its full-JSON lead further. It reaches 1.63x the throughput of `winnow` 1.0.0, 2.58x that of nom, 5.12x that of chumsky, and 70.2x that of pom (mean basis).** Even though the subset benchmark is mixed, the realistic 107KB payload benefits from the whitespace-boundary cleanup, which suggests the reduced repeated scans matter more on object-heavy real inputs than on tiny primitive cases. The current ranking still reflects the same oni-comb design wins:
- Function recursion via `fn_parser` (eliminates `recursive()`'s `Box<dyn Parser>` vtable)
- Leading-byte dispatch via `peek_byte` (eliminates `or` chain linear scanning)
- Zero-copy strings via `quoted_string` (unescaped strings use `&str` slices)
- Zero-copy number parsing via `take_while1`
- ASCII fast-path token access in `StrInputStream`

- Consume-then-reset generic primitives (`satisfy` / `take_while*` / `one_of` / `none_of`) that avoid `peek_token()` + `next_token()` double decoding

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

The tables below are historical snapshots captured during earlier optimization steps. They explain where speedups came from, but they are not directly comparable to the March 18, 2026 rerun above.

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

1. **`recursive()` is still heavy**: A single integer in the arithmetic benchmark takes ~151ns (`fn_parser` would be ~3ns). The vtable cost remains for cases where `recursive()` is needed (when `fn` recursion isn't structurally feasible).
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

**Later recovery (March 18, 2026 fast-path pass)**: the generic path now avoids most of that regression for ASCII-heavy inputs by adding an ASCII fast path to `StrInputStream` and switching generic primitives from `peek_token()` + `next_token()` to `next_token()` + `reset()` on mismatch. Current means are now much closer to the old text-specific implementations: identifier `"foo_bar_123"` is 18.6ns and integer `"184467...615"` is 20.0ns.

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

- **oni-comb now leads the macro benchmark by a wider margin** — 932.1 MiB/s on the 107KB JSON rerun, ahead of winnow's 571.3 MiB/s
- **oni-comb is comfortably ahead of nom / chumsky / pom on full JSON** — 2.58x faster than nom, 5.12x faster than chumsky, and 70.2x faster than pom
- **Token-level generic parsers are no longer the glaring weak spot they were** — identifier 11B: oni-comb 18.6ns vs winnow 20.3ns / nom 33.2ns; integer 20B: oni-comb 20.0ns vs winnow 23.1ns / nom 22.7ns
- **chumsky 0.12 remains dramatically better than older releases** — short identifiers are still in the same ballpark as oni-comb (`"x"`: 17.5ns vs 15.0ns), but medium/long inputs still trail substantially (`"foo_bar_123"`: 86.9ns vs 18.6ns)
- **flat_map still has a measurable gap to the best parsers** — especially same-type branch dispatch (`"1one"`: oni-comb 5.7ns vs winnow 2.6ns / nom 3.2ns)
- **zip and flat_map stay in the same envelope** — validates the concrete combinator type design without a structural flat_map penalty
- **The whitespace refactor is mixed on JSON subset** — primitive-heavy cases regressed, but object-heavy cases improved (`object_large`: ~1.26µs) and the full JSON macro benchmark still moved in the right direction
- **Zero heap allocation for Applicative combinators**
