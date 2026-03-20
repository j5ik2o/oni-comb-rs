# Known Design Issues

## 1. line_start (byte) と column (char) の単位不整合

**影響範囲**: `StrInputStream::advance`, `ByteInputStream::advance`, `StrCheckpoint`, `ByteCheckpoint`

**概要**:
`column` は char (codepoint) 単位で数えるが、`line_start` はバイトオフセットで管理している。
マルチバイト文字を含む行では、`line_start` からの文字数と `column` が一致しない場合がある。

**現時点での影響**:
- YAML のインデントはスペース (ASCII) のみなので `current_indent` (`column - 1`) は正しく動作する
- `line_start` は Checkpoint の保存・復元にしか使われておらず、行テキスト抽出機能は未実装

**本質的な解決**:
YAML の block syntax を本格的に扱う段階で `line_start` の用途を明確化し、
必要なら char 単位に統一するか、バイト単位のまま「行テキスト抽出専用」と明確に分離する。

## 2. ParseError の line/column が自動で埋まらない

**影響範囲**: `ParseError::from_expected`, 全コンビネータのエラー生成

**概要**:
`InputStream` トレイトに `line()`/`column()` を追加し、`ParseError` に `line`/`column` フィールドを
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
に変更し、エラー生成時点で `InputStream` から行/列を取得する。これは parser クレート全体の
破壊的変更になるため、parser core を見直す段階で計画的に実施する。
