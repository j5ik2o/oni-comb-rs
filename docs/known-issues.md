# Known Design Issues

## 1. line_start (byte) と column (char) の単位不整合

**影響範囲**: `StrInput::advance`, `ByteInput::advance`, `StrCheckpoint`, `ByteCheckpoint`

**概要**:
`column` は char (codepoint) 単位で数えるが、`line_start` はバイトオフセットで管理している。
マルチバイト文字を含む行では、`line_start` からの文字数と `column` が一致しない場合がある。

**現時点での影響**:
- YAML のインデントはスペース (ASCII) のみなので `current_indent` (`column - 1`) は正しく動作する
- `line_start` は Checkpoint の保存・復元にしか使われておらず、行テキスト抽出機能は未実装

**本質的な解決**:
`yaml-input-redesign` で `YamlInput` を導入する際に、`line_start` の用途を明確化し、
必要なら char 単位に統一するか、バイト単位のまま「行テキスト抽出専用」と明確に分離する。

## 2. YAML パーサーの手続き的スタイル

**影響範囲**: `modules/yaml/src/` 全体 (`block.rs`, `flow.rs`, `document.rs`)

**概要**:
`ParseContext` (アンカーマップ) と `min_indent` を関数引数で引き回しており、
`Parser` トレイトに乗らない。結果、`parse_next` の戻り値を捨てる手続き的コードが蔓延している。
パーサーコンビネータの設計思想に反する。

**根本原因**:
`Input` 型 (`StrInput`) がパース状態を含んでいないため、追加状態を外部引数で渡す必要がある。

**本質的な解決**:
`yaml-input-redesign` で `YamlInput` 型を導入し、`StrInput` + `ParseContext` + インデントスタック
を一体化する。全パーサーを `fn() -> impl Parser<YamlInput, ...>` 形式にし、
コンビネータパイプラインで記述可能にする。

参照: `openspec/changes/yaml-input-redesign/`

## 3. ParseError の line/column が自動で埋まらない

**影響範囲**: `ParseError::from_expected`, 全コンビネータのエラー生成

**概要**:
`Input` トレイトに `line()`/`column()` を追加し、`ParseError` に `line`/`column` フィールドを
追加したが、`from_expected` は `position: usize` しか受け取らず、常に `line: 0, column: 0` で
生成される。エラーに行/列情報が含まれない。

**暫定対応**:
`ParseError::fill_location_from_src(src)` メソッドを追加し、公開 API (`parse`, `parse_documents`)
のエラー返却時にソーステキストから `position` を走査して行/列を後付け計算する。

**暫定対応の限界**:
- ソーステキスト全体を走査するため O(n)
- 内部コンビネータのエラー合成 (`MergeError::merge`) では行/列が 0 のまま
- `context` エラーにも行/列がない

**本質的な解決**:
`ExpectError` トレイトのシグネチャを `fn from_expected(input: &I, expected: Expected) -> Self`
に変更し、エラー生成時点で `Input` から行/列を取得する。これは parser クレート全体の
破壊的変更になるため、`yaml-input-redesign` と合わせて計画的に実施する。

## 4. YAML タグのパースが未統合

**影響範囲**: `modules/yaml/src/tag.rs`

**概要**:
`parse_tag` 関数は実装済みだが `#[allow(dead_code)]` でパースパイプラインに統合されていない。
`!!str 42` のようなタグ付きスカラーはパース時に認識されず、`apply_tag` による手動後処理が必要。

**暫定対応**:
`apply_tag` を公開 API として提供し、ユーザーがパース後に手動で型変換できるようにしている。

**本質的な解決**:
`yaml-input-redesign` で全パーサーをパイプラインに書き直す際に、`save_anchor` と同様の
パターンで `with_tag(value_parser)` コンビネータを導入し、パース時にタグを認識・適用する。
