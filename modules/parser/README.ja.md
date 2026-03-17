# oni-comb-parser

[English](README.md)

[oni-comb-rs](../../) のコアクレート。Rust 製パーサーモナドコンビネータライブラリ。

## 特徴

- **Parsec スタイルの再帰下降** — デフォルト LL(1)、`attempt` で LL(*)
- **完全な型クラス階層** — Functor (`map`) / Applicative (`zip`) / Alternative (`or`) / Monad (`flat_map`)
- **ゼロコストなコンビネータ合成** — Applicative コンビネータは具象型でスタック上に構築、ヒープアロケーションゼロ
- **Backtrack / Cut エラー制御** — `or` は `Backtrack` のみリカバリ。`Cut` は伝播。`attempt` / `cut` で制御
- **構造化エラー** — `ParseError`（位置・期待トークン・`.context()` ラベル）
- **ジェネリック Input トレイト** — `StrInput<'a>`（`&str`）、`ByteInput<'a>`（`&[u8]`）
- **`no_std` 対応** — `#![no_std]` + `alloc`

## クイックスタート

```rust
use oni_comb_parser::prelude::*;

// 'a' または 'b' にマッチ
let mut parser = char('a').or(char('b'));
let mut input = StrInput::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// 識別子: 先頭が英字/_, 以降は英数字/_
let mut input = StrInput::new("foo_123");
let (head, tail) = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'))
    .parse_next(&mut input)
    .unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_123");

// 整数
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInput::new("42");
assert_eq!(int_parser.parse_next(&mut input).unwrap(), 42);
```

## 利用可能なパーサー

### テキストパーサー

| 関数 | 説明 | 戻り値 |
|------|------|--------|
| `char(c)` | 指定した1文字にマッチ | `char` |
| `tag(s)` | 指定した文字列にマッチ | `&str` |
| `satisfy(f)` | 述語を満たす1文字にマッチ | `char` |
| `take_while0(f)` / `take_while1(f)` | 述語を満たす文字を消費 | `&str` |
| `eof()` | 入力の終端にマッチ | `()` |
| `whitespace0()` / `whitespace1()` | ASCII 空白を消費 | `&str` |
| `identifier()` | ASCII 識別子 `[a-zA-Z_][a-zA-Z0-9_]*` | `&str` |
| `integer()` | 符号付き整数 | `i64` |
| `quoted_string()` | JSON 準拠ダブルクォート文字列 | `String` |
| `quoted_string_cow()` | ゼロコピー版 quoted string | `Cow<'a, str>` |
| `escaped(open, close, esc, handler)` | 汎用エスケープ文字列 | `String` |
| `lexeme(p)` | パーサー実行後に後続の空白を消費 | `P::Output` |
| `between(l, p, r)` | l, p, r を順に実行し p の値を返す | `P::Output` |
| `recursive(f)` | 再帰パーサーを構築 | `P::Output` |
| `fn_parser(f)` | 関数を Parser にラップ | `O` |

### コンビネータ（ParserExt）

| メソッド | 型クラス | 説明 |
|---------|---------|------|
| `.map(f)` | Functor | 成功値を変換 |
| `.zip(p)` | Applicative | 2つのパーサーを順次適用、ペアを返す |
| `.zip_left(p)` | Applicative | 両方実行、左の値だけ返す |
| `.zip_right(p)` | Applicative | 両方実行、右の値だけ返す |
| `.or(p)` | Alternative | 左が Backtrack なら右を試行 |
| `.flat_map(f)` | Monad | 文脈依存の分岐 |
| `.attempt()` | — | Cut を Backtrack に降格 |
| `.cut()` | — | Backtrack を Cut に昇格 |
| `.optional()` | — | Backtrack を None に変換 |
| `.many0()` / `.many1()` | — | 0回以上 / 1回以上の繰り返し |
| `.sep_by0(sep)` / `.sep_by1(sep)` | — | 区切り付き繰り返し |
| `.chainl1(op)` / `.chainr1(op)` | — | 演算子結合チェーン |
| `.context(label)` | — | エラーコンテキストラベル追加 |

## 入力型

| 型 | Token | Slice | 用途 |
|----|-------|-------|------|
| `StrInput<'a>` | `char` | `&'a str` | テキストパース（デフォルト） |
| `ByteInput<'a>` | `u8` | `&'a [u8]` | バイナリプロトコルパース |

## ビルド・テスト

```bash
cargo build -p oni-comb-parser
cargo test -p oni-comb-parser

# ベンチマーク
cargo bench -p oni-comb-parser --bench comparison
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
