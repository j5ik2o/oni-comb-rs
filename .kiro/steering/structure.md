# Project Structure

## Organization Philosophy

ワークスペースレベルではドメイン別にクレートを分離し、各クレート内は責務別のレイヤー構造を採用する。`parser` クレートが設計パターンの規範であり、他クレートはそのパターンに従う。

## Directory Patterns

### ワークスペースクレート
**Location**: `{crate}/`
**Purpose**: 各ドメインのパーサー実装
**Pattern**: `parser/` が基盤、`uri/`, `hocon/`, `crond/` がドメイン別、`toys/` が実験用

### パーサー内部の3層構造 (parser クレート)
**Location**: `parser/src/`
**Purpose**: 公開API・拡張・内部実装の分離

| レイヤー | ディレクトリ | 責務 |
|----------|-------------|------|
| core | `src/core/` | 基本型定義 (`Parser`, `ParseResult`, `ParseState`, `Element` 等) |
| extension | `src/extension/` | パーサーのトレイト拡張 (`RepeatParser`, `OperatorParser` 等) |
| internal | `src/internal/` | トレイト実装の詳細 (`parser_impl/`, `parsers_impl/`) |

### エントリーポイント
**Location**: `parser/src/lib.rs`
**Purpose**: `prelude` モジュールで全公開APIを再エクスポート
**Pattern**: ユーザーは `use oni_comb_parser_rs::prelude::*;` のみで利用可能

### テスト・ベンチマーク・例
**Location**: `{crate}/tests/`, `parser/benches/`, `parser/examples/`
**Purpose**: 統合テスト、Criterion ベンチマーク、学習用スニペット

## Naming Conventions

- **ファイル・モジュール**: `snake_case` (`parse_result.rs`, `repeat_parser.rs`)
- **型・トレイト**: `PascalCase` (`Parser`, `ParseResult`, `RepeatParser`)
- **関数**: `snake_case`、動詞始まり (`elm_pred`, `take_while1`, `of_many0`)
- **パーサーファクトリ関数**: 名詞的 (`elm()`, `tag()`, `seq()`, `regex()`)
- **パーサー拡張メソッド**: `of_` / `with_` プレフィックス (`of_many1()`, `with_filter()`)
- **参照版サフィックス**: `_ref` (`elm_ref()`, `elm_any_ref()`)

## Import Organization

```rust
// ユーザー向け: prelude で一括インポート
use oni_comb_parser_rs::prelude::*;

// クレート内部: 層ごとのモジュールパスを使用
use crate::core::{ParseResult, ParseState};
use crate::extension::parser::RepeatParser;
use crate::internal::ParsersImpl;
```

## Code Organization Principles

- `core/` の型は他レイヤーに依存しない
- `extension/` は `core/` にのみ依存し、トレイト定義を行う
- `internal/` は `core/` と `extension/` のトレイト実装を格納する
- ドメインクレート (`uri/`, `hocon/`, `crond/`) は `parser` クレートの `prelude` のみに依存する
- パーサー拡張は「トレイト定義 → impl 分離」のパターンに従う

---
_Document patterns, not file trees. New files following patterns shouldn't require updates_
