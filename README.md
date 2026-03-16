# oni-comb-rs

Rust 製パーサーモナドライブラリ（v2 リブート版）。

旧 v1 の `Rc<dyn Fn>` ベース設計を捨て、**trait + 具象コンビネータ型**（`Map`, `Zip`, `Or`, `FlatMap` 等）で構成。Functor / Applicative / Alternative / Monad の全階層を提供しつつ、動的ディスパッチ・ヒープ確保を最小化する設計です。

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

### Token ワークロード結果（Identifier）

入力が長くなるほど oni-comb-rs の `TakeWhile` のバイトスキャンが効き、nom を大きく引き離します。

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"x"` (1B) | 14.9 ns | 15.2 ns | 13.2 ns | 66.4 ns | 894 ns |
| `"foo_bar_123"` (11B) | 26.7 ns | 21.4 ns | 37.8 ns | 199 ns | 1,144 ns |
| `"longIdentifier..."` (28B) | 44.1 ns | 33.6 ns | 82.2 ns | 269 ns | 1,447 ns |

### Token ワークロード結果（Integer）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"42"` (2B) | 3.6 ns | 2.3 ns | 2.7 ns | 72 ns | 907 ns |
| `"9999999"` (7B) | 8.2 ns | 5.2 ns | 5.3 ns | 133 ns | 995 ns |
| `"184467...615"` (20B) | 25.9 ns | 22.6 ns | 22.7 ns | 264 ns | 1,256 ns |

### flat_map ワークロード結果

#### 同一型分岐（digit → tag、Box 不要）

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"1one"` | 6.1 ns | 2.6 ns | 2.4 ns | 70 ns | 892 ns |
| `"3three"` | 4.8 ns | 2.6 ns | 2.4 ns | 94 ns | 929 ns |

ParseError 導入 + `#[inline]` で旧 8.3ns → 6.1ns（累計 ~26% 改善）。

#### 異種型分岐（`Box<dyn Parser>` / 動的ディスパッチ）

| 入力 | oni-comb | winnow | nom\* | pom | chumsky |
|------|----------|--------|-------|-----|---------|
| `"c:hello"` | 21.5 ns | 19.3 ns | 3.9 ns | 164 ns | 1,052 ns |
| `"i:42"` | 19.8 ns | 18.6 ns | 2.8 ns | 109 ns | 972 ns |

\* nom は `Parser` trait が dyn 非互換のため手動二段パース（Box なし）。

#### zip vs flat_map（oni-comb-rs 内部比較）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| `"x"` | 4.8 ns | 4.8 ns | 0% (誤差) |
| `"foo_bar_123"` | 17.7 ns | 17.7 ns | 0% (誤差) |
| `"longIdentifier..."` | 31.2 ns | 31.1 ns | 0% (誤差) |

### JSON subset（oni-comb のみ）

| 入力 | 時間 |
|------|------|
| `null` | 15 ns |
| `42` | 86 ns |
| `"hello world"` | 147 ns |
| `[1, 2, 3]` | 536 ns |
| `{"name":"oni-comb","version":2,"active":true}` | 693 ns |

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）

| 入力 | 時間 |
|------|------|
| `42` | 156 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 440 ns |
| `(((1 + 2) * 3) - 4) / 5` | 931 ns |

### JSON フルベンチ（107KB — chumsky ベンチ互換）

同一マシンでの計測:

| # | ライブラリ | 時間 | スループット |
|---|-----------|------|-------------|
| 1 | **oni-comb** | **109 µs** | **937 MB/s** |
| 2 | winnow | 156 µs | 656 MB/s |
| 3 | nom | 272 µs | 376 MB/s |

`fn_parser` による関数再帰 + `peek_byte` 先頭バイト分岐 + `quoted_string_cow` ゼロコピーにより、winnow を 1.43x 上回る。

### 特性まとめ

- **winnow を上回るスループット** — 107KB JSON で winnow の 1.43 倍（`fn_parser` + `peek_byte` 分岐 + ゼロコピー文字列）
- **nom を中〜長入力で上回る** — 11B で 28% 高速、28B で 46% 高速（identifier）
- **pom の 3〜30 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky の 30〜200 倍高速**
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計によりモナディック合成でもゼロコスト
- **3 回の最適化で累計 ~83% 改善** — ParseError 導入（~12%）+ `#[inline]`（~17%）+ ゼロコピー＋fn再帰（~77%）
- **Applicative / flat_map 同一型でヒープアロケーションゼロ** — dhat で 0 bytes / 0 blocks 確認
- 詳細な考察は [`parser/benches/README.md`](parser/benches/README.md) を参照

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
