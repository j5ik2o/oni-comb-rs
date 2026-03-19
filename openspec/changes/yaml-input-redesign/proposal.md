## Why

現在の YAML パーサーは `ParseContext`（アンカーマップ）と `min_indent` を関数引数で引き回しており、`Parser` トレイトに乗らない。結果、パーサーコンビネータライブラリでありながら手続き的なコードを強要される。`parse_next` の戻り値を捨てる副作用依存のパターンが蔓延し、JSON パーサーで実現した純粋パイプラインスタイルとの一貫性もない。根本原因は `Input` 型（`StrInput`）がパース状態を含んでいないこと。

## What Changes

- `YamlInput<'a>` 型を新規作成: `StrInput<'a>` + `ParseContext`（アンカーマップ）+ インデントスタックを一体化。`Input` トレイトを実装し、`StrInput` に委譲
- YAML 固有コンビネータを導入:
  - `with_indent(n, parser)`: インデントレベルを設定してパーサーを実行、完了後に復元
  - `save_anchor(parser)`: アンカープレフィックスを検出し、内部パーサーの結果を `YamlInput` のアンカーマップに保存
  - `resolve_alias()`: エイリアス (`*name`) をアンカーマップから解決
  - `indent_guard(min)`: 現在のインデントが `min` 以上でなければ Backtrack
- 全 YAML パーサー関数を `fn(...) -> impl Parser<YamlInput, ...>` 形式に書き直し。`parse_next` の直接呼び出しは公開 API の `parse()` / `parse_documents()` のみに限定
- `ParseContext` を独立モジュールとして残すが、外部から直接使用せず `YamlInput` 経由でアクセス
- `context.rs` の `ParseContext` を `YamlInput` の内部実装に統合

## Capabilities

### New Capabilities
- `yaml-input`: YamlInput 型と Input トレイト実装。StrInput 委譲 + YAML 固有状態（アンカーマップ、インデントスタック）
- `yaml-combinators`: YAML 固有コンビネータ（with_indent, save_anchor, resolve_alias, indent_guard）
- `yaml-pipeline-rewrite`: 全 YAML パーサーの純粋パイプラインスタイルへの書き直し

### Modified Capabilities

## Impact

- `modules/yaml/src/` の全ファイルが影響を受ける（block.rs, flow.rs, document.rs, scalar.rs, multiline.rs, common.rs, context.rs, lib.rs）
- 公開 API（`parse`, `parse_documents`, `YamlValue`, `apply_tag`）は変更なし
- `modules/parser/` への変更なし（YamlInput は YAML クレート内で Input を実装）
- 全35テストが引き続き通ること
