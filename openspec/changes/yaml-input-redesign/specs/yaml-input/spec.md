## ADDED Requirements

### Requirement: YamlInput は Input トレイトを実装し StrInput に委譲する
`YamlInput<'a>` は `StrInput<'a>` をラップし、`Input` トレイトの全メソッド（`next_token`, `peek_token`, `checkpoint`, `reset`, `offset`, `remaining`, `is_eof`, `line`, `column`, `slice_since`）を内部の `StrInput` に委譲する。

#### Scenario: Input メソッドが StrInput と同等に動作する
- **WHEN** `YamlInput::new("abc")` を作成し `next_token()` を呼ぶ
- **THEN** `Some('a')` を返し、`remaining()` は `"bc"` を返す

#### Scenario: Checkpoint と reset が正しく動作する
- **WHEN** 2トークン消費後に `checkpoint()` を取り、さらに1トークン消費後に `reset` する
- **THEN** `remaining()` が checkpoint 時点の状態に戻る

### Requirement: YamlInput はアンカーマップを保持する
`YamlInput` は内部に `HashMap<String, YamlValue>` を持ち、`set_anchor(name, value)` と `get_anchor(name) -> Option<&YamlValue>` メソッドを提供する。

#### Scenario: アンカーの保存と取得
- **WHEN** `input.set_anchor("defaults", value)` を呼んだ後 `input.get_anchor("defaults")` を呼ぶ
- **THEN** 保存した value への参照を返す

#### Scenario: 存在しないアンカーの取得
- **WHEN** `input.get_anchor("unknown")` を呼ぶ
- **THEN** `None` を返す

### Requirement: YamlInput はインデントスタックを保持する
`YamlInput` は内部に `Vec<usize>` のインデントスタックを持ち、`push_indent(n)`, `pop_indent()`, `current_min_indent() -> usize` メソッドを提供する。スタックが空の場合 `current_min_indent()` は 0 を返す。

#### Scenario: インデントの push と pop
- **WHEN** `push_indent(2)` の後 `current_min_indent()` を呼ぶ
- **THEN** 2 を返す

#### Scenario: ネストした push と pop
- **WHEN** `push_indent(2)` → `push_indent(4)` → `pop_indent()` の後 `current_min_indent()` を呼ぶ
- **THEN** 2 を返す（外側のインデントに戻る）

#### Scenario: 空スタックでの current_min_indent
- **WHEN** 何も push せずに `current_min_indent()` を呼ぶ
- **THEN** 0 を返す
