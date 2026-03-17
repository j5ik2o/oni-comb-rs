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

Rust では `flat_map` のクロージャが異なる型のパーサーを返す場合、`Box<dyn Parser>` による型消去が必要になり、ヒープアロケーション＋動的ディスパッチが発生します。旧 v1 や pom は全コンビネータを `Rc<dyn Fn>` で構成しており、ベンチマークでは v2 の 3〜30 倍遅い結果になっています。

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
| `quoted_string()` | ダブルクォート文字列（JSON 準拠エスケープ） | `String` |
| `quoted_string_cow()` | ゼロコピー版 quoted_string（エスケープなしなら借用） | `Cow<'a, str>` |
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
| `.sep_by0(sep)` | — | 区切り付き 0回以上の繰り返し |
| `.sep_by1(sep)` | — | 区切り付き 1回以上の繰り返し |
| `.chainl1(op)` | — | 左結合の二項演算子チェーン |
| `.chainr1(op)` | — | 右結合の二項演算子チェーン |

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

chumsky 0.12 で v0.9 比 ~54 倍の劇的改善（identifier "x": 918ns -> 17.1ns）。短い入力では oni-comb/winnow/nom と競合するレベルに到達。ただし中〜長入力では依然 2 倍程度遅い。

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"x"` (1B) | 16.6 ns | 15.5 ns | 14.9 ns | 17.1 ns | 66.3 ns |
| `"foo_bar_123"` (11B) | 38.9 ns | 21.7 ns | 33.4 ns | 83.8 ns | 230 ns |
| `"longIdentifier..."` (28B) | 81.7 ns | 34.2 ns | 82.7 ns | 132.0 ns | 266.5 ns |

### Token ワークロード結果（Integer）（mean）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 8.8 ns | 3.8 ns | 3.8 ns | 77.2 ns | 17.2 ns |
| `"9999999"` (7B) | 20.3 ns | 5.7 ns | 5.8 ns | 136 ns | 32.3 ns |
| `"184467...615"` (20B) | 62.4 ns | 24.1 ns | 23.4 ns | 253 ns | 86.2 ns |

### flat_map ワークロード結果

#### 同一型分岐（digit → tag、Box 不要）（mean）

| 入力 | oni-comb | winnow | nom | chumsky | pom |
|------|----------|--------|-----|---------|-----|
| `"1one"` | 7.2 ns | 2.7 ns | 2.4 ns | 49.7 ns | 69.4 ns |
| `"3three"` | 5.9 ns | 2.6 ns | 2.4 ns | 51.9 ns | 94.7 ns |

ParseError 導入 + `#[inline]` で旧 8.3ns → 7.2ns。chumsky 0.12 は ~930ns から ~50ns に改善。

#### 異種型分岐（`Box<dyn Parser>` / 動的ディスパッチ）（mean）

| 入力 | oni-comb | winnow | nom\* | chumsky | pom |
|------|----------|--------|-------|---------|-----|
| `"c:hello"` | 30.9 ns | 20.1 ns | 3.7 ns | 25.4 ns | 163.7 ns |
| `"i:42"` | 23.8 ns | 18.0 ns | 3.0 ns | 19.7 ns | 111.0 ns |

\* nom は `Parser` trait が dyn 非互換のため手動二段パース（Box なし）。

#### zip vs flat_map（oni-comb-rs 内部比較）（mean）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| `"x"` | 4.3 ns | 4.2 ns | ≈0% (誤差) |
| `"foo_bar_123"` | 26.0 ns | 25.9 ns | ≈0% (誤差) |
| `"longIdentifier..."` | 64.9 ns | 64.7 ns | ≈0% (誤差) |

### JSON subset（oni-comb のみ）（mean）

| 入力 | 時間 |
|------|------|
| `null` | 8.5 ns |
| `42` | 83.4 ns |
| `"hello world"` | 143.8 ns |
| `[1, 2, 3]` | 517.6 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 663.5 ns |

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）（mean）

| 入力 | 時間 |
|------|------|
| `42` | 155 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 443 ns |
| `(((1 + 2) * 3) - 4) / 5` | 905 ns |

### JSON フルベンチ（107KB）

同一マシンでの計測（100 サンプル）。pom は除外（pom 3.x の API ではフル JSON パーサーの実装が困難）。

| ライブラリ | Mean | Throughput (mean) |
|-----------|------|-------------------|
| **oni-comb** | **196.5 µs** | **519 MB/s** |
| winnow | 201.0 µs | 508 MB/s |
| nom | 274.5 µs | 372 MB/s |
| chumsky | 495.7 µs | 206 MB/s |

`fn_parser` による関数再帰 + `peek_byte` 先頭バイト分岐 + `quoted_string_cow` ゼロコピーにより、winnow の 1.03 倍、nom の 1.40 倍、chumsky の 2.52 倍。

### 特性まとめ

- **winnow を上回るスループット** — 107KB JSON で winnow の 1.06 倍（`fn_parser` + `peek_byte` 分岐 + ゼロコピー文字列）
- **nom と中〜長入力で同等〜上回る** — identifier 11B/28B でほぼ同等
- **pom の 3〜30 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky 0.12 で大幅改善** — identifier "x": 918ns -> 17.1ns（v0.9 比 ~54 倍高速化）。短い入力では競合レベルに到達。ただし中〜長入力では依然 ~2 倍遅い
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計によりモナディック合成でもゼロコスト
- **3 回の最適化で累計 ~83% 改善** — ParseError 導入（~12%）+ `#[inline]`（~17%）+ ゼロコピー＋fn再帰（~77%）
- **JSON/arithmetic ワークロードで 2-5% 改善** — 全ワークロードで継続的な微改善
- **Applicative / flat_map 同一型でヒープアロケーションゼロ** — dhat で 0 bytes / 0 blocks 確認
- 詳細な考察は [`modules/parser/benches/README.md`](modules/parser/benches/README.md) を参照

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
| [oni-comb-parser](modules/parser/) | コアパーサーコンビネータライブラリ |
| [oni-comb-crond](modules/crond/) | cron 式パーサー＆スケジューラー |
| [oni-comb-uri](modules/uri/) | RFC 3986 URI パーサー（ゼロコピー、URN サポート） |

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
