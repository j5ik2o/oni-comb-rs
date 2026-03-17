# Benchmarks

[日本語](README.ja.md)

Performance comparison of oni-comb-rs v2 against other libraries (winnow, nom, chumsky, pom).

## How to Run

```bash
# Run all benchmarks
cargo bench -p oni-comb-parser --bench comparison

# Run specific groups
cargo bench -p oni-comb-parser --bench comparison -- identifier
cargo bench -p oni-comb-parser --bench comparison -- integer
cargo bench -p oni-comb-parser --bench comparison -- flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# Compile check only (no measurements)
cargo bench -p oni-comb-parser --bench comparison -- --test

# Heap allocation measurement
cargo bench -p oni-comb-parser --bench alloc_count
```

## Benchmark Groups

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
| `"x"` (1B) | 17.7 ns | 16.9 ns | 15.6 ns | 17.8 ns | 67.2 ns |
| `"foo"` (3B) | 21.7 ns | 15.7 ns | 16.2 ns | 27.8 ns | 83.7 ns |
| `"foo_bar_123"` (11B) | 39.2 ns | 19.8 ns | 32.7 ns | 84.7 ns | 203.5 ns |
| `"_private"` (8B) | 33.8 ns | 19.7 ns | 24.3 ns | 56.0 ns | 140.5 ns |
| `"longIdent..."` (28B) | 80.1 ns | 33.3 ns | 81.4 ns | 130.8 ns | 263.5 ns |

### Token Workload — Integer (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"0"` | 4.3 ns | 2.0 ns | 1.8 ns | 69.9 ns | 20.7 ns |
| `"42"` | 6.9 ns | 2.7 ns | 2.5 ns | 72.6 ns | 20.9 ns |
| `"9999999"` | 19.3 ns | 5.2 ns | 5.1 ns | 131.5 ns | 28.8 ns |
| `"184467...615"` (20B) | 59.2 ns | 22.3 ns | 21.9 ns | 256.4 ns | 94.0 ns |

### flat_map Same-Type Branch (digit → tag) (mean)

| Library | "1one" | "2two" | "3three" |
|---------|--------|--------|----------|
| winnow | 2.4 ns | 2.4 ns | 2.7 ns |
| nom | 2.4 ns | 2.4 ns | 2.3 ns |
| **oni-comb** | **6.8 ns** | **6.8 ns** | **5.5 ns** |
| chumsky | 48.4 ns | 48.8 ns | 51.6 ns |
| pom | 69.9 ns | 69.6 ns | 94.4 ns |

**Effect of MS6 ParseError introduction:**
- Old (format!-based): 8.3 / 7.8 / 6.9 ns
- New (ParseError): 7.3 / 7.3 / 6.0 ns
- **~12% improvement**. Eliminating `format!` reduced error-path code generation weight.

### flat_map Heterogeneous Branch (Box\<dyn Parser\>) (mean)

| Library | "c:hello" | "i:42" |
|---------|-----------|--------|
| nom | 3.6 ns | 2.7 ns |
| winnow | 18.8 ns | 17.6 ns |
| chumsky | 25.7 ns | 18.8 ns |
| **oni-comb** | **30.5 ns** | **23.3 ns** |
| pom | 161.8 ns | 110.5 ns |

### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| "x" | 3.8 ns | 3.8 ns | ≈0% (within margin) |
| "foo" | 8.0 ns | 8.0 ns | ≈0% (within margin) |
| "foo_bar_123" | 25.2 ns | 25.3 ns | ≈0% (within margin) |
| "_private" | 19.3 ns | 19.3 ns | ≈0% (within margin) |
| "longIdent..." | 62.9 ns | 62.8 ns | ≈0% (within margin) |

**zip ≒ flat_map (same type) continues to hold.** This validates the concrete combinator type design.

### JSON Subset (oni-comb only) (mean)

| Input | Time | byte/ns |
|-------|------|---------|
| `null` (4B) | 8.4 ns | 0.48 |
| `42` (2B) | 77.5 ns | 0.03 |
| `"hello world"` (13B) | 115.4 ns | 0.11 |
| `[1, 2, 3]` (9B) | 484.4 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 499.9 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 625.6 ns | 0.08 |
| `{"a":1,...,"h":8}` (64B) | 1,322 ns | 0.05 |

**Observations:**
- `null` completes in ~8.4ns with a single tag match. `integer` is still costlier because it goes through `whitespace0` → `integer()` → `whitespace0`.
- Arrays and objects continue to scale roughly linearly with element count. The 8-field object is now ~1.32µs.
- The fixed cost is still dominated by branch dispatch and whitespace handling; the smaller string / object figures in this rerun suggest codegen and cache effects matter, but they do not change that shape.

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

Same-machine rerun on March 18, 2026 using the same 107KB JSON file (100 samples), after updating the benchmark dependency from `winnow` 0.7 to 1.0.0.

| Library | Mean | Throughput (mean, MiB/s) |
|---------|------|-------------------------|
| oni-comb | 203.7 µs | 501.1 |
| **winnow** | **180.7 µs** | **564.8** |
| nom | 260.5 µs | 391.8 |
| chumsky | 490.0 µs | 208.3 |
| pom | 7.33 ms | 13.9 |

**On this rerun, `winnow` 1.0.0 leads the full-JSON benchmark. oni-comb still achieves 1.28x the throughput of nom, 2.41x that of chumsky, and 36.0x that of pom, while reaching 0.89x of winnow's throughput (mean basis).** The current ranking still reflects the same oni-comb design wins:
- Function recursion via `fn_parser` (eliminates `recursive()`'s `Box<dyn Parser>` vtable)
- Leading-byte dispatch via `peek_byte` (eliminates `or` chain linear scanning)
- Zero-copy strings via `quoted_string` (unescaped strings use `&str` slices)
- Zero-copy number parsing via `take_while1`

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
2. **`whitespace0` call frequency**: The JSON parser calls `whitespace0()` multiple times before and after values, with room for consolidation.

### Generic Input Refactoring Effect (Input trait generification)

Introduced `Token`/`Slice` associated types to `Input` trait and moved `satisfy`, `take_while0/1`, `take`, `take_while_n_m`, `eof` to generic `primitive/` module. Also added `ByteInput<'a>` for `&[u8]` parsing.

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

**Root cause**: The old `text/` implementations iterated `remaining.chars()` once and called `advance(consumed)` at the end. The generic `primitive/` implementations call `peek_token()` + `next_token()` per token, each of which recomputes `&self.src[self.offset..]` and calls `.chars().next()`. This is the cost of genericity — the `Input` trait cannot expose a batch character iterator.

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
| "x" | 4.3 ns | 4.2 ns | ≈0% |
| "foo" | 8.4 ns | 8.3 ns | ≈0% |
| "foo_bar_123" | 26.0 ns | 25.9 ns | ≈0% |
| "_private" | 19.9 ns | 19.9 ns | ≈0% |
| "longIdent..." | 64.9 ns | 64.7 ns | ≈0% |

## Overall Assessment

- **`winnow` 1.0.0 now leads the macro benchmark** — 564.8 MiB/s on the 107KB JSON rerun, ahead of oni-comb's 501.1 MiB/s
- **oni-comb still stays ahead of nom / chumsky / pom on full JSON** — 1.28x faster than nom, 2.41x faster than chumsky, and 36.0x faster than pom
- **Token-level parsers remain a weak spot versus winnow and nom** — identifier 11B: oni-comb 39.2ns vs winnow 19.8ns / nom 32.7ns; integer 20B: oni-comb 59.2ns vs winnow 22.3ns / nom 21.9ns
- **chumsky 0.12 remains dramatically better than older releases** — short identifiers are now near oni-comb (`"x"`: 17.8ns vs 17.7ns), but medium/long inputs still trail substantially (`"foo_bar_123"`: 84.7ns vs 39.2ns)
- **zip ≒ flat_map (same type)** — validates the concrete combinator type design
- **The current rerun keeps JSON subset / arithmetic stable** — JSON `object_large` is ~1.32µs and arithmetic `complex` is ~618ns
- **Zero heap allocation for Applicative combinators**
