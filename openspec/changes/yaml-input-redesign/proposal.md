## Why

`docs/known-issues.md` に記録された4件の設計課題を一括で解決する。

1. **YAML 手続き的スタイル**: `ParseContext` と `min_indent` を関数引数で引き回し、`Parser` トレイトに乗らない。パーサーコンビネータなのに `parse_next` の戻り値を捨てる副作用コードが蔓延
2. **ParseError の line/column 未反映**: `ExpectError::from_expected` が `position` しか受け取らず、エラーに行/列情報が入らない。暫定の `fill_location_from_src` は O(n) で内部エラーには効かない
3. **line_start/column の単位不整合**: `column` は char 単位、`line_start` はバイト単位で、マルチバイト文字で不整合が起きうる
4. **YAML タグ未統合**: `parse_tag` が dead code で、`!!str 42` がパース時に認識されない

根本原因は (a) `Input` 型がパース状態を含まない、(b) `ExpectError` が `Input` にアクセスできない、の2点。

## What Changes

### parser クレート (ExpectError 改修)
- **`ExpectError` トレイトのシグネチャ変更**: `fn from_expected(position: usize, expected: Expected)` → `fn from_expected_at(input: &I, expected: Expected)` を追加。既存の `from_expected` は互換維持し deprecated にする
- **ParseError**: `from_expected_at` で `input.line()` / `input.column()` を自動取得。`fill_location_from_src` は削除
- **line_start の用途明確化**: `StrCheckpoint` / `ByteCheckpoint` の `line_start` フィールドにドキュメント追加。`column` (char 単位) と `line_start` (byte 単位) の意図的な分離を明示
- 全コンビネータのエラー生成を `from_expected` → `from_expected_at` に移行

### yaml クレート (YamlInput + パイプライン化)
- **`YamlInput<'a>` 新規作成**: `StrInput<'a>` + アンカーマップ + インデントスタックを一体化。`Input` トレイトを委譲実装
- **YAML 固有コンビネータ**: `with_indent`, `save_anchor`, `resolve_alias`, `indent_guard`, `with_tag`
- **全パーサーをパイプラインに書き直し**: `fn() -> impl Parser<YamlInput, ...>` 形式
- **`parse_tag` をパースパイプラインに統合**: `with_tag(value_parser)` コンビネータで `!!str 42` をパース時に認識・型変換
- `context.rs` を削除し `YamlInput` に統合

### json クレート
- `fill_location_from_src` 呼び出しを削除（`from_expected_at` により不要に）

### 下流クレート (uri, crond)
- `from_expected` → `from_expected_at` への移行（コンパイルエラー対応）

## Capabilities

### New Capabilities
- `expect-error-redesign`: `ExpectError` トレイトに `from_expected_at(input, expected)` を追加し、エラー生成時に行/列を自動取得
- `yaml-input`: YamlInput 型。StrInput 委譲 + アンカーマップ + インデントスタック
- `yaml-combinators`: with_indent, save_anchor, resolve_alias, indent_guard, with_tag
- `yaml-pipeline-rewrite`: 全 YAML パーサーの純粋パイプラインスタイルへの書き直し

### Modified Capabilities

## Impact

- **parser クレート**: `ExpectError` トレイト変更、全コンビネータ・プリミティブのエラー生成箇所を移行。`ParseError::fill_location_from_src` 削除
- **json クレート**: `fill_location_from_src` 削除。エラーに自動で行/列が入るように
- **yaml クレート**: 全ファイル書き直し
- **uri / crond クレート**: `from_expected` → `from_expected_at` のコンパイルエラー対応
- 公開 API (`parse`, `parse_documents`, `JsonValue`, `YamlValue`) は変更なし
- 全テストが引き続き通ること
