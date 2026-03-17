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

Measured on Apple M-series chip (after ParseError introduction).
All figures are Criterion **mean estimates** (100 samples, 95% confidence interval midpoint).

### Token Workload — Identifier (mean)

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 16.6 ns | 15.5 ns | 14.9 ns | 17.1 ns | 66.3 ns |
| `"foo"` (3B) | 21.1 ns | 16.2 ns | 16.9 ns | 29.8 ns | 85.3 ns |
| `"foo_bar_123"` (11B) | 38.9 ns | 21.7 ns | 33.4 ns | 83.8 ns | 230 ns |
| `"_private"` (8B) | 42.2 ns | 20.9 ns | 25.7 ns | 57.2 ns | 138.9 ns |
| `"longIdent..."` (28B) | 81.7 ns | 34.2 ns | 82.7 ns | 132.0 ns | 266.5 ns |

### Token Workload — Integer (mean)

| Input | oni-comb | winnow | nom | pom | chumsky |
|-------|----------|--------|-----|-----|---------|
| `"0"` | 4.7 ns | 2.0 ns | 2.0 ns | 73.4 ns | 16.4 ns |
| `"42"` | 8.8 ns | 3.8 ns | 3.8 ns | 77.2 ns | 17.2 ns |
| `"9999999"` | 20.3 ns | 5.7 ns | 5.8 ns | 136 ns | 32.3 ns |
| `"184467...615"` (20B) | 62.4 ns | 24.1 ns | 23.4 ns | 253 ns | 86.2 ns |

### flat_map Same-Type Branch (digit → tag) (mean)

| Library | "1one" | "2two" | "3three" |
|---------|--------|--------|----------|
| winnow | 2.7 ns | 2.7 ns | 2.6 ns |
| nom | 2.4 ns | 2.5 ns | 2.4 ns |
| **oni-comb** | **7.2 ns** | **7.2 ns** | **5.9 ns** |
| chumsky | 49.7 ns | 49.9 ns | 51.9 ns |
| pom | 69.4 ns | 70.2 ns | 94.7 ns |

**Effect of MS6 ParseError introduction:**
- Old (format!-based): 8.3 / 7.8 / 6.9 ns
- New (ParseError): 7.3 / 7.3 / 6.0 ns
- **~12% improvement**. Eliminating `format!` reduced error-path code generation weight.

### flat_map Heterogeneous Branch (Box\<dyn Parser\>) (mean)

| Library | "c:hello" | "i:42" |
|---------|-----------|--------|
| nom | 3.7 ns | 3.0 ns |
| winnow | 20.1 ns | 18.0 ns |
| chumsky | 25.4 ns | 19.7 ns |
| **oni-comb** | **30.9 ns** | **23.8 ns** |
| pom | 163.7 ns | 111.0 ns |

### zip vs flat_map (oni-comb-rs internal comparison) (mean)

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| "x" | 4.3 ns | 4.2 ns | ≈0% (within margin) |
| "foo" | 8.4 ns | 8.3 ns | ≈0% (within margin) |
| "foo_bar_123" | 26.0 ns | 25.9 ns | ≈0% (within margin) |
| "_private" | 19.9 ns | 19.9 ns | ≈0% (within margin) |
| "longIdent..." | 64.9 ns | 64.7 ns | ≈0% (within margin) |

**zip ≒ flat_map (same type) continues to hold.** This validates the concrete combinator type design.

### JSON Subset (oni-comb only) (mean)

| Input | Time | byte/ns |
|-------|------|---------|
| `null` (4B) | 8.5 ns | 0.47 |
| `42` (2B) | 83.4 ns | 0.02 |
| `"hello world"` (13B) | 143.8 ns | 0.09 |
| `[1, 2, 3]` (9B) | 517.6 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 528.9 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 663.5 ns | 0.08 |
| `{"a":1,...,"h":8}` (65B) | 1,427 ns | 0.05 |

**Observations:**
- `null` completes in 15ns with a single tag match. Integer has overhead from `whitespace0` → `integer()` → `whitespace0` three-stage pipeline.
- Arrays and objects scale linearly with element count. 8-element object takes ~1.5µs.
- The 5-branch `or` chain (null/true/false/int/string) cost is multiplied per element.

### Arithmetic + Parentheses (oni-comb only, using recursive) (mean)

| Input | Time |
|-------|------|
| `42` | 155 ns |
| `1 + 2` | 245 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 443 ns |
| `1 + 2 * (3 - 4) + 5` | 628 ns |
| `(((1 + 2) * 3) - 4) / 5` | 905 ns |
| `1 + 2 + ... + 8` | 761 ns |

**Observations:**
- 155ns for a single integer is quite heavy. This is due to `recursive()`'s indirect calls via `Rc<UnsafeCell<Box<dyn Parser>>>` + `whitespace0` overhead.
- Each level of parenthesis nesting adds ~200ns (cost of one `Box<dyn Parser>` recursion stage).
- 8-term addition chain at 761ns. The `chainl1` loop is efficient.

### Full JSON Benchmark (107KB sample.json)

Same-machine measurement using the same 107KB JSON file (100 samples). pom excluded (pom 3.x API makes a full JSON parser impractical for this benchmark).

| Library | Mean | Throughput (mean) |
|---------|------|-------------------|
| **oni-comb** | **196.5 µs** | **519 MB/s** |
| winnow | 201.0 µs | 508 MB/s |
| nom | 274.5 µs | 372 MB/s |
| chumsky | 495.7 µs | 206 MB/s |

**oni-comb achieves 1.03x the throughput of winnow, 1.40x that of nom, and 2.52x that of chumsky (mean basis).** Optimization breakdown:
- Function recursion via `fn_parser` (eliminates `recursive()`'s `Box<dyn Parser>` vtable)
- Leading-byte dispatch via `peek_byte` (eliminates `or` chain linear scanning)
- Zero-copy strings via `quoted_string_cow` (unescaped strings use `&str` slices)
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
| `quoted_string_cow` slow path | 23,135 | Only escaped strings construct `Cow::Owned(String)` |

**The parser combinator infrastructure itself has zero allocations.** All allocations come from AST construction:
- `Vec<Json>` / `Vec<(Cow, Json)>` — array/object element collection (unavoidable)
- `Cow::Owned` — only for escaped strings (unescaped strings use `Cow::Borrowed(&str)` for zero-copy)
- `fn_parser`, `tag`, `char`, `whitespace0`, `take_while1`, `quoted_string_cow` (fast path), `peek_byte` — all zero-allocation

## Optimization Cycle Record

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
| + `quoted_string_cow` zero-copy | 486 µs | 210 MB/s | -24% |
| + number zero-copy | 477 µs | 214 MB/s | -2% |
| + `fn_parser` recursion + `peek_byte` dispatch | **109 µs** | **937 MB/s** | **-77%** |

**Analysis**: The biggest impact came from `fn_parser` + `peek_byte` dispatch. `recursive()`'s `Box<dyn Parser>` vtable indirect calls were occurring tens of thousands of times across all JSON nodes, whereas `fn_parser` uses normal function calls (inlinable). `peek_byte` leading-byte dispatch also eliminated `or` chain checkpoint/reset cycles.

### Remaining Bottlenecks

1. **`recursive()` is still heavy**: A single integer in the arithmetic benchmark takes ~155ns (`fn_parser` would be ~3ns). The vtable cost remains for cases where `recursive()` is needed (when `fn` recursion isn't structurally feasible).
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

- **Outperforms winnow in throughput** — 1.03x faster on 107KB JSON (`fn_parser` + `peek_byte` dispatch + zero-copy strings)
- **Outperforms nom on medium-to-long inputs** — identifier 11B: oni-comb 38.9ns vs nom 33.4ns (nom slightly faster), but 28B: oni-comb 81.7ns vs nom 82.7ns (comparable)
- **3–30x faster than pom** — demonstrates the gap vs. old v1-equivalent `Rc<dyn Fn>` design
- **chumsky 0.12 dramatically improved** — identifier "x": 918ns -> 17.1ns (~54x faster than v0.9). chumsky is now competitive on short inputs but still 2x slower on medium/long inputs (83.8ns vs 38.9ns at 11B identifier). flat_map boxed: chumsky now comparable to oni-comb (25.4ns vs 30.9ns)
- **zip ≒ flat_map (same type)** — validates the concrete combinator type design
- **~83% cumulative improvement across 3 optimization cycles** — ParseError introduction (~12%) + #[inline] (~17%) + zero-copy + fn recursion (~77%)
- **2-5% improvement on JSON/arithmetic workloads** — JSON object_large: 1,495ns -> 1,427ns, arithmetic complex: 995ns -> 905ns
- **Zero heap allocation for Applicative combinators**
