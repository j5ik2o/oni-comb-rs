# oni-comb-rs

Rust 製パーサーモナドライブラリ（v2 リブート版）。

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
char('a').zip(char('b'))   // Zip<Char, Char> — 具象型

// Monad (同一型): Box 不要、ゼロコスト
satisfy(|c: char| c.is_ascii_digit()).flat_map(|n| match n {
    '1' => tag("one"),
    _ => tag("other"),
})

// Monad (異種型): Box<dyn Parser> で型消去
satisfy(|c: char| c == 'c' || c == 't')
    .flat_map(|c| -> Box<dyn Parser<StrInput<'_>, Output = &str, Error = String>> {
        match c {
            'c' => Box::new(tag("har")),
            _ => Box::new(take_while1(|c: char| c.is_ascii_digit())),
        }
    })
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
| `escaped(open, close, esc, handler)` | 汎用エスケープ文字列パーサー | `String` |
| `lexeme(p)` | パーサー実行後に後続の空白を消費 | `P::Output` |
| `between(l, p, r)` | `l`, `p`, `r` を順に実行し `p` の値を返す | `P::Output` |

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

### Token ワークロード結果（Identifier）

入力が長くなるほど oni-comb-rs の `TakeWhile` のバイトスキャンが効き、nom を大きく引き離します。

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"x"` (1B) | 18.3 ns | 15.1 ns | 13.5 ns | 66.8 ns | 886 ns |
| `"foo_bar_123"` (11B) | 27.5 ns | 21.2 ns | 37.7 ns | 212 ns | 1,131 ns |
| `"longIdentifier..."` (28B) | 45.6 ns | 33.4 ns | 86.8 ns | 278 ns | 1,413 ns |

### Token ワークロード結果（Integer）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 3.9 ns | 2.6 ns | 3.1 ns | 73 ns | 898 ns |
| `"9999999"` (7B) | 8.2 ns | 5.3 ns | 7.2 ns | 138 ns | 1,033 ns |
| `"184467...615"` (20B) | 25.8 ns | 23.0 ns | 22.8 ns | 260 ns | 1,328 ns |

### flat_map ワークロード結果

#### 同一型分岐（digit → tag、Box 不要）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"1one"` | 8.3 ns | 2.6 ns | 2.4 ns | 69 ns | 924 ns |
| `"3three"` | 6.9 ns | 2.3 ns | 2.3 ns | 93 ns | 983 ns |

#### 異種型分岐（`Box<dyn Parser>` / 動的ディスパッチ）

| 入力 | oni-comb | winnow | nom\* | pom | chumsky |
|------|----------|--------|-------|-----|---------|
| `"c:hello"` | 21.9 ns | 19.4 ns | 3.8 ns | 160 ns | 1,139 ns |
| `"i:42"` | 20.9 ns | 18.8 ns | 2.7 ns | 110 ns | 1,053 ns |

\* nom は `Parser` trait が dyn 非互換のため手動二段パース（Box なし）。他ライブラリとは条件が異なる。

#### zip vs flat_map（oni-comb-rs 内部比較）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| `"x"` | 4.7 ns | 4.9 ns | +4% (誤差) |
| `"foo_bar_123"` | 17.5 ns | 17.3 ns | -1% (誤差) |
| `"longIdentifier..."` | 30.7 ns | 31.0 ns | +1% (誤差) |

**zip と flat_map のオーバーヘッド差はほぼゼロ。** 同一型を返す限り、具象コンビネータ型の恩恵で LLVM が同等に最適化する。

### 特性まとめ

- **winnow の 70-90% のスループット** — 具象型化の恩恵で改善余地あり
- **nom を中〜長入力で上回る** — 11B で 37% 高速、28B で 90% 高速（identifier）
- **pom の 3〜30 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky の 30〜230 倍高速**
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計によりモナディック合成でもゼロコスト
- **Box\<dyn Parser\> のオーバーヘッドは ~15ns** — 再帰パーサー設計時の見積もり基準値
- **Applicative / flat_map 同一型でヒープアロケーションゼロ** — dhat による計測で 0 bytes / 0 blocks を確認
- 詳細な考察は [`parser/benches/README.md`](parser/benches/README.md) を参照

### ベンチマーク実行

```bash
# 比較ベンチマーク
cargo bench -p oni-comb-parser --bench comparison

# flat_map ベンチのみ
cargo bench -p oni-comb-parser --bench comparison -- flat_map

# zip vs flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs

# アロケーション計測
cargo bench -p oni-comb-parser --bench alloc_count
```

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
| 5 | Recursive | 未着手 | boxed `recursive()` helper, precedence parser |
| 6 | Error reporting | 未着手 | span, expected-set, context stack |
| 7 | Benchmark | **進行中** | identifier/integer/flat_map の 5 ライブラリ比較、zip vs flat_map 内部比較、dhat アロケーション計測 |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
