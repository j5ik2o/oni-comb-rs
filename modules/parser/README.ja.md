# oni-comb-parser

[English](README.md)

[oni-comb-rs](../../) のコアクレート。Rust 製パーサーモナドコンビネータライブラリ。

## 特徴

- **Parsec スタイルの再帰下降** — デフォルト LL(1)、`attempt` で LL(*)
- **完全な型クラス階層** — Functor (`map`) / Applicative (`zip`) / Alternative (`or`) / Monad (`flat_map`)
- **ゼロコストなコンビネータ合成** — Applicative コンビネータは具象型でスタック上に構築、ヒープアロケーションゼロ
- **Backtrack / Cut エラー制御** — `or` は `Backtrack` のみリカバリ。`Cut` は伝播。`attempt` / `cut` で制御
- **構造化エラー** — `ParseError`（位置・期待トークン・`.context()` ラベル）
- **ジェネリック InputStream トレイト** — `StrInputStream<'a>`（`&str`）、`ByteInputStream<'a>`（`&[u8]`）
- **`no_std` 対応** — `#![no_std]` + `alloc`

## パフォーマンス

- **JSON フルベンチ首位（2026-03-18 再計測）** — 107KB サンプルで `109.5 µs`、`932.1 MiB/s`。`winnow` は `178.7 µs`、`571.3 MiB/s`
- **generic token パーサーは今回の `comparison` 再計測でも competitive** — identifier `"foo_bar_123"` が `20.0 ns`、integer `"184467...615"` が `22.8 ns`
- **今回の `comparison` 再計測では subset JSON 全体が悪化** — object 寄りケースも含めて（`{"name":"oni-comb",...}`: `704.0 ns`, `{"a":1,...,"h":8}`: `1,459 ns`）後退した
- **残るホットスポット** — 極小の分岐ディスパッチ系 microbenchmark では `flat_map` がまだ `winnow` / `nom` に劣る
- 詳細は [ベンチマーク結果（英語）](benches/README.md) と [ベンチマーク結果（日本語）](benches/README.ja.md) を参照

## クイックスタート

```rust
use oni_comb_parser::prelude::*;

// 'a' または 'b' にマッチ
let mut parser = char('a').or(char('b'));
let mut input = StrInputStream::new("b");
assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');

// 識別子: 先頭が英字/_, 以降は英数字/_
let mut input = StrInputStream::new("foo_123");
let (head, tail) = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
    .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'))
    .parse_next(&mut input)
    .unwrap();
assert_eq!(head, 'f');
assert_eq!(tail, "oo_123");

// 整数
let mut int_parser = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<u64>().unwrap());
let mut input = StrInputStream::new("42");
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
| `quoted_string()` | JSON 準拠ダブルクォート文字列（エスケープなしなら借用） | `Cow<'a, str>` |
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
| `.many0_fold(init, f)` / `.many1_fold(init, f)` | — | 0個以上 / 1個以上の畳み込み（ゼロアロケーション） |
| `.many0_into(c)` / `.many1_into(c)` | — | ユーザー指定コンテナ（`Extend`）に収集 |
| `.sep_by0(sep)` / `.sep_by1(sep)` | — | 区切り付き繰り返し |
| `.sep_by0_fold(sep, init, f)` / `.sep_by1_fold(sep, init, f)` | — | 区切り付き畳み込み（ゼロアロケーション） |
| `.sep_by0_into(sep, c)` / `.sep_by1_into(sep, c)` | — | 区切り付きコンテナ収集 |
| `.chainl1(op)` / `.chainr1(op)` | — | 演算子結合チェーン |
| `.context(label)` | — | エラーコンテキストラベル追加 |
| `.map_res(f, label)` | — | 失敗しうる関数で変換 |

## 入力型

| 型 | Token | Slice | 用途 |
|----|-------|-------|------|
| `StrInputStream<'a>` | `char` | `&'a str` | テキストパース（デフォルト） |
| `ByteInputStream<'a>` | `u8` | `&'a [u8]` | バイナリプロトコルパース |

## YAML-ready の現状

`oni-comb-parser` は、parser モジュール単体として `yaml-ready-parser` の受け入れ契約を満たした状態になりました。

- 実行可能な受け入れ契約は `modules/parser/tests/yaml_ready_acceptance.rs` にあります
- litmus grammar は block list、indent nesting、flow/block switching、multiline block、block scalar header、document boundary、simple-key gating、simple-key backtrack、flow plain scalar boundary、indent error を含みます
- この契約は `modules/parser` に YAML 専用の Layout API を追加せずに満たしています
- 将来の YAML 実装は、既存の parser/core capability の組み合わせで進める前提であり、`parse_next` 直呼び、`checkpoint/reset` 直呼び、`fn_parser` による不足補完を前提にしません

## ビルド・テスト

```bash
cargo build -p oni-comb-parser
cargo test -p oni-comb-parser

# ベンチマーク
cargo bench -p oni-comb-parser --bench comparison
```

詳細なベンチ表と分析:
- [ベンチマーク結果（英語）](benches/README.md)
- [ベンチマーク結果（日本語）](benches/README.ja.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
