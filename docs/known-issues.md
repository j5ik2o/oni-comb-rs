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

## 2. YAML block syntax の indentation 制御は未評価

**影響範囲**: `modules/yaml/src/syntax_parser/mod.rs`, `modules/yaml/src/syntax_parser/parser.rs`, `modules/yaml/src/syntax_parser/cursor.rs`, `modules/yaml/src/syntax_parser/scalar.rs`

**概要**:
現在の YAML syntax parser は `SyntaxParser { src, pos }` によるローカルなカーソル実装であり、
Phase 1 の対象である flow subset と document marker には十分だった。一方で block mapping /
block sequence / block scalar の indentation 制御はまだ未実装であり、parser core 拡張が本当に必要かは
Phase 2 の試作結果で判断する必要がある。

**現時点の評価**:
- Gate A の観点では、まだ `oni-comb-parser` 本体の拡張を入れる段階ではない
- まずは `SyntaxParser` 内で indentation 引数や補助メソッドを追加し、YAML モジュール内で block syntax を試作する
- その試作が不自然な `fn_parser` 依存や parser 内部状態の要求に発展した場合だけ、core 拡張を再評価する

**本質的な解決**:
Phase 2 の試作結果をもとに判断する。`SyntaxParser` のローカル実装で block syntax を自然に保てるなら、
parser core は拡張しない。逆に indentation の引き回しや再帰表現が破綻した場合は、
`docs/yaml-parser-roadmap.md` の Gate A / Phase 5 に沿って parameterized recursion や
indentation-aware error support を検討する。

## 3. ParseError の line/column が自動で埋まらない

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

## 4. YAML タグのパースが未統合

**影響範囲**: `modules/yaml/src/syntax_parser/`, `modules/yaml/src/lib.rs`

**概要**:
Phase 1 の syntax parser は tag 構文自体を対象外としており、`!!str 42` のような
タグ付きスカラーは `parse_syntax` / `parse` のどちらでも認識されない。
現状の `apply_tag` は、既に得られた `YamlValue` に対して手動で Core Schema タグを適用する補助 API に留まる。

**暫定対応**:
`apply_tag` を公開 API として提供し、ユーザーがパース後に手動で型変換できるようにしている。
不正なタグ適用は panic ではなく `Result` のエラーとして返す。

**本質的な解決**:
tag syntax と resolver の統合フェーズで、syntax parser が tag を構文要素として保持し、
resolver 側で意味解決できる形に接続する。必要ならその段階で `with_tag(value_parser)` 相当の
補助 API を YAML モジュール内に追加する。
