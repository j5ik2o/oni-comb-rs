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

以下は 2026-03-18 に上記マシンで再計測した結果（ベンチマーク基準の `winnow` を 1.0.0 に更新後）。
全数値は Criterion 報告の **mean 推定値**（100 サンプル、95% 信頼区間中央）。

### Token ワークロード — Identifier（mean）

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 17.7 ns | 16.9 ns | 15.6 ns | 17.8 ns | 67.2 ns |
| `"foo"` (3B) | 21.7 ns | 15.7 ns | 16.2 ns | 27.8 ns | 83.7 ns |
| `"foo_bar_123"` (11B) | 39.2 ns | 19.8 ns | 32.7 ns | 84.7 ns | 203.5 ns |
| `"_private"` (8B) | 33.8 ns | 19.7 ns | 24.3 ns | 56.0 ns | 140.5 ns |
| `"longIdent..."` (28B) | 80.1 ns | 33.3 ns | 81.4 ns | 130.8 ns | 263.5 ns |

### Token ワークロード — Integer（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 4.3 ns | 2.0 ns | 1.8 ns | 69.9 ns | 20.7 ns |
| `"42"` | 6.9 ns | 2.7 ns | 2.5 ns | 72.6 ns | 20.9 ns |
| `"9999999"` | 19.3 ns | 5.2 ns | 5.1 ns | 131.5 ns | 28.8 ns |
| `"184467...615"` (20B) | 59.2 ns | 22.3 ns | 21.9 ns | 256.4 ns | 94.0 ns |

### flat_map 同一型分岐（digit → tag）（mean）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.4 ns | 2.4 ns | 2.7 ns |
| nom | 2.4 ns | 2.4 ns | 2.3 ns |
| **oni-comb** | **6.8 ns** | **6.8 ns** | **5.5 ns** |
| chumsky | 48.4 ns | 48.8 ns | 51.6 ns |
| pom | 69.9 ns | 69.6 ns | 94.4 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）（mean）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 3.6 ns | 2.7 ns |
| winnow | 18.8 ns | 17.6 ns |
| chumsky | 25.7 ns | 18.8 ns |
| **oni-comb** | **30.5 ns** | **23.3 ns** |
| pom | 161.8 ns | 110.5 ns |

### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 3.8 ns | 3.8 ns | ≈0% (誤差) |
| "foo" | 8.0 ns | 8.0 ns | ≈0% (誤差) |
| "foo_bar_123" | 25.2 ns | 25.3 ns | ≈0% (誤差) |
| "_private" | 19.3 ns | 19.3 ns | ≈0% (誤差) |
| "longIdent..." | 62.9 ns | 62.8 ns | ≈0% (誤差) |

**zip ≒ flat_map（同一型）は引き続き成立。** 具象コンビネータ型設計の成果。

### JSON subset（oni-comb のみ）（mean）

| 入力 | 時間 | byte/ns |
|------|------|---------|
| `null` (4B) | 8.4 ns | 0.48 |
| `42` (2B) | 77.5 ns | 0.03 |
| `"hello world"` (13B) | 115.4 ns | 0.11 |
| `[1, 2, 3]` (9B) | 484.4 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 499.9 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 625.6 ns | 0.08 |
| `{"a":1,...,"h":8}` (64B) | 1,322 ns | 0.05 |

**所見:**
- `null` は tag 1回で ~8.4ns。`integer` は依然として `whitespace0` → `integer()` → `whitespace0` の3段ぶん固定コストが乗る。
- 配列・オブジェクトは引き続き要素数にほぼ比例。8フィールドのオブジェクトで ~1.32μs。
- 固定コストの主体は分岐ディスパッチと空白処理で、今回の string / object の改善はコード生成やキャッシュ影響も含むが、ボトルネックの形自体は変わっていない。

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）（mean）

| 入力 | 時間 |
|------|------|
| `42` | 151 ns |
| `1 + 2` | 240 ns |
| `1 + 2 * 3` | 265 ns |
| `(1 + 2) * 3` | 429 ns |
| `1 + 2 * (3 - 4) + 5` | 618 ns |
| `(((1 + 2) * 3) - 4) / 5` | 912 ns |
| `1 + 2 + ... + 8` | 762 ns |

**所見:**
- 単一整数で ~151ns は依然かなり重い。`recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` 経由の間接呼び出し + `whitespace0` のオーバーヘッド。
- 括弧のネストごとにおおむね ~180-200ns 追加されており、再帰 1 段の `Box<dyn Parser>` コストと整合する。
- 8項の加算チェーンは ~0.76µs で横ばい。主ボトルネックは `chainl1` ループ自体ではない。

### JSON フルベンチ（107KB sample.json）

2026-03-18 に同一マシンで 107KB JSON ファイルを再計測（100 サンプル）。`winnow` のベンチ依存も 0.7 から 1.0.0 に更新済み。

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| oni-comb | 203.7 µs | 501.1 |
| **winnow** | **180.7 µs** | **564.8** |
| nom | 260.5 µs | 391.8 |
| chumsky | 490.0 µs | 208.3 |
| pom | 7.33 ms | 13.9 |

**今回の再計測では `winnow` 1.0.0 が JSON フルベンチの首位。oni-comb は nom の 1.28 倍、chumsky の 2.41 倍、pom の 36.0 倍のスループットを維持しつつ、winnow 比では 0.89 倍だった。** それでも順位を支えているのは次の設計要素:
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

以下の表は過去の最適化ステップで取得した履歴値であり、上の 2026-03-18 再計測結果と直接比較するためのものではない。どこで速度改善が出たかを説明するための記録として残している。

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

1. **`recursive()` は依然として重い**: 四則演算ベンチの単一整数で ~151ns（`fn_parser` なら ~3ns）。`recursive()` が必要なケース（文法構造上 `fn` で書けない場合）では vtable コストが残る。
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

- **`winnow` 1.0.0 がマクロベンチで先行** — 107KB JSON 再計測で 564.8 MiB/s、oni-comb は 501.1 MiB/s
- **oni-comb は JSON フルで nom / chumsky / pom より依然高速** — nom の 1.28 倍、chumsky の 2.41 倍、pom の 36.0 倍
- **token レベルのパーサーは winnow / nom に対して依然弱い** — identifier 11B: oni-comb 39.2ns vs winnow 19.8ns / nom 32.7ns、integer 20B: oni-comb 59.2ns vs winnow 22.3ns / nom 21.9ns
- **chumsky 0.12 は旧版より大幅改善したまま** — 短い identifier は oni-comb に近い（`"x"`: 17.8ns vs 17.7ns）が、中〜長入力ではなお差がある（`"foo_bar_123"`: 84.7ns vs 39.2ns）
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計の妥当性を確認
- **今回の再計測でも JSON subset / arithmetic は安定** — JSON `object_large` は ~1.32µs、arithmetic `complex` は ~618ns
- **Applicative コンビネータでヒープアロケーションゼロ**
