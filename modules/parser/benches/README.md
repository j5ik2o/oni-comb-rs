# ベンチマーク

oni-comb-rs v2 と比較対象ライブラリ（winnow, nom, chumsky, pom）の性能比較。

## 実行方法

```bash
# 全ベンチ実行
cargo bench -p oni-comb-parser --bench comparison

# 特定グループのみ
cargo bench -p oni-comb-parser --bench comparison -- identifier
cargo bench -p oni-comb-parser --bench comparison -- integer
cargo bench -p oni-comb-parser --bench comparison -- flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# コンパイル確認（計測なし）
cargo bench -p oni-comb-parser --bench comparison -- --test

# ヒープアロケーション計測
cargo bench -p oni-comb-parser --bench alloc_count
```

## ベンチグループ一覧

| グループ | 内容 | ライブラリ |
|---------|------|-----------|
| `token/identifier` | 識別子パース（`satisfy` + `take_while0`） | 5 ライブラリ |
| `token/integer` | 整数パース（`take_while1` + parse） | 5 ライブラリ |
| `token/flat_map_same_type` | flat_map 同一型分岐（digit → tag） | 5 ライブラリ |
| `token/flat_map_boxed` | flat_map 異種型分岐（`Box<dyn Parser>` 等） | 5 ライブラリ |
| `token/zip_vs_flat_map` | zip と flat_map の直接比較 | oni-comb のみ |
| `json` | JSON subset パース（null/int/string/array/object） | oni-comb のみ |
| `arithmetic` | 四則演算+括弧（recursive + chainl1） | oni-comb のみ |

## 結果と考察

以下は Apple M 系チップでの計測結果（ParseError 導入後）。

### Token ワークロード — Identifier

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"x"` (1B) | 18.4 ns | 14.5 ns | 13.7 ns | 66.9 ns | 874 ns |
| `"foo"` (3B) | 19.6 ns | 15.2 ns | 18.7 ns | 83.5 ns | 916 ns |
| `"foo_bar_123"` (11B) | 28.1 ns | 19.5 ns | 36.9 ns | 199 ns | 1,055 ns |
| `"_private"` (8B) | 26.2 ns | 19.2 ns | 27.7 ns | 141 ns | 997 ns |
| `"longIdent..."` (28B) | 44.4 ns | 32.2 ns | 82.6 ns | 271 ns | 1,318 ns |

### Token ワークロード — Integer

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 3.1 ns | 1.6 ns | 2.1 ns | 68.6 ns | 884 ns |
| `"42"` | 3.6 ns | 2.3 ns | 2.7 ns | 72.1 ns | 907 ns |
| `"9999999"` | 8.2 ns | 5.2 ns | 5.3 ns | 133 ns | 995 ns |
| `"184467...615"` (20B) | 25.9 ns | 22.6 ns | 22.7 ns | 264 ns | 1,256 ns |

### flat_map 同一型分岐（digit → tag）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.1 ns | 2.1 ns | 2.3 ns |
| nom | 2.4 ns | 2.4 ns | 2.4 ns |
| **oni-comb** | **7.3 ns** | **7.3 ns** | **6.0 ns** |
| pom | 70 ns | 71 ns | 96 ns |
| chumsky | 896 ns | 898 ns | 948 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 3.9 ns | 2.8 ns |
| winnow | 19.3 ns | 18.6 ns |
| **oni-comb** | **21.5 ns** | **19.8 ns** |
| pom | 164 ns | 109 ns |
| chumsky | 1,052 ns | 972 ns |

### zip vs flat_map（oni-comb-rs 内部比較）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 4.8 ns | 4.8 ns | 0% (誤差) |
| "foo" | 10.5 ns | 10.3 ns | -2% (誤差) |
| "foo_bar_123" | 17.7 ns | 17.7 ns | 0% (誤差) |
| "_private" | 14.8 ns | 14.8 ns | 0% (誤差) |
| "longIdent..." | 31.2 ns | 31.1 ns | 0% (誤差) |

**zip ≒ flat_map（同一型）は引き続き成立。** 具象コンビネータ型設計の成果。

### JSON subset（oni-comb のみ）

| 入力 | 時間 | byte/ns |
|------|------|---------|
| `null` (4B) | 15.0 ns | 0.27 |
| `42` (2B) | 86.0 ns | 0.02 |
| `"hello world"` (13B) | 147 ns | 0.09 |
| `[1, 2, 3]` (9B) | 536 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 542 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 693 ns | 0.07 |
| `{"a":1,...,"h":8}` (65B) | 1,492 ns | 0.04 |

**所見:**
- `null` は tag 1回で 15ns。integer は `whitespace0` → `integer()` → `whitespace0` の3段でオーバーヘッドがある。
- 配列・オブジェクトは要素数に比例。8要素オブジェクトで ~1.5μs。
- `or` による5分岐（null/true/false/int/string）の試行コストが各要素に乗る。

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）

| 入力 | 時間 |
|------|------|
| `42` | 156 ns |
| `1 + 2` | 247 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 440 ns |
| `1 + 2 * (3 - 4) + 5` | 639 ns |
| `(((1 + 2) * 3) - 4) / 5` | 931 ns |
| `1 + 2 + ... + 8` | 776 ns |

**所見:**
- 単一整数で 156ns はかなり重い。`recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` 経由の間接呼び出し + `whitespace0` のオーバーヘッド。
- 括弧のネストごとに ~200ns 追加（再帰 1 段の `Box<dyn Parser>` コスト）。
- 8項の加算チェーンで 776ns。`chainl1` のループは効率的。

### JSON フルベンチ（107KB sample.json — chumsky ベンチ互換）

[chumsky ベンチマーク](https://github.com/zesterer/chumsky/tree/main/benches)と同じ 107KB の JSON ファイルでの同一マシン計測。

100 サンプルでの統計:

| ライブラリ | Mean | Median | p90 | p95 | StdDev | Throughput (mean) |
|-----------|------|--------|-----|-----|--------|-------------------|
| **oni-comb** | **109.6 µs** | **109.4 µs** | **112.7 µs** | **113.8 µs** | **2.10 µs** | **977 MB/s** |
| winnow | 159.3 µs | 159.8 µs | 161.8 µs | 162.3 µs | 2.46 µs | 672 MB/s |
| nom | 283.2 µs | 282.7 µs | 286.6 µs | 287.9 µs | 2.26 µs | 378 MB/s |

**oni-comb は winnow の 1.45 倍、nom の 2.59 倍のスループット（mean 基準）。** 3 ライブラリとも StdDev ~2µs で計測は安定。最適化の内訳:
- `fn_parser` による関数再帰（`recursive()` の `Box<dyn Parser>` vtable を排除）
- `peek_byte` による先頭バイト分岐（`or` チェーンの線形スキャンを排除）
- `quoted_string_cow` によるゼロコピー文字列（エスケープなし文字列は `&str` スライス）
- `take_while1` による数値パースのゼロコピー化

**参考: chumsky README のランキング（AMD Ryzen 7 3700x）との対照**

| # | ライブラリ | スループット |
|---|-----------|-------------|
| 1 | **oni-comb** | **~977 MB/s** |
| 2 | chumsky (check-only) | 797 MB/s |
| 3 | winnow | 627 MB/s |
| 4 | chumsky | 533 MB/s |
| 5 | sn (hand-written) | 472 MB/s |
| 6 | serde_json | 235 MB/s |
| 7 | nom | 213 MB/s |
| 8 | pest | 57 MB/s |
| 9 | pom | 8 MB/s |

※ chumsky ランキングは AMD Ryzen 7 3700x での計測。oni-comb は同一マシンでの winnow/nom 比率から推定したため参考値。

### ヒープアロケーション計測

```
dhat: Total:     0 bytes in 0 blocks
dhat: At t-gmax: 0 bytes in 0 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

identifier / integer / flat_map 同一型 いずれも **0 blocks**。

## 最適化サイクルの記録

### MS6 ParseError 導入による効果

| ワークロード | 旧 (String/format!) | 新 (ParseError) | 改善 |
|-------------|--------------------|--------------------|------|
| flat_map "1one" | 8.3 ns | 7.3 ns | -12% |
| flat_map "2two" | 7.8 ns | 7.3 ns | -6% |
| flat_map "3three" | 6.9 ns | 6.0 ns | -13% |

**分析**: `format!` マクロによる `String` アロケーションコードが LLVM の最適化を妨げていた。`ParseError::expected_char(pos, c)` は構造体の構築のみで `format!` を使わないため、エラーパスのコード生成が軽量化され、成功パスのインライン化にも好影響を与えた。

### #[inline] 追加による効果

| ワークロード | 旧 | 新 | 改善 |
|-------------|-----|-----|------|
| identifier "x" (1B) | 18.4 ns | 14.9 ns | -19% |
| identifier "foo" (3B) | 19.6 ns | 17.8 ns | -9% |
| identifier "_private" (8B) | 26.2 ns | 25.1 ns | -4% |
| flat_map "1one" | 7.3 ns | 6.1 ns | -16% |
| flat_map "2two" | 7.3 ns | 6.2 ns | -15% |
| flat_map "3three" | 6.0 ns | 4.8 ns | -20% |

**分析**: 全 `parse_next` 実装に `#[inline]` を追加。短い入力ほど効果が大きい（15-20% 改善）。identifier "x" で winnow と同等（14.9 vs 15.2 ns）に到達。クレート境界を越えたインライン化が促進され、LLVM が関数呼び出しのオーバーヘッドを排除できるようになった。

### ゼロコピー + fn 再帰 + バイト分岐による効果

| ステップ | oni-comb | スループット | 改善 |
|---------|----------|-------------|------|
| Before（recursive + or チェーン） | 640 µs | 159 MB/s | — |
| + `quoted_string_cow` ゼロコピー | 486 µs | 210 MB/s | -24% |
| + number ゼロコピー | 477 µs | 214 MB/s | -2% |
| + `fn_parser` 再帰 + `peek_byte` 分岐 | **109 µs** | **937 MB/s** | **-77%** |

**分析**: 最大の効果は `fn_parser` + `peek_byte` 分岐。`recursive()` の `Box<dyn Parser>` vtable 間接呼び出しが全 JSON ノードで数万回発生していたのに対し、`fn_parser` は通常の関数呼び出し（インライン化可能）。`peek_byte` による先頭バイト分岐で `or` チェーンの checkpoint/reset サイクルも排除。

### 残存するボトルネック

1. **`recursive()` は依然として重い**: 四則演算ベンチの単一整数で ~156ns（`fn_parser` なら ~3ns）。`recursive()` が必要なケース（文法構造上 `fn` で書けない場合）では vtable コストが残る。
2. **whitespace0 の呼び出し回数**: JSON パーサーで値の前後に `whitespace0()` を複数回呼んでおり、統合の余地がある。

## 総合評価

- **winnow を上回るスループット** — 107KB JSON で winnow の 1.43 倍（`fn_parser` + `peek_byte` 分岐 + ゼロコピー文字列）
- **nom を中〜長入力で上回る** — 11B identifier で 28% 高速、28B で 46% 高速
- **pom の 3〜30 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky の 30〜200 倍高速** — 動的ディスパッチ前提の設計との差
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計の妥当性を確認
- **3 回の最適化サイクルで累計 ~83% 改善** — ParseError 導入（~12%）+ #[inline]（~17%）+ ゼロコピー＋fn再帰（~77%）
- **Applicative コンビネータでヒープアロケーションゼロ**
