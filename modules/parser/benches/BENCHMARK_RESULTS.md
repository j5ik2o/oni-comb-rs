# Benchmark Results (2026-03-17)

## Environment

| Item | Value |
|------|-------|
| Date | 2026-03-17 |
| CPU | AMD EPYC 7763 64-Core Processor |
| OS | Linux 6.14.0-1017-azure (Ubuntu) |
| Memory | 16 GB |
| Rust | rustc 1.96.0-nightly (1e2183119 2026-03-15) |
| oni-comb-parser | v2.1.0 |
| Criterion | 0.8 (100 samples, 95% CI) |

## Summary

All benchmarks: `comparison` (token/identifier, token/integer, flat_map, zip_vs_flat_map, json subset, arithmetic), `json_full` (107KB sample.json), `alloc_count` (heap allocation).

---

## 1. Token — Identifier (mean)

5-library comparison. `satisfy` + `take_while0` composition.

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 24.6 ns | 25.0 ns | 22.6 ns | 28.9 ns | 102 ns |
| `"foo"` (3B) | 40.6 ns | 26.0 ns | 25.0 ns | 46.5 ns | 117 ns |
| `"foo_bar_123"` (11B) | 102 ns | 33.6 ns | 35.4 ns | 175 ns | 280 ns |
| `"_private"` (8B) | 78.7 ns | 30.8 ns | 31.7 ns | 95.7 ns | 211 ns |
| `"longIdent..."` (28B) | 191 ns | 50.5 ns | 57.2 ns | 242 ns | 403 ns |

**Observations:**
- 1B input では oni-comb ≒ winnow ≒ nom（22–25 ns の範囲内）
- 入力長が伸びると generic `primitive::satisfy`/`take_while0` の per-token オーバーヘッドにより winnow/nom との差が開く
- pom の 3–7 倍高速、chumsky の 1.2–1.7 倍高速

---

## 2. Token — Integer (mean)

`take_while1` + `str::parse` composition.

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"0"` (1B) | 10.9 ns | 4.1 ns | 3.9 ns | 24.0 ns | 107 ns |
| `"42"` (2B) | 16.6 ns | 5.2 ns | 5.6 ns | 27.7 ns | 109 ns |
| `"9999999"` (7B) | 45.5 ns | 13.2 ns | 13.0 ns | 44.9 ns | 197 ns |
| `"184467...615"` (20B) | 136 ns | 36.1 ns | 35.2 ns | 133 ns | 395 ns |

**Observations:**
- winnow/nom が一貫して最速（バイト列直接走査）
- oni-comb は 20B で chumsky と同等（136 vs 133 ns）
- pom の 4–10 倍高速

---

## 3. flat_map Same-Type Branch (mean)

`satisfy(digit).flat_map(|c| match c { '1' => tag("one"), ... })` — 分岐後に同一型を返すケース。

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"1one"` | 10.9 ns | 4.7 ns | 5.0 ns | 69.8 ns | 104 ns |
| `"2two"` | 10.9 ns | 4.7 ns | 5.0 ns | 69.5 ns | 96.3 ns |
| `"3three"` | 10.9 ns | 4.7 ns | 5.3 ns | 74.4 ns | 139 ns |

**Observations:**
- oni-comb は 3 入力で安定して ~10.9 ns（分岐による差なし）
- winnow/nom が ~2 倍高速
- chumsky の 6 倍、pom の 10–13 倍高速

---

## 4. flat_map Heterogeneous Branch (Box\<dyn Parser\>) (mean)

分岐後に異なる型を返すケース。oni-comb は `Box<dyn Parser>` を使用。

| Input | oni-comb | winnow | nom | chumsky | pom |
|-------|----------|--------|-----|---------|-----|
| `"c:hello"` | 64.2 ns | 26.5 ns | 10.6 ns | 43.3 ns | 230 ns |
| `"i:42"` | 44.0 ns | 24.3 ns | 9.0 ns | 36.7 ns | 149 ns |

**Observations:**
- Box\<dyn Parser\> の vtable 間接呼び出しオーバーヘッドにより、nom（enum dispatch）の 5–7 倍遅い
- chumsky よりも 1.2–1.5 倍遅い（chumsky も動的ディスパッチだが最適化されている）
- pom の 3–4 倍高速

---

## 5. zip vs flat_map (oni-comb 内部比較) (mean)

同一型を返す `zip` と `flat_map` の比較。具象コンビネータ型設計の検証。

| Input | zip | flat_map | Diff |
|-------|-----|----------|------|
| `"x"` (1B) | 11.4 ns | 9.5 ns | flat_map 17% faster |
| `"foo"` (3B) | 23.7 ns | 20.5 ns | flat_map 13% faster |
| `"foo_bar_123"` (11B) | 68.2 ns | 82.2 ns | zip 17% faster |
| `"_private"` (8B) | 55.6 ns | 49.6 ns | flat_map 11% faster |
| `"longIdent..."` (28B) | 192 ns | 147 ns | flat_map 24% faster |

**Observations:**
- 短い入力では flat_map がやや高速、長い入力では結果が分かれる
- いずれも同一オーダー内の差であり、具象コンビネータ型設計の有効性を確認

---

## 6. JSON Subset (oni-comb only) (mean)

JSON 値パーサーの単体ベンチマーク。

| Input | Time |
|-------|------|
| `null` (4B) | 26.1 ns |
| `42` (integer) | 145 ns |
| `"hello world"` (string) | 248 ns |
| `[1, 2, 3]` (array_3) | 889 ns |
| `[1, "two", true, null]` (array_mixed) | 926 ns |
| `{"name":"oni-comb",...}` (object) | 1.19 µs |
| `{"a":1,...,"h":8}` (object_large) | 2.59 µs |

**Observations:**
- `null` は `peek_byte` + `tag` のみで 26 ns
- 配列・オブジェクトは要素数に線形スケール
- 8 要素オブジェクト（object_large）は ~2.6 µs

---

## 7. Arithmetic + Parentheses (oni-comb only, recursive + chainl1) (mean)

| Input | Time |
|-------|------|
| `42` (single) | 309 ns |
| `1 + 2` (add) | 450 ns |
| `1 + 2 * 3` (mul_add) | 505 ns |
| `(1 + 2) * 3` (parens) | 753 ns |
| `1 + 2 * (3 - 4) + 5` (complex) | 1.09 µs |
| `(((1 + 2) * 3) - 4) / 5` (deeply_nested) | 1.50 µs |
| `1 + 2 + ... + 8` (long_chain) | 1.34 µs |

**Observations:**
- 単一整数で 309 ns は `recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` オーバーヘッドに起因
- 括弧ネスト 1 段あたり ~250 ns 追加
- 8 項加算チェーン 1.34 µs — `chainl1` ループは効率的

---

## 8. Full JSON Benchmark (107KB sample.json)

3 ライブラリ比較。Criterion 100 samples。ファイルサイズ: 107,033 bytes。

| Library | Mean | Throughput (MiB/s) |
|---------|------|--------------------|
| **winnow** | **354.5 µs** | **287.9 MiB/s** |
| oni-comb | 463.8 µs | 220.1 MiB/s |
| nom | 618.4 µs | 165.1 MiB/s |

**Detailed Statistics:**

| Library | Low | Mean | High | Throughput (mean) |
|---------|-----|------|------|-------------------|
| oni-comb | 461.4 µs | 463.8 µs | 466.2 µs | 220.1 MiB/s |
| winnow | 354.1 µs | 354.5 µs | 355.0 µs | 287.9 MiB/s |
| nom | 616.6 µs | 618.4 µs | 620.6 µs | 165.1 MiB/s |

**Observations:**
- winnow が最速（288 MiB/s）、oni-comb は 1.31 倍遅い（220 MiB/s）
- oni-comb は nom の 1.33 倍高速
- oni-comb / winnow / nom すべてが低分散（CI range ~5 µs 以内）で安定

**Note:** この計測環境（AMD EPYC 7763, GitHub Actions VM）では、以前の Apple M-series ネイティブ計測（oni-comb: 109.6 µs, 977 MB/s）とは異なる結果となっている。仮想化環境のオーバーヘッドとキャッシュ特性の違いが影響していると考えられる。

---

## 9. Heap Allocation Measurement (dhat-rs)

### Token Workload

identifier / integer / flat_map same-type はすべて **0 allocations**。

パーサーコンビネータインフラ（`fn_parser`, `tag`, `char`, `whitespace0`, `take_while1`, `satisfy`, `zip`, `map`, `or` 等）は **完全にゼロアロケーション**。

### Full JSON Parse (107KB sample.json)

```
dhat: Total:     335,647 bytes in 743 blocks
dhat: At t-gmax: 218,047 bytes in 470 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

| Source | Bytes | Blocks | Description |
|--------|-------|--------|-------------|
| `Vec` grow | ~312,512 | ~700+ | JSON 配列/オブジェクトの要素収集 (`Vec::push` による grow) |
| `quoted_string_cow` slow path | ~23,135 | ~40 | エスケープ文字列のみ `Cow::Owned(String)` を構築 |

**すべてのアロケーションは AST 構築に起因**。パーサーコンビネータインフラ自体のアロケーションは 0。

---

## Summary Table

| Benchmark Group | Key Finding |
|----------------|-------------|
| Identifier | oni-comb は 1B で winnow/nom と同等、長い入力では 2–4 倍遅い（generic primitive のオーバーヘッド） |
| Integer | winnow/nom が一貫して最速。oni-comb は chumsky と同等 |
| flat_map same-type | oni-comb ~10.9 ns で安定。winnow/nom の約 2 倍 |
| flat_map boxed | Box\<dyn Parser\> により nom の 5–7 倍。pom の 3–4 倍高速 |
| zip vs flat_map | 同一オーダー。具象コンビネータ型設計は有効 |
| JSON subset | null 26 ns、object_large 2.6 µs。要素数に線形スケール |
| Arithmetic | recursive() のオーバーヘッドあり。chainl1 は効率的 |
| **Full JSON 107KB** | **oni-comb 220 MiB/s, winnow 288 MiB/s, nom 165 MiB/s** |
| Allocation | コンビネータインフラはゼロアロケーション |
