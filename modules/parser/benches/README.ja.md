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

以下の `comparison` テーブルは 2026-03-21 に上記マシンで再計測した結果。
全数値は Criterion 報告の **mean 推定値**（100 サンプル、95% 信頼区間中央）。
この文書後半の JSON フルベンチ節だけは、別ハーネス `json_full` による 2026-03-18 の再計測結果を維持している。

### Token ワークロード — Identifier（mean）

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 14.6 ns | 15.5 ns | 14.0 ns | 16.6 ns | 67.7 ns |
| `"foo"` (3B) | 15.4 ns | 15.9 ns | 15.7 ns | 27.5 ns | 84.3 ns |
| `"foo_bar_123"` (11B) | 20.0 ns | 20.5 ns | 33.3 ns | 85.0 ns | 205.1 ns |
| `"_private"` (8B) | 19.3 ns | 20.0 ns | 24.8 ns | 58.1 ns | 145.2 ns |
| `"longIdent..."` (28B) | 33.8 ns | 33.6 ns | 83.1 ns | 133.4 ns | 272.5 ns |

### Token ワークロード — Integer（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 2.3 ns | 2.1 ns | 2.0 ns | 69.0 ns | 21.1 ns |
| `"42"` | 3.1 ns | 2.8 ns | 2.6 ns | 74.3 ns | 21.7 ns |
| `"9999999"` | 6.8 ns | 6.2 ns | 5.2 ns | 133.0 ns | 29.6 ns |
| `"184467...615"` (20B) | 22.8 ns | 22.7 ns | 22.5 ns | 252.5 ns | 95.1 ns |

### flat_map 同一型分岐（digit → tag）（mean）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.4 ns | 2.4 ns | 2.6 ns |
| nom | 2.6 ns | 2.6 ns | 2.7 ns |
| **oni-comb** | **10.6 ns** | **10.7 ns** | **10.4 ns** |
| chumsky | 49.1 ns | 49.5 ns | 52.0 ns |
| pom | 69.9 ns | 70.1 ns | 95.2 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）（mean）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 3.7 ns | 2.7 ns |
| winnow | 20.1 ns | 17.2 ns |
| **oni-comb** | **24.2 ns** | **21.9 ns** |
| chumsky | 25.8 ns | 18.7 ns |
| pom | 166.3 ns | 109.5 ns |

### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 3.1 ns | 2.4 ns | flat_map が速い |
| "foo" | 3.4 ns | 3.3 ns | ≈0% (誤差) |
| "foo_bar_123" | 8.4 ns | 7.9 ns | flat_map がやや速い |
| "_private" | 6.4 ns | 6.2 ns | ≈0% (誤差) |
| "longIdent..." | 18.8 ns | 18.0 ns | flat_map がやや速い |

**zip と flat_map（同一型）は今回も同じレンジに収まっているが、ペア全体としては前回スナップショットより上振れた。** それでも、具象コンビネータ型設計に大きな構造的 flat_map ペナルティは見られない。

### JSON subset（oni-comb のみ）（mean）

| 入力 | 時間 | byte/ns |
|------|------|---------|
| `null` (4B) | 19.5 ns | 0.21 |
| `42` (2B) | 93.9 ns | 0.02 |
| `"hello world"` (13B) | 139.9 ns | 0.09 |
| `[1, 2, 3]` (9B) | 530.9 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 557.0 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 704.0 ns | 0.07 |
| `{"a":1,...,"h":8}` (64B) | 1,459 ns | 0.04 |

**所見:**
- 2026-03-18 のスナップショットと比べると、今回の `comparison` 再計測では JSON subset の全ケースで悪化が見られた。primitive 寄りのケースはさらに悪化し、`null` は ~19.5ns、`integer` は ~93.9ns、`string` は ~139.9ns になった。
- 以前見えていた object 側の改善も今回は維持されず、`object` は ~704ns、`object_large` は ~1.46µs まで戻っている。前回の改善は安定していなかったか、後続変更で失われた可能性が高い。
- `array_3` と `array_mixed` も両方悪化しているため、現時点の subset ベンチは mixed というより regression signal と読むべき状況になっている。

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）（mean）

| 入力 | 時間 |
|------|------|
| `42` | 166 ns |
| `1 + 2` | 268 ns |
| `1 + 2 * 3` | 299 ns |
| `(1 + 2) * 3` | 487 ns |
| `1 + 2 * (3 - 4) + 5` | 719 ns |
| `(((1 + 2) * 3) - 4) / 5` | 1,044 ns |
| `1 + 2 + ... + 8` | 862 ns |

**所見:**
- 単一整数で ~166ns は依然かなり重い。`recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` 経由の間接呼び出し + `whitespace0` のオーバーヘッド。
- 括弧のネストごとにおおむね ~200ns 追加されており、再帰 1 段の `Box<dyn Parser>` コストと整合する。
- 8項の加算チェーンは ~0.86µs。主ボトルネックは引き続き `chainl1` ループ自体ではない。

### JSON フルベンチ（107KB sample.json）

この節は `cargo bench -p oni-comb-parser --bench json_full` の結果。

2026-03-21 に同一マシンで 107KB JSON ファイルを再計測（100 サンプル）。

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| oni-comb | 238.7 µs | 427.7 |
| **winnow** | **175.0 µs** | **583.3** |
| nom | 283.2 µs | 360.5 |
| chumsky | 508.7 µs | 200.6 |
| pom | 7.63 ms | 13.4 |

**今回の再計測では JSON フルベンチの首位は `winnow` になった。oni-comb は nom の 1.19 倍、chumsky の 2.13 倍、pom の 32.0 倍のスループットを維持しているが、`winnow` 1.0.0 比では 0.73 倍にとどまる。** 現在の順位でも、oni-comb 側の優位性は次の設計要素に支えられているが、107KB の実運用寄り JSON では以前のマクロベンチ首位は維持できていない:
- `fn_parser` による関数再帰（`recursive()` の `Box<dyn Parser>` vtable を排除）
- `peek_byte` による先頭バイト分岐（`or` チェーンの線形スキャンを排除）
- `quoted_string` によるゼロコピー文字列（エスケープなし文字列は `&str` スライス）
- `take_while1` による数値パースのゼロコピー化
- `StrInputStream` の ASCII fast path
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

### Generic InputStream リファクタリングの影響（InputStream トレイトジェネリック化）

`InputStream` トレイトに `Token`/`Slice` associated type を追加し、`satisfy`, `take_while0/1`, `take`, `take_while_n_m`, `eof` をジェネリックな `primitive/` モジュールに移動。`ByteInputStream<'a>`（`&[u8]` パース用）も新規追加。

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

**原因**: 旧 `text/` 実装は `remaining.chars()` で1回イテレーションし最後に `advance(consumed)` を呼んでいた。ジェネリック `primitive/` 実装はトークン毎に `peek_token()` + `next_token()` を呼び、各呼び出しで `&self.src[self.offset..]` の再計算と `.chars().next()` が発生する。これはジェネリシティのコストであり、`InputStream` トレイトではバッチ文字イテレーターを公開できない。

**その後の回復（2026-03-18 fast path パス）**: `StrInputStream` への ASCII fast path 追加と、generic primitive の `peek_token()` + `next_token()` を `next_token()` + mismatch 時 `reset()` に置き換えたことで、この回帰の大半を ASCII 中心ワークロードでは回収した。現在の mean は identifier `"foo_bar_123"` が 18.6ns、integer `"184467...615"` が 20.0ns まで戻っている。

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

- **マクロベンチの首位は `winnow`** — 107KB JSON 再計測で 583.3 MiB/s、oni-comb は 427.7 MiB/s
- **oni-comb は JSON フルで nom / chumsky / pom を上回る** — nom の 1.19 倍、chumsky の 2.13 倍、pom の 32.0 倍
- **generic token パーサーは依然 competitive だが、今回の再計測は前回より厳しい** — identifier 11B: oni-comb 20.0ns vs winnow 20.5ns / nom 33.3ns、integer 20B: oni-comb 22.8ns vs winnow 22.7ns / nom 22.5ns
- **chumsky 0.12 は旧版より大幅改善したまま** — 短い identifier は今も oni-comb と近い（`"x"`: 16.6ns vs 14.6ns）が、中〜長入力ではなお差がある（`"foo_bar_123"`: 85.0ns vs 20.0ns）
- **flat_map は依然として最速勢に差がある** — とくに同一型分岐（`"1one"`: oni-comb 10.6ns vs winnow 2.4ns / nom 2.6ns）
- **zip と flat_map は同レンジに収まる** — 具象コンビネータ型設計の妥当性を確認
- **今回の `comparison` 再計測では JSON subset 全体が悪化し、新しい `json_full` 再計測でも以前のマクロベンチ首位は消えた** — subset の object-heavy ケース (`object`: ~704ns, `object_large`: ~1.46µs) だけでなく、107KB 実入力でも `winnow` が上回っている
- **Applicative コンビネータでヒープアロケーションゼロ**
