# oni-comb-rs (v2/reboot)

[![Workflow Status](https://github.com/j5ik2o/oni-comb-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/oni-comb-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/oni-comb-parser-rs.svg)](https://crates.io/crates/oni-comb-parser-rs)
[![docs.rs](https://docs.rs/oni-comb-parser-rs/badge.svg)](https://docs.rs/oni-comb-parser-rs)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/j5ik2o/oni-comb-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![dependency status](https://deps.rs/repo/github/j5ik2o/oni-comb-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/oni-comb-rs)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-APACHE2.0-blue.svg)](https://opensource.org/licenses/apache-2-0)

[英語](README.md)

Rust 製パーサーモナドライブラリ（**v2/リブート版**）。

旧 v1 の `Rc<dyn Fn>` ベース設計を捨て、**trait + 具象コンビネータ型**（`Map`, `Zip`, `Or`, `FlatMap` 等）で構成。Functor / Applicative / Alternative / Monad の全階層を提供しつつ、動的ディスパッチ・ヒープ確保を最小化する設計です。

## Quickstart

```rust
use oni_comb_parser::prelude::*;

// 'a' または 'b' にマッチ
let mut parser = char('a').or(char('b'));
let mut input = StrInput::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// identifier: 先頭が英字/_, 以降は英数字/_
let mut ident = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'));
let mut input = StrInput::new("foo_bar_123");
let (head, tail) = ident.parse_next(&mut input).unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_bar_123");

// integer
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInput::new("42");
assert_eq!(int_parser.parse_next(&mut input).unwrap(), 42);
```

## 設計の特徴

- **Parsec スタイルの再帰下降パーサー** — デフォルト LL(1) で `attempt` により LL(\*) に拡張可能。`cut` でコミットしエラー報告を改善。`flat_map` で文脈依存の分岐にも対応
- **パーサーモナド** — Functor (`map`) / Applicative (`zip`) / Alternative (`or`) / Monad (`flat_map`) の全階層を提供
- **ゼロコストなコンビネータ合成** — Applicative コンビネータは具象型でスタック上に構築され、ヒープアロケーションはゼロ。`flat_map` も同一型分岐ならゼロコスト
- **Backtrack / Cut によるエラー制御** — `or` は `Backtrack` のみリカバリし、`Cut` はそのまま伝播。`attempt` で Cut→Backtrack 降格、`cut` で Backtrack→Cut 昇格
- **再帰は boxed recursion** — 再帰の結び目だけ `Box<dyn Parser>` に落とし、非再帰部分は具象型を維持

### 型クラス階層とコスト

| 操作 | 型クラス | Rust での型 | コスト |
|------|---------|------------|--------|
| `p.map(f)` | Functor | `Map<P, F>` | ゼロ |
| `p1.zip(p2)` | Applicative | `Zip<P1, P2>` | ゼロ |
| `p1.zip_left(p2)` | Applicative | `ZipLeft<P1, P2>` | ゼロ |
| `p1.zip_right(p2)` | Applicative | `ZipRight<P1, P2>` | ゼロ |
| `p1.or(p2)` | Alternative | `Or<P1, P2>` | ゼロ |
| `p.flat_map(f)` 同一型分岐 | Monad | `FlatMap<P, F>` | ゼロ |
| `p.flat_map(f)` 異種型分岐 | Monad | `FlatMap<P, F>` + `Box<dyn Parser>` | Box 1回 |

全パーサーで `.flat_map()` が使えますが、**性能を最大化するには Applicative コンビネータ（`zip`, `map`, `or`）を優先**し、`flat_map` は文脈依存の分岐が必要な場面で使います。これは Haskell の Parsec でも同様の推奨事項です。

### なぜ Applicative 優先か

Rust では `flat_map` のクロージャが異なる型のパーサーを返す場合、`Box<dyn Parser>` による型消去が必要になり、ヒープアロケーション＋動的ディスパッチが発生します。旧 v1 や pom は全コンビネータを `Rc<dyn Fn>` で構成しており、ベンチマークでは v2 の 3〜39 倍遅い結果になっています。

一方、`zip`（Applicative）は `Zip<Char, Tag>` のような具象型としてスタック上に構築されるため、コンパイラがモノモーフィゼーション → インライン化 → LLVM 最適化まで一気通貫で行え、手書きの再帰下降パーサーに近い性能が出ます。

```rust
// Applicative: 構造がコンパイル時に確定 → インライン化可能
let parser = char('a').zip(char('b'));   // Zip<Char, Char> — 具象型

// Monad (同一型): Box 不要、ゼロコスト
let parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|n| match n {
    '1' => tag("one"),
    _ => tag("other"),
});

// Monad (異種型): Box<dyn Parser> で型消去
let parser = satisfy(|c: char| c == 'c' || c == 't')
    .flat_map(|c| -> Box<dyn Parser<StrInput<'_>, Output = &str, Error = String>> {
        match c {
            'c' => Box::new(tag("har")),
            _ => Box::new(take_while1(|c: char| c.is_ascii_digit())),
        }
    });
```

## 利用可能なパーサー

### テキストパーサー（`text` モジュール）

| 関数 | 説明 | 戻り値 |
|------|------|--------|
| `char(c)` | 指定した1文字にマッチ | `char` |
| `tag(s)` | 指定した文字列にマッチ | `&str` |
| `satisfy(f)` | 述語を満たす1文字にマッチ | `char` |
| `take_while0(f)` | 述語を満たす文字を0個以上消費 | `&str` |
| `take_while1(f)` | 述語を満たす文字を1個以上消費 | `&str` |
| `eof()` | 入力の終端にマッチ | `()` |
| `whitespace0()` | ASCII 空白を 0 個以上消費 | `&str` |
| `whitespace1()` | ASCII 空白を 1 個以上消費 | `&str` |
| `identifier()` | ASCII 識別子（`[a-zA-Z_][a-zA-Z0-9_]*`） | `&str` |
| `integer()` | 符号付き整数 | `i64` |
| `quoted_string()` | ダブルクォート文字列（JSON 準拠エスケープ、エスケープなしなら借用） | `Cow<'a, str>` |
| `escaped(open, close, esc, handler)` | 汎用エスケープ文字列パーサー | `String` |
| `lexeme(p)` | パーサー実行後に後続の空白を消費 | `P::Output` |
| `between(l, p, r)` | `l`, `p`, `r` を順に実行し `p` の値を返す | `P::Output` |
| `recursive(f)` | 再帰パーサーを構築（クロージャ内で再帰参照を受け取る） | `P::Output` |
| `fn_parser(f)` | 関数ポインタを `Parser` にラップ（vtable 不要の再帰に最適） | `O` |

### コンビネータ（`ParserExt` メソッドチェーン）

| メソッド | 型クラス | 説明 |
|----------|---------|------|
| `.map(f)` | Functor | 成功値を変換 |
| `.zip(p)` | Applicative | 2つのパーサーを順次適用し、ペアを返す |
| `.zip_left(p)` | Applicative | 両方実行し、左の値だけを返す（= terminated） |
| `.zip_right(p)` | Applicative | 両方実行し、右の値だけを返す（= preceded） |
| `.or(p)` | Alternative | 左が Backtrack なら右を試行 |
| `.flat_map(f)` | Monad | 1つ目の結果に基づいて次のパーサーを動的に選択 |
| `.attempt()` | — | Cut を Backtrack に降格（巻き戻し可能にする） |
| `.cut()` | — | Backtrack を Cut に昇格（or での分岐を禁止する） |
| `.optional()` | — | Backtrack を `None` に変換 |
| `.many0()` | — | 0回以上の繰り返し |
| `.many1()` | — | 1回以上の繰り返し |
| `.many0_fold(init, f)` | — | 0個以上の要素を畳み込み（ゼロアロケーション） |
| `.many1_fold(init, f)` | — | 1個以上の要素を畳み込み（ゼロアロケーション） |
| `.many0_into(container)` | — | 0個以上の要素をユーザー指定コンテナ（`Extend`）に収集 |
| `.many1_into(container)` | — | 1個以上の要素をユーザー指定コンテナ（`Extend`）に収集 |
| `.sep_by0(sep)` | — | 区切り付き 0回以上の繰り返し |
| `.sep_by1(sep)` | — | 区切り付き 1回以上の繰り返し |
| `.sep_by0_fold(sep, init, f)` | — | 区切り付き 0個以上の要素を畳み込み（ゼロアロケーション） |
| `.sep_by1_fold(sep, init, f)` | — | 区切り付き 1個以上の要素を畳み込み（ゼロアロケーション） |
| `.sep_by0_into(sep, container)` | — | 区切り付き 0個以上の要素をユーザー指定コンテナに収集 |
| `.sep_by1_into(sep, container)` | — | 区切り付き 1個以上の要素をユーザー指定コンテナに収集 |
| `.chainl1(op)` | — | 左結合の二項演算子チェーン |
| `.chainr1(op)` | — | 右結合の二項演算子チェーン |
| `.context(label)` | — | エラーコンテキストラベル追加 |
| `.map_res(f, label)` | — | 失敗しうる関数で変換 |

## ベンチマーク

Criterion.rs による他ライブラリとの比較ベンチマークを同梱しています。

### 比較対象

| ライブラリ | 設計 |
|-----------|------|
| **winnow** | 最速クラス。`Parser` trait + `parse_next(&mut I)` で oni-comb-rs と最も設計が近い |
| **nom** | デファクト標準。関数ポインタベース |
| **chumsky** | エラーリカバリ特化。trait ベースのコンビネータ |
| **pom** | 演算子オーバーロード中心。旧 v1 に近い設計 |

### 機能比較

| 評価項目 | oni-comb | winnow | nom | chumsky | pom |
|---------|:--------:|:------:|:---:|:-------:|:---:|
| **メソッドチェーン API** (`p1.zip(p2)`) | o | o | x | o | x |
| **パーサーモナド** (Functor/Applicative/Monad 全階層) | o | x | x | x | o |
| **Applicative 合成でヒープ確保ゼロ** | o | o | o | x | x |
| **flat_map 同一型がゼロコスト** | o | o | o | x | x |
| **構造化エラー** (位置・期待トークン) | o | o | △ | o | x |
| **Backtrack / Cut の明示的制御** | o | o | o | x | x |
| **`.context()` ラベル付け** | o | o | △ | o | x |
| **`recursive()` ヘルパー** | o | x | x | o | x |
| **`chainl1` / `chainr1`** (演算子結合) | o | x | x | x | x |
| **`sep_by` / `between`** | o | o | o | o | o |
| **`no_std` 対応** (`alloc` 使用) | o | o | o | x | x |

- o = サポート、△ = 部分的（VerboseError 等で追加対応が必要）、x = 未サポートまたは設計上不可

**oni-comb-rs の立ち位置**: winnow/nom 並みのゼロコスト性能と、chumsky 並みのメソッドチェーン体験を両立。さらに Monad 階層（`flat_map`）と `chainl1`/`recursive()` を提供する唯一のライブラリ。

※ 以下の数値はすべて Criterion 報告の **mean 推定値**（100 サンプル、95% 信頼区間中央）。

### Token ワークロード結果（Identifier）（mean）

2026-03-18 の再計測では、generic identifier パスの回復により、以下の中〜長 ASCII 入力では oni-comb が `winnow` を上回るまで戻った。chumsky 0.12 は旧版より大幅改善したままだが、実用的な長さではまだ差がある。

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 15.0 ns | 15.2 ns | 15.1 ns | 17.5 ns | 68.2 ns |
| `"foo_bar_123"` (11B) | 18.6 ns | 20.3 ns | 33.2 ns | 86.9 ns | 202.0 ns |
| `"longIdentifier..."` (28B) | 30.2 ns | 33.2 ns | 86.5 ns | 132.7 ns | 269.7 ns |

### Token ワークロード結果（Integer）（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 2.6 ns | 2.8 ns | 2.8 ns | 70.8 ns | 20.7 ns |
| `"9999999"` (7B) | 5.2 ns | 5.4 ns | 5.4 ns | 132.3 ns | 29.5 ns |
| `"184467...615"` (20B) | 20.0 ns | 23.1 ns | 22.7 ns | 273.1 ns | 100.1 ns |

### flat_map ワークロード結果

#### 同一型分岐（digit → tag、Box 不要）（mean）

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"1one"` | 5.7 ns | 2.6 ns | 3.2 ns | 72.8 ns | 76.2 ns |
| `"3three"` | 4.1 ns | 2.7 ns | 2.4 ns | 51.5 ns | 95.0 ns |

ParseError 導入、`#[inline]`、さらに generic token fast path の整理により、旧 8ns 台からさらに短縮した。残差は主に branch dispatch のオーバーヘッド。

#### 異種型分岐（`Box<dyn Parser>` / 動的ディスパッチ）（mean）

| 入力 | oni-comb | winnow | nom\* | chumsky | pom |
|------|----------|--------|-------|---------|-----|
| `"c:hello"` | 20.7 ns | 19.2 ns | 4.6 ns | 41.4 ns | 307.5 ns |
| `"i:42"` | 18.3 ns | 17.7 ns | 2.8 ns | 24.2 ns | 114.2 ns |

\* nom は `Parser` trait が dyn 非互換のため手動二段パース（Box なし）。

#### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| `"x"` | 2.3 ns | 1.9 ns | 同レンジ |
| `"foo_bar_123"` | 6.7 ns | 6.6 ns | 同レンジ |
| `"longIdentifier..."` | 15.6 ns | 15.4 ns | 同レンジ |

### JSON subset（oni-comb のみ）（mean）

| 入力 | 時間 |
|------|------|
| `null` | 11.5 ns |
| `42` | 88.7 ns |
| `"hello world"` | 129.1 ns |
| `[1, 2, 3]` | 494.3 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 596.5 ns |

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）（mean）

| 入力 | 時間 |
|------|------|
| `42` | 151 ns |
| `1 + 2 * 3` | 265 ns |
| `(1 + 2) * 3` | 429 ns |
| `(((1 + 2) * 3) - 4) / 5` | 912 ns |

### JSON フルベンチ（107KB）

2026-03-18 に同一マシンで再計測（100 サンプル）。ベンチ基準の `winnow` も 0.7 から 1.0.0 に更新済み。
計測マシン: Mac mini (Mac16,11), Apple M4 Pro (14 cores: 10P + 4E), 64 GB RAM, macOS 26.3.1, arm64.

| ライブラリ | Mean | Throughput (mean, MiB/s) |
|-----------|------|-------------------------|
| **oni-comb** | **109.5 µs** | **932.1** |
| winnow | 178.7 µs | 571.3 |
| nom | 282.8 µs | 360.9 |
| chumsky | 561.0 µs | 181.9 |
| pom | 7.69 ms | 13.3 |

今回の再計測では oni-comb が JSON フルベンチ首位をさらに広げた。`winnow` 1.0.0 の 1.63 倍、nom の 2.58 倍、chumsky の 5.12 倍、pom の 70.2 倍のスループットに到達している。

### 特性まとめ

- **oni-comb が JSON フルのマクロベンチ首位をさらに広げた** — 932.1 MiB/s、winnow は 571.3 MiB/s
- **oni-comb は JSON フルで nom / chumsky / pom に大差** — nom の 2.58 倍、chumsky の 5.12 倍、pom の 70.2 倍
- **generic identifier / integer はもはや最大の弱点ではない** — この再計測では掲載ケースで oni-comb が winnow を上回る
- **chumsky 0.12 で大幅改善** — 短い identifier は今も oni-comb に近いが、中〜長入力では依然差がある
- **flat_map が現時点の最大の microbenchmark ギャップ** — とくに同一型分岐で winnow / nom に差がある
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計により両者は同じ性能レンジに収まる
- **今回の whitespace リファクタは JSON subset では mixed** — primitive 寄りは悪化したが、object は ~596.5ns まで改善し、full JSON は前進した
- **Applicative / flat_map 同一型でヒープアロケーションゼロ** — dhat で 0 bytes / 0 blocks 確認
- 詳細な考察は [`modules/parser/benches/README.ja.md`](modules/parser/benches/README.ja.md) を参照

### ベンチマーク実行

```bash
# 比較ベンチマーク
cargo bench -p oni-comb-parser --bench comparison

# JSON / arithmetic ベンチ
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# アロケーション計測
cargo bench -p oni-comb-parser --bench alloc_count
```

## クレート一覧

| クレート | 説明 |
|---------|------|
| [oni-comb-parser](modules/parser/README.ja.md) | コアパーサーコンビネータライブラリ |
| [oni-comb-crond](modules/crond/README.ja.md) | cron 式パーサー＆スケジューラー |
| [oni-comb-uri](modules/uri/README.ja.md) | RFC 3986 URI パーサー（ゼロコピー、URN サポート） |

## ビルド・テスト

```bash
# ビルド
cargo build

# 全テスト実行
cargo test -p oni-comb-parser

# 特定テスト実行
cargo test -p oni-comb-parser -- test_name
```

## ロードマップ

| MS | 名前 | 状態 | 内容 |
|----|------|------|------|
| 1 | Core | **完了** | Input, Fail, PResult, Parser, ParserExt, StrInput |
| 2 | Primitive | **完了** | eof, char, tag, satisfy, take_while0/1, peek |
| 3 | Combinators | **完了** | map, zip, zip_left, zip_right, between, or, attempt, cut, optional, many0/1, sep_by0/1, chainl1/r1, flat_map/and_then |
| 4 | Text module | **完了** | whitespace0/1, identifier, integer, quoted_string, escaped, lexeme。JSON subset・URI tokenizer テストで実証済み |
| 5 | Recursive | **完了** | `recursive()` ヘルパー（`Rc<UnsafeCell<Box<dyn Parser>>>`）。四則演算+括弧テストで実証済み |
| 6 | Error reporting | **完了** | `ParseError`（位置・期待トークン・コンテキスト）、`or` のマージ、`.context()` コンビネータ |
| 7 | Benchmark | **完了** | 5 ライブラリ比較（token/flat_map）、JSON subset、四則演算、zip vs flat_map、dhat。ParseError 導入で ~12% 改善の最適化サイクル完了 |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
