- 日本語でやりとりしてください

## プロジェクト概要

oni-comb-rs は Rust 製パーサーコンビネータライブラリの v2 リブート版。旧 v1 の `Rc<dyn Fn>` ベース設計を捨て、trait + concrete combinator 型（`Map`, `Then`, `Or` 等）で構成する。動的ディスパッチ・ヒープ確保を排し、Applicative/Alternative 主体で最適化しやすい設計を目指している。

## ビルド・テスト

```bash
# ビルド
cargo build

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
Input (trait)          -- 入力ストリーム抽象。Checkpoint による巻き戻しを提供
  └─ StrInput          -- &str 向け実装。Checkpoint = usize (byte offset)

Parser (trait)         -- parse_next(&mut self, &mut I) -> PResult<O, E>
  └─ ParserExt (trait) -- map/then/or/attempt/cut/optional/many0 のメソッドチェーン

Fail (enum)            -- Backtrack(E) | Cut(E) | Incomplete | ZeroProgress
PResult<T, E>          -- Result<T, Fail<E>>
```

### モジュール構成 (`parser/src/`)

| モジュール | 役割 |
|-----------|------|
| `input.rs` | `Input` トレイト（`Checkpoint`, `Slice`, `reset`, `is_eof`） |
| `str_input.rs` | `StrInput<'a>` — `&str` 向け `Input` 実装 |
| `parser.rs` | `Parser<I>` トレイト（`Output`, `Error`, `parse_next`） |
| `parser_ext.rs` | `ParserExt<I>` — 全 `Parser` に自動実装されるコンビネータメソッド |
| `fail.rs` | `Fail<E>` enum と `PResult` 型エイリアス |
| `combinator/` | 各コンビネータの concrete 型（`Map`, `Then`, `Or`, `Attempt`, `Cut`, `Optional`, `Many`） |
| `text/` | テキスト専用パーサー（`Char`, `Tag`, `Eof`） |

### 設計上の重要な判断

- **Fail::Backtrack vs Fail::Cut**: `or` は Backtrack のみリカバリし、Cut はそのまま伝播。`attempt` は Cut を Backtrack に降格、`cut` は Backtrack を Cut に昇格
- **flat_map は意図的に制限**: 構文形状を静的に保つため、Applicative/Alternative 主体。flat_map は将来 escape hatch として提供予定
- **再帰は boxed recursion**: 再帰の結び目だけ `Box<dyn Parser>` に落とし、非再帰部分は concrete 型を維持
- **入力型は当面 `&str` 限定**: `no_std`/streaming/bytes は後回し
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
`p` の Backtrack を Cut に昇格させる。`tag(":").then(value.cut())` のように使う。

### `optional(p)`
Backtrack のみ `Ok(None)` に変換。Cut/Incomplete は伝播。

### `many0(p)`
Backtrack で停止し収集結果を返す。Cut/Incomplete は伝播。zero-progress（入力を消費せずに成功し続ける）は `ZeroProgress` エラー。

### `sep_by`, `between`, `chainl1`, `chainr1`
flat_map 再帰ではなく専用ループで実装する。

## マイルストーン

| # | 名前 | 実装対象 | まだやらない | 完了条件 |
|---|------|----------|-------------|---------|
| 1 | Core | `Input`, `Span`, `Fail`, `PResult`, `Parser`, `ParserExt`, `StrInput` | regex, cache, bytes, recursive helper | `or/attempt/cut` の単体テストが通る |
| 2 | Primitive | `eof`, `char`, `tag`, `satisfy`, `take_while0/1`, `peek` | unicode category, regex, bytes | identifier/integer parser が組める |
| 3 | Combinators | `map`, `then`, `preceded`, `terminated`, `between`, `optional`, `many0/1`, `sep_by0/1`, `chainl1`, `chainr1` | monadic DSL 拡張 | expression parser と CSV/JSON subset の骨格が書ける |
| 4 | Text module | whitespace, ascii token, identifier, integer, quoted string | bytes 共通化 | JSON subset と URI tokenizer が動く |
| 5 | Recursive | boxed `recursive()` helper, precedence parser | left recursion, packrat | 四則演算+括弧の parser が動く |
| 6 | Error reporting | span, expected-set, context stack, cut-aware merge | カラー診断, IDE 連携 | JSON subset の失敗位置と期待トークンが出る |
| 7 | Benchmark | criterion bench, allocation counter, regression threshold | micro-opt の先走り | v1 比較でボトルネック定量化、1回最適化サイクル完了 |

## ベンチマーク計画

- **比較対象**: 旧 v1 PoC（`7a038a18` を bench 専用 `legacy_v1` クレートとして固定）、`nom`、`pom`、v2 マイルストーン間の回帰
- **代表 workload**: identifier/integer（token hot path）、JSON subset、URI/URL、expression parser
- **観測項目**: throughput（Criterion）、allocation count（`dhat-rs`）、clone/refcount 発生数、error path cost
- **最適化開始時点**: Milestone 3 完了後（core だけでは最終コスト配分が不明なため）
