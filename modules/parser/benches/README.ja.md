# ベンチマーク

[English](README.md)

oni-comb-rs v2 と比較対象ライブラリ（winnow, nom, chumsky, pom）の性能比較。

## 実行方法

```bash
# 比較ベンチ実行（token / JSON subset / arithmetic）
cargo bench -p oni-comb-parser --bench comparison

# comparison 内の特定グループのみ
cargo bench -p oni-comb-parser --bench comparison -- identifier
cargo bench -p oni-comb-parser --bench comparison -- integer
cargo bench -p oni-comb-parser --bench comparison -- flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# 独立した JSON フルベンチ（107KB sample.json）
cargo bench -p oni-comb-parser --bench json_full

# コンパイル確認（計測なし）
cargo bench -p oni-comb-parser --bench comparison -- --test
cargo bench -p oni-comb-parser --bench json_full -- --test

# ヒープアロケーション計測
cargo bench -p oni-comb-parser --bench alloc_count
```

## ベンチターゲット一覧

| ターゲット | 対象 | 補足 |
|-----------|------|------|
| `comparison` | token マイクロベンチ、JSON subset、四則演算 | `identifier`, `integer`, `flat_map`, `zip_vs`, `json`, `arithmetic` などの Criterion filter を利用可能 |
| `json_full` | 107KB `sample.json` のフルパース順位比較 | 実運用寄りの JSON 負荷でマクロスループットを測る独立ハーネス |
| `alloc_count` | `dhat-rs` によるヒープアロケーション計測 | token ワークロードと JSON フルパースの両方を計測 |

## この README に含まれる結果

| 結果セクション | 元のターゲット |
|---------------|----------------|
| Token ワークロード（`identifier`, `integer`, `flat_map`, `zip_vs_flat_map`） | `comparison` |
| JSON subset と四則演算 | `comparison` |
| 107KB `sample.json` の JSON フルベンチ | `json_full` |
| ヒープアロケーション計測 | `alloc_count` |

## ベンチグループ一覧（`comparison` ターゲット）

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
| `"x"` (1B) | 15.0 ns | 15.2 ns | 15.1 ns | 17.5 ns | 68.2 ns |
| `"foo"` (3B) | 14.7 ns | 16.2 ns | 16.5 ns | 29.4 ns | 86.5 ns |
| `"foo_bar_123"` (11B) | 18.6 ns | 20.3 ns | 33.2 ns | 86.9 ns | 202.0 ns |
| `"_private"` (8B) | 18.2 ns | 19.7 ns | 29.3 ns | 60.1 ns | 146.4 ns |
| `"longIdent..."` (28B) | 30.2 ns | 33.2 ns | 86.5 ns | 132.7 ns | 269.7 ns |

### Token ワークロード — Integer（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 2.2 ns | 2.3 ns | 1.9 ns | 72.0 ns | 20.8 ns |
| `"42"` | 2.6 ns | 2.8 ns | 2.8 ns | 70.8 ns | 20.7 ns |
| `"9999999"` | 5.2 ns | 5.4 ns | 5.4 ns | 132.3 ns | 29.5 ns |
| `"184467...615"` (20B) | 20.0 ns | 23.1 ns | 22.7 ns | 273.1 ns | 100.1 ns |

### flat_map 同一型分岐（digit → tag）（mean）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.6 ns | 2.6 ns | 2.7 ns |
| nom | 3.2 ns | 2.7 ns | 2.4 ns |
| **oni-comb** | **5.7 ns** | **5.5 ns** | **4.1 ns** |
| chumsky | 72.8 ns | 51.9 ns | 51.5 ns |
| pom | 76.2 ns | 69.0 ns | 95.0 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）（mean）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 4.6 ns | 2.8 ns |
| winnow | 19.2 ns | 17.7 ns |
| **oni-comb** | **20.7 ns** | **18.3 ns** |
| chumsky | 41.4 ns | 24.2 ns |
| pom | 307.5 ns | 114.2 ns |

### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 2.3 ns | 1.9 ns | flat_map がやや速い |
| "foo" | 2.7 ns | 2.5 ns | flat_map がやや速い |
| "foo_bar_123" | 6.7 ns | 6.6 ns | ≈0% (誤差) |
| "_private" | 5.3 ns | 6.3 ns | zip がやや速い |
| "longIdent..." | 15.6 ns | 15.4 ns | ≈0% (誤差) |

**zip と flat_map（同一型）は、依然として同じ低 ns 台から十数 ns 台のレンジに収まる。** 具象コンビネータ型設計に、構造的な flat_map ペナルティは見られない。

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

この節は `cargo bench -p oni-comb-parser --bench json_full` の結果。

2026-03-18 に同一マシンで 107KB JSON ファイルを再計測（100 サンプル）。`winnow` のベンチ依存も 0.7 から 1.0.0 に更新済み。

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| **oni-comb** | **112.6 µs** | **906.6** |
| winnow | 202.6 µs | 503.9 |
| nom | 280.2 µs | 364.4 |
| chumsky | 552.1 µs | 184.9 |
| pom | 8.58 ms | 11.9 |

**今回の再計測では oni-comb が JSON フルベンチの首位を奪還した。`winnow` 1.0.0 の 1.80 倍、nom の 2.49 倍、chumsky の 4.90 倍、pom の 76.2 倍のスループットに到達している。** この順位を支えているのは、従来の設計要素に加えて generic primitive の fast path 整理:
- `fn_parser` による関数再帰（`recursive()` の `Box<dyn Parser>` vtable を排除）
- `peek_byte` による先頭バイト分岐（`or` チェーンの線形スキャンを排除）
- `quoted_string` によるゼロコピー文字列（エスケープなし文字列は `&str` スライス）
- `take_while1` による数値パースのゼロコピー化
- `StrInput` の ASCII fast path
- `satisfy` / `take_while*` / `one_of` / `none_of` の consume-then-reset 化による二重デコード削減

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
2. **`flat_map` はまだ最速勢より重い**: 同一型分岐は 5.7 / 5.5 / 4.1ns まで改善したが、winnow / nom にはまだ差がある。異種型 boxed 分岐は winnow にかなり近づいたが、nom との差は大きい。

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

**その後の回復（2026-03-18 fast path パス）**: `StrInput` への ASCII fast path 追加と、generic primitive の `peek_token()` + `next_token()` を `next_token()` + mismatch 時 `reset()` に置き換えたことで、この回帰の大半を ASCII 中心ワークロードでは回収した。現在の mean は identifier `"foo_bar_123"` が 18.6ns、integer `"184467...615"` が 20.0ns まで戻っている。

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
| "x" | 2.3 ns | 1.9 ns | 同レンジ |
| "foo" | 2.7 ns | 2.5 ns | 同レンジ |
| "foo_bar_123" | 6.7 ns | 6.6 ns | 同レンジ |
| "_private" | 5.3 ns | 6.3 ns | 同レンジ |
| "longIdent..." | 15.6 ns | 15.4 ns | 同レンジ |

## 総合評価

- **oni-comb がマクロベンチ首位に復帰** — 107KB JSON 再計測で 906.6 MiB/s、winnow は 503.9 MiB/s
- **oni-comb は JSON フルで nom / chumsky / pom に大差を付けた** — nom の 2.49 倍、chumsky の 4.90 倍、pom の 76.2 倍
- **generic token パーサーは、もはや以前ほどの弱点ではない** — identifier 11B: oni-comb 18.6ns vs winnow 20.3ns / nom 33.2ns、integer 20B: oni-comb 20.0ns vs winnow 23.1ns / nom 22.7ns
- **chumsky 0.12 は旧版より大幅改善したまま** — 短い identifier は今も oni-comb と同じ桁にある（`"x"`: 17.5ns vs 15.0ns）が、中〜長入力ではなお差がある（`"foo_bar_123"`: 86.9ns vs 18.6ns）
- **flat_map は依然として最速勢に差がある** — とくに同一型分岐（`"1one"`: oni-comb 5.7ns vs winnow 2.6ns / nom 3.2ns）
- **zip と flat_map は同レンジに収まる** — 具象コンビネータ型設計の妥当性を確認
- **今回の再計測でも JSON subset / arithmetic は安定** — JSON `object_large` は ~1.32µs、arithmetic `complex` は ~618ns
- **Applicative コンビネータでヒープアロケーションゼロ**
