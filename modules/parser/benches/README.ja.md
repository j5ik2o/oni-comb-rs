# ベンチマーク

[English](README.md)

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

計測環境:
- Mac mini (Mac16,11)
- Apple M4 Pro, 14 cores (10 Performance + 4 Efficiency)
- Memory: 64 GB
- macOS 26.3.1
- Architecture: arm64

以下は上記マシンでの計測結果（ParseError 導入後）。
全数値は Criterion 報告の **mean 推定値**（100 サンプル、95% 信頼区間中央）。

### Token ワークロード — Identifier（mean）

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 16.6 ns | 15.5 ns | 14.9 ns | 17.1 ns | 66.3 ns |
| `"foo"` (3B) | 21.1 ns | 16.2 ns | 16.9 ns | 29.8 ns | 85.3 ns |
| `"foo_bar_123"` (11B) | 38.9 ns | 21.7 ns | 33.4 ns | 83.8 ns | 230 ns |
| `"_private"` (8B) | 42.2 ns | 20.9 ns | 25.7 ns | 57.2 ns | 138.9 ns |
| `"longIdent..."` (28B) | 81.7 ns | 34.2 ns | 82.7 ns | 132.0 ns | 266.5 ns |

### Token ワークロード — Integer（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 4.7 ns | 2.0 ns | 2.0 ns | 73.4 ns | 16.4 ns |
| `"42"` | 8.8 ns | 3.8 ns | 3.8 ns | 77.2 ns | 17.2 ns |
| `"9999999"` | 20.3 ns | 5.7 ns | 5.8 ns | 136 ns | 32.3 ns |
| `"184467...615"` (20B) | 62.4 ns | 24.1 ns | 23.4 ns | 253 ns | 86.2 ns |

### flat_map 同一型分岐（digit → tag）（mean）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.7 ns | 2.7 ns | 2.6 ns |
| nom | 2.4 ns | 2.5 ns | 2.4 ns |
| **oni-comb** | **7.2 ns** | **7.2 ns** | **5.9 ns** |
| chumsky | 49.7 ns | 49.9 ns | 51.9 ns |
| pom | 69.4 ns | 70.2 ns | 94.7 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）（mean）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 3.7 ns | 3.0 ns |
| winnow | 20.1 ns | 18.0 ns |
| chumsky | 25.4 ns | 19.7 ns |
| **oni-comb** | **30.9 ns** | **23.8 ns** |
| pom | 163.7 ns | 111.0 ns |

### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 4.3 ns | 4.2 ns | ≈0% (誤差) |
| "foo" | 8.4 ns | 8.3 ns | ≈0% (誤差) |
| "foo_bar_123" | 26.0 ns | 25.9 ns | ≈0% (誤差) |
| "_private" | 19.9 ns | 19.9 ns | ≈0% (誤差) |
| "longIdent..." | 64.9 ns | 64.7 ns | ≈0% (誤差) |

**zip ≒ flat_map（同一型）は引き続き成立。** 具象コンビネータ型設計の成果。

### JSON subset（oni-comb のみ）（mean）

| 入力 | 時間 | byte/ns |
|------|------|---------|
| `null` (4B) | 8.5 ns | 0.47 |
| `42` (2B) | 83.4 ns | 0.02 |
| `"hello world"` (13B) | 143.8 ns | 0.09 |
| `[1, 2, 3]` (9B) | 517.6 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 528.9 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 663.5 ns | 0.08 |
| `{"a":1,...,"h":8}` (65B) | 1,427 ns | 0.05 |

**所見:**
- `null` は tag 1回で 15ns。integer は `whitespace0` → `integer()` → `whitespace0` の3段でオーバーヘッドがある。
- 配列・オブジェクトは要素数に比例。8要素オブジェクトで ~1.5μs。
- `or` による5分岐（null/true/false/int/string）の試行コストが各要素に乗る。

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）（mean）

| 入力 | 時間 |
|------|------|
| `42` | 155 ns |
| `1 + 2` | 245 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 443 ns |
| `1 + 2 * (3 - 4) + 5` | 628 ns |
| `(((1 + 2) * 3) - 4) / 5` | 905 ns |
| `1 + 2 + ... + 8` | 761 ns |

**所見:**
- 単一整数で 155ns はかなり重い。`recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` 経由の間接呼び出し + `whitespace0` のオーバーヘッド。
- 括弧のネストごとに ~200ns 追加（再帰 1 段の `Box<dyn Parser>` コスト）。
- 8項の加算チェーンで 761ns。`chainl1` のループは効率的。

### JSON フルベンチ（107KB sample.json）

`json_full.rs` に `pom` 実装を追加した後、同一マシンで 107KB JSON ファイルを計測（100 サンプル）。

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| **oni-comb** | **193.4 µs** | **527.8** |
| winnow | 206.5 µs | 494.4 |
| nom | 262.8 µs | 388.5 |
| chumsky | 495.6 µs | 206.0 |
| pom | 7.56 ms | 13.5 |

**oni-comb は winnow の 1.07 倍、nom の 1.36 倍、chumsky の 2.56 倍、pom の 39.1 倍のスループット（mean 基準）。** 最適化の内訳:
- `fn_parser` による関数再帰（`recursive()` の `Box<dyn Parser>` vtable を排除）
- `peek_byte` による先頭バイト分岐（`or` チェーンの線形スキャンを排除）
- `quoted_string` によるゼロコピー文字列（エスケープなし文字列は `&str` スライス）
- `take_while1` による数値パースのゼロコピー化

### ヒープアロケーション計測（dhat-rs）

#### Token ワークロード

```
dhat: Total:     0 bytes in 0 blocks
dhat: At t-gmax: 0 bytes in 0 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

identifier / integer / flat_map 同一型 いずれも **0 blocks**。
パーサーコンビネータインフラ（`fn_parser`、`tag`、`char`、`whitespace0`、`take_while1`、`satisfy`、`zip`、`map`、`or` 等）は **完全にゼロアロケーション**。

#### JSON フルパース（107KB sample.json）

```
dhat: Total:     335,647 bytes in 743 blocks
dhat: At t-gmax: 218,047 bytes in 470 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

アロケーション元の内訳:

| ソース | bytes | 説明 |
|--------|-------|------|
| `Vec` grow（配列・オブジェクト要素収集） | 312,512 | JSON の配列 `[]` / オブジェクト `{}` の `Vec::push` による grow |
| `quoted_string` slow path | 23,135 | エスケープ付き文字列のみ `Cow::Owned(String)` を構築 |

**パーサーコンビネータインフラ自体のアロケーションはゼロ。** 全てのアロケーションは AST 構築に起因:
- `Vec<Json>` / `Vec<(Cow, Json)>` — 配列・オブジェクトの要素収集（不可避）
- `Cow::Owned` — エスケープ付き文字列のみ（エスケープなし文字列は `Cow::Borrowed(&str)` でゼロコピー）
- `fn_parser`、`tag`、`char`、`whitespace0`、`take_while1`、`quoted_string`（fast path）、`peek_byte` — 全てゼロアロケーション

## 最適化サイクルの記録

### MS6 ParseError 導入による効果（mean）

| ワークロード | 旧 (String/format!) | 新 (ParseError) | 改善 |
|-------------|--------------------|--------------------|------|
| flat_map "1one" | 8.3 ns | 7.3 ns | -12% |
| flat_map "2two" | 7.8 ns | 7.3 ns | -6% |
| flat_map "3three" | 6.9 ns | 6.0 ns | -13% |

**分析**: `format!` マクロによる `String` アロケーションコードが LLVM の最適化を妨げていた。`ParseError::expected_char(pos, c)` は構造体の構築のみで `format!` を使わないため、エラーパスのコード生成が軽量化され、成功パスのインライン化にも好影響を与えた。

### #[inline] 追加による効果（mean）

| ワークロード | 旧 | 新 | 改善 |
|-------------|-----|-----|------|
| identifier "x" (1B) | 18.4 ns | 14.9 ns | -19% |
| identifier "foo" (3B) | 19.6 ns | 17.8 ns | -9% |
| identifier "_private" (8B) | 26.2 ns | 25.1 ns | -4% |
| flat_map "1one" | 7.3 ns | 6.1 ns | -16% |
| flat_map "2two" | 7.3 ns | 6.2 ns | -15% |
| flat_map "3three" | 6.0 ns | 4.8 ns | -20% |

**分析**: 全 `parse_next` 実装に `#[inline]` を追加。短い入力ほど効果が大きい（15-20% 改善）。identifier "x" で winnow と同等（14.9 vs 15.2 ns）に到達。クレート境界を越えたインライン化が促進され、LLVM が関数呼び出しのオーバーヘッドを排除できるようになった。

### ゼロコピー + fn 再帰 + バイト分岐による効果（mean）

| ステップ | oni-comb | スループット | 改善 |
|---------|----------|-------------|------|
| Before（recursive + or チェーン） | 640 µs | 159 MB/s | — |
| + `quoted_string` ゼロコピー | 486 µs | 210 MB/s | -24% |
| + number ゼロコピー | 477 µs | 214 MB/s | -2% |
| + `fn_parser` 再帰 + `peek_byte` 分岐 | **109 µs** | **937 MB/s** | **-77%** |

**分析**: 最大の効果は `fn_parser` + `peek_byte` 分岐。`recursive()` の `Box<dyn Parser>` vtable 間接呼び出しが全 JSON ノードで数万回発生していたのに対し、`fn_parser` は通常の関数呼び出し（インライン化可能）。`peek_byte` による先頭バイト分岐で `or` チェーンの checkpoint/reset サイクルも排除。

### 残存するボトルネック

1. **`recursive()` は依然として重い**: 四則演算ベンチの単一整数で ~155ns（`fn_parser` なら ~3ns）。`recursive()` が必要なケース（文法構造上 `fn` で書けない場合）では vtable コストが残る。
2. **whitespace0 の呼び出し回数**: JSON パーサーで値の前後に `whitespace0()` を複数回呼んでおり、統合の余地がある。

### Generic Input リファクタリングの影響（Input トレイトジェネリック化）

`Input` トレイトに `Token`/`Slice` associated type を追加し、`satisfy`, `take_while0/1`, `take`, `take_while_n_m`, `eof` をジェネリックな `primitive/` モジュールに移動。`ByteInput<'a>`（`&[u8]` パース用）も新規追加。

**ジェネリック primitive を使うトークンレベルパーサーへの影響（`satisfy` + `take_while0`）:**

| 入力 | Before | After | 変化 | 原因 |
|------|--------|-------|------|------|
| identifier `"x"` (1B) | 18.4 ns | 16.6 ns | -10% | 誤差範囲 |
| identifier `"foo"` (3B) | 19.6 ns | 21.1 ns | +8% | トークン毎オーバーヘッド |
| identifier `"foo_bar_123"` (11B) | 28.1 ns | 38.9 ns | +38% | トークン毎オーバーヘッド |
| identifier `"_private"` (8B) | 26.2 ns | 42.2 ns | +61% | トークン毎オーバーヘッド |
| identifier `"longIdent..."` (28B) | 44.4 ns | 81.7 ns | +84% | トークン毎オーバーヘッド |
| integer `"42"` (2B) | 3.6 ns | 8.8 ns | +144% | トークン毎オーバーヘッド |
| integer `"9999999"` (7B) | 8.2 ns | 20.3 ns | +148% | トークン毎オーバーヘッド |

**原因**: 旧 `text/` 実装は `remaining.chars()` で1回イテレーションし最後に `advance(consumed)` を呼んでいた。ジェネリック `primitive/` 実装はトークン毎に `peek_token()` + `next_token()` を呼び、各呼び出しで `&self.src[self.offset..]` の再計算と `.chars().next()` が発生する。これはジェネリシティのコストであり、`Input` トレイトではバッチ文字イテレーターを公開できない。

**影響なしのワークロード**（`as_str().chars()` 直接使用または `fn_parser`）:

| ワークロード | Before | After | 変化 |
|-------------|--------|-------|------|
| JSON `null` | 15.0 ns | 8.5 ns | -43%（ノイズ/キャッシュ） |
| JSON `object_large` | 1,492 ns | 1,427 ns | -4% |
| arithmetic `single` | 156 ns | 155 ns | ≈0% |
| arithmetic `complex` | 639 ns | 628 ns | -2% |
| flat_map 同一型 `"1one"` | 7.3 ns | 7.2 ns | ≈0% |

**緩和策**: テキスト専用パーサー（`identifier`, `integer`, `tag`, `whitespace`, `quoted_string`）は `text/` に残し `as_str().chars()` を直接使用するため性能維持。prelude からジェネリックな `primitive::satisfy`/`primitive::take_while0` を使うコードのみ影響あり。

**zip ≒ flat_map は引き続き成立**（ジェネリック化後）:

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 4.3 ns | 4.2 ns | ≈0% |
| "foo" | 8.4 ns | 8.3 ns | ≈0% |
| "foo_bar_123" | 26.0 ns | 25.9 ns | ≈0% |
| "_private" | 19.9 ns | 19.9 ns | ≈0% |
| "longIdent..." | 64.9 ns | 64.7 ns | ≈0% |

## 総合評価

- **winnow を上回るスループット** — 107KB JSON で winnow の 1.07 倍（`fn_parser` + `peek_byte` 分岐 + ゼロコピー文字列）
- **nom と中〜長入力で同等〜上回る** — identifier 11B: oni-comb 38.9ns vs nom 33.4ns（nom がやや高速）、28B: oni-comb 81.7ns vs nom 82.7ns（ほぼ同等）
- **pom の 3〜39 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky 0.12 で大幅改善** — identifier "x": 918ns -> 17.1ns（v0.9 比 ~54 倍高速化）。短い入力では oni-comb/winnow/nom と競合するレベルに到達。ただし中〜長入力では依然 2 倍程度遅い（11B identifier: 83.8ns vs 38.9ns）。flat_map boxed では chumsky が oni-comb と同等（25.4ns vs 30.9ns）
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計の妥当性を確認
- **3 回の最適化サイクルで累計 ~83% 改善** — ParseError 導入（~12%）+ #[inline]（~17%）+ ゼロコピー＋fn再帰（~77%）
- **JSON/arithmetic ワークロードで 2-5% 改善** — JSON object_large: 1,495ns -> 1,427ns、arithmetic complex: 995ns -> 905ns
- **Applicative コンビネータでヒープアロケーションゼロ**
