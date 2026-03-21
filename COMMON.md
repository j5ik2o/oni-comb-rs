- 日本語でやりとりしてください

## プロジェクト概要

oni-comb-rs は Rust 製パーサーコンビネータライブラリの v2 リブート版。旧 v1 の `Rc<dyn Fn>` ベース設計を捨て、trait + 具象コンビネータ型（`Map`, `Zip`, `Or` 等）で構成する。動的ディスパッチ・ヒープ確保を排し、Applicative/Alternative 主体で最適化しやすい設計を目指している。

## ビルド・テスト

```bash
# ビルド
cargo build

# フォーマット
cargo fmt

# 全テスト実行
cargo test

# parser crateのみテスト
cargo test -p oni-comb-parser

# 特定テスト実行
cargo test -p oni-comb-parser -- test_name

# クリーンビルド
cargo clean && cargo build
```

## アーキテクチャ

Cargo workspace 構成。現在のメンバーは `parser` クレートのみ。

### コア型の階層

```
InputStream (trait)    -- 入力ストリーム抽象。Token/Slice/Checkpoint を提供
  ├─ StrInputStream    -- &str 向け実装。Token=char, Slice=&'a str, Checkpoint=usize
  └─ ByteInputStream   -- &[u8] 向け実装。Token=u8, Slice=&'a [u8], Checkpoint=usize

Parser (trait)         -- parse_next(&mut self, &mut I) -> PResult<O, E>
  └─ ParserExt (trait) -- map/zip/zip_left/zip_right/or/attempt/cut/optional/many0/many1/many0_fold/many1_fold/many0_into/many1_into/sep_by0/sep_by1/sep_by0_fold/sep_by1_fold/sep_by0_into/sep_by1_into/chainl1/chainr1/context/map_res/flat_map/and_then のメソッドチェーン

Fail (enum)            -- Backtrack(E) | Cut(E) | Incomplete | ZeroProgress
PResult<T, E>          -- Result<T, Fail<E>>
```

### モジュール構成 (`modules/parser/src/`)

| モジュール | 役割 |
|-----------|------|
| `input_stream.rs` | `InputStream` トレイト（`Token`, `Slice`, `Checkpoint`, `next_token`, `peek_token`, `slice_since`, `reset`, `is_eof`） |
| `str_input_stream.rs` | `StrInputStream<'a>` — `&str` 向け `InputStream` 実装 |
| `byte_input_stream.rs` | `ByteInputStream<'a>` — `&[u8]` 向け `InputStream` 実装 |
| `parser.rs` | `Parser<I>` トレイト（`Output`, `Error`, `parse_next`） |
| `parser_ext.rs` | `ParserExt<I>` — 全 `Parser` に自動実装されるコンビネータメソッド |
| `fail.rs` | `Fail<E>` enum と `PResult` 型エイリアス |
| `combinator/` | 各コンビネータの具象型（`Map`, `Zip`, `ZipLeft`, `ZipRight`, `Or`, `Attempt`, `Cut`, `Optional`, `Many`, `Many1`, `ManyFold`, `Many1Fold`, `SepBy0`, `SepBy1`, `SepByFold0`, `SepByFold1`, `ChainL1`, `ChainR1`, `FlatMap`, `MapRes`, `Context`, `FnParser`, `Recursive`） |
| `primitive/` | ジェネリックパーサー（`Take`, `Satisfy`, `TakeWhile0/1`, `TakeWhileNM`, `Eof`）— `I: InputStream` で `StrInputStream`/`ByteInputStream` 両対応 |
| `text/` | テキスト専用パーサー（`Char`, `Tag`, `Whitespace0/1`, `Identifier`, `Integer`, `QuotedString`）— `StrInputStream` 固定 |

### 設計上の重要な判断

- **Fail::Backtrack vs Fail::Cut**: `or` は Backtrack のみリカバリし、Cut はそのまま伝播。`attempt` は Cut を Backtrack に降格、`cut` は Backtrack を Cut に昇格
- **flat_map は実装済みだが Applicative 優先を推奨**: ベンチマークで zip ≒ flat_map（同一型）を確認済み。ただし異種型分岐では `Box<dyn Parser>` が必要で ~15ns のオーバーヘッドが発生するため、Applicative (`zip`, `or`) を優先し flat_map は文脈依存の分岐に限定する方針
- **再帰は boxed recursion**: 再帰の結び目だけ `Box<dyn Parser>` に落とし、非再帰部分は具象型を維持
- **入力型は `&str` と `&[u8]` をサポート**: `StrInputStream`（Token=char, Slice=&str）と `ByteInputStream`（Token=u8, Slice=&[u8]）。primitive/ のパーサーは両方で動作。text/ は StrInputStream 専用
- **primitive パーサーは PhantomData で InputStream 型を保持**: 型推論のため `Satisfy<F, I>`, `TakeWhile0<F, I>` 等は `PhantomData<fn(&mut I)>` を持つ。prelude は StrInputStream 固定のラッパー関数をエクスポート
- **`many`/`sep_by`/`chainl1` は専用ループコンビネータ**: flat_map 再帰ではなくループで実装

## コンビネータ意味論

各コンビネータの `Fail` に対する振る舞い。実装時はこの仕様に従うこと。

### `or(left, right)`
左を checkpoint 付きで試し、Backtrack なら rewind して右を試行。Cut/Incomplete はそのまま伝播。
```rust
match left.parse_next(input) {
    Ok(v) => Ok(v),
    Err(Fail::Backtrack(_)) => { input.reset(cp); right.parse_next(input) }
    Err(e @ Fail::Cut(_)) => Err(e),
    Err(e @ Fail::Incomplete) => Err(e),
}
```

### `attempt(p)`
`p` 内で起きた Cut を Backtrack に降格し、開始 checkpoint へ戻す。成功時は何もしない。

### `cut(p)`
`p` の Backtrack を Cut に昇格させる。`tag(":").zip(value.cut())` のように使う。

### `optional(p)`
Backtrack のみ `Ok(None)` に変換。Cut/Incomplete は伝播。

### `many0(p)`
Backtrack で停止し収集結果を返す。Cut/Incomplete は伝播。zero-progress（入力を消費せずに成功し続ける）は `ZeroProgress` エラー。

### `sep_by`, `between`, `chainl1`, `chainr1`
flat_map 再帰ではなく専用ループで実装する。

## マイルストーン

| # | 名前 | 実装対象 | まだやらない | 完了条件 |
|---|------|----------|-------------|---------|
| 1 | Core | `InputStream`, `Span`, `Fail`, `PResult`, `Parser`, `ParserExt`, `StrInputStream` | regex, cache, bytes, recursive helper | `or/attempt/cut` の単体テストが通る |
| 2 | Primitive | `eof`, `char`, `tag`, `satisfy`, `take_while0/1`, `peek` | unicode category, regex, bytes | identifier/integer parser が組める |
| 3 | Combinators | `map`, `zip`, `zip_left`, `zip_right`, `between`, `optional`, `many0/1`, `sep_by0/1`, `chainl1`, `chainr1`, `flat_map`/`and_then` | — | expression parser と CSV/JSON subset の骨格が書ける |
| 4 | Text module | whitespace, ascii token, identifier, integer, quoted string | bytes 共通化 | JSON subset と URI tokenizer が動く |
| 5 | Recursive | boxed `recursive()` helper, precedence parser | left recursion, packrat | 四則演算+括弧の parser が動く |
| 6 | Error reporting | span, expected-set, context stack, cut-aware merge | カラー診断, IDE 連携 | JSON subset の失敗位置と期待トークンが出る |
| 7 | Benchmark | criterion bench, allocation counter, regression threshold | micro-opt の先走り | v1 比較でボトルネック定量化、1回最適化サイクル完了 |

## ベンチマーク結果

- **比較対象**: `winnow`、`nom`、`chumsky`、`pom`
- **workload**: identifier/integer、flat_map 同一型/異種型、zip vs flat_map、JSON subset、四則演算+括弧、107KB JSON フル
- **観測項目**: throughput（Criterion）、allocation count（`dhat-rs`）
- **計測マシン**: Mac mini (Mac16,11), Apple M4 Pro (14 cores: 10P + 4E), 64 GB RAM, macOS 26.3.1, arm64
- **最適化サイクル**: ParseError 導入（~12%）+ `#[inline]`（~17%）+ ゼロコピー＋fn再帰（~77%）で累計 ~83% 改善
- **107KB JSON フルベンチ（`json_full`, 100 サンプル、2026-03-18 再計測）**:

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| **oni-comb** | **109.5 µs** | **932.1** |
| winnow | 178.7 µs | 571.3 |
| nom | 282.8 µs | 360.9 |
| chumsky | 561.0 µs | 181.9 |
| pom | 7.69 ms | 13.3 |

- **知見**:
  - 2026-03-18 の `json_full` 再計測では oni-comb が首位で、`winnow` 1.0.0 の 1.63 倍、nom の 2.58 倍、chumsky の 5.12 倍、pom の 70.2 倍のスループットに到達した
  - 一方で 2026-03-21 の `comparison` 再計測では token 系は依然 competitive だが、same-type `flat_map` と JSON subset は前回スナップショットより悪化した
  - `zip ≒ flat_map` という構造的傾向は維持されているが、現在の same-type `flat_map` は `"1one"` で 10.6ns と、以前の ~5ns 台より重い
  - 11B identifier は oni-comb 20.0ns / winnow 20.5ns / nom 33.3ns、20B integer は oni-comb 22.8ns / winnow 22.7ns / nom 22.5ns で、generic token parser はまだ競争力がある
- **Generic InputStream リファクタリングの影響**: 初期の generic primitive 化では `peek_token`+`next_token` の per-token オーバーヘッドにより長い入力で大きな退行があったが、その後の ASCII fast path と consume-then-reset 化で多くを回収した。現在の token ベンチは以前より厳しい再計測結果ではあるものの、旧「identifier 28B で 80ns 台」の状態からは回復している。詳細は `modules/parser/benches/README.md` を参照
- **アロケーション**: パーサーコンビネータインフラはゼロアロケーション。JSON フルパースのアロケーション（743 blocks / 336KB）は全て AST 構築（`Vec` grow + エスケープ文字列 `Cow::Owned`）に起因

## 設計メモ: `no_std` core-only 層

現在 `#![no_std]` + `extern crate alloc` だが、`alloc` なしの `core` のみ層を feature gate で分離可能。

**`alloc` 不要（core のみで動作）**:
`tag`, `char`, `satisfy`, `take_while0/1`, `eof`, `whitespace0/1`, `identifier`, `integer`,
`zip`, `zip_left`, `zip_right`, `map`, `or`, `attempt`, `cut`, `optional`, `context`,
`fn_parser`, `flat_map`（同一型返却時）, `peek_byte`,
`many0_fold/many1_fold`, `sep_by0_fold/sep_by1_fold`（fold 系はゼロアロケーション）

**`alloc` が必要**:
`many0/1`, `sep_by0/1`, `many0_into/many1_into`, `sep_by0_into/sep_by1_into`（`Vec`/`Extend` 返却）,
`chainl1/r1`（`Vec` 返却）, `quoted_string`/`escaped`（`Cow`/`String`）,
`recursive`（`Box<dyn Parser>` + `Rc`）, `ParseError`（`Vec<Expected>` + `Vec<&str>`）

**実装方針**: `default = ["alloc"]` feature で分離。core-only 層だけでプロトコルパーサーや組み込みトークナイザーに使える。
