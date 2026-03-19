## ADDED Requirements

### Requirement: with_indent はインデントレベルを設定して内部パーサーを実行する
`with_indent(n, parser)` は `YamlInput` のインデントスタックに `n` を push し、内部パーサーを実行した後、成功・失敗に関わらず pop する。

#### Scenario: 内部パーサーが成功する場合
- **WHEN** `with_indent(2, some_parser)` を実行し、内部パーサーが `Ok(value)` を返す
- **THEN** `Ok(value)` を返し、インデントスタックは `with_indent` 呼び出し前の状態に戻る

#### Scenario: 内部パーサーが失敗する場合
- **WHEN** `with_indent(2, some_parser)` を実行し、内部パーサーが `Err(Backtrack)` を返す
- **THEN** `Err(Backtrack)` を返し、インデントスタックは `with_indent` 呼び出し前の状態に戻る

### Requirement: indent_guard は現在のインデントが最小値以上かを検査する
`indent_guard()` は `YamlInput` の `current_min_indent()` と現在の列位置を比較し、列位置が最小値未満なら Backtrack エラーを返す。入力を消費しない。

#### Scenario: インデントが十分な場合
- **WHEN** 現在の column が 5 で `current_min_indent()` が 2 の状態で `indent_guard()` を適用する
- **THEN** `Ok(())` を返す

#### Scenario: インデントが不足する場合
- **WHEN** 現在の column が 1 で `current_min_indent()` が 2 の状態で `indent_guard()` を適用する
- **THEN** `Err(Backtrack)` を返す

### Requirement: save_anchor はアンカープレフィックスを検出し値をアンカーマップに保存する
`save_anchor(parser)` は `&name` プレフィックスを検出した場合、名前を取得し、内部パーサーで値をパースし、`input.set_anchor(name, value.clone())` で保存してから値を返す。プレフィックスがない場合は内部パーサーをそのまま実行する。

#### Scenario: アンカープレフィックスがある場合
- **WHEN** `save_anchor(scalar_parser)` を `"&myref hello"` に適用する
- **THEN** `Ok(YamlValue::String("hello"))` を返し、アンカーマップに `"myref" -> String("hello")` が保存される

#### Scenario: アンカープレフィックスがない場合
- **WHEN** `save_anchor(scalar_parser)` を `"hello"` に適用する
- **THEN** `Ok(YamlValue::String("hello"))` を返し、アンカーマップは変更されない

### Requirement: resolve_alias はエイリアスをアンカーマップから解決する
`resolve_alias()` は `*name` をパースし、`YamlInput` のアンカーマップから対応する値をクローンして返す。アンカーが見つからない場合は Cut エラーを返す。

#### Scenario: 既知のエイリアスを解決する
- **WHEN** アンカーマップに `"ref" -> Integer(42)` がある状態で `resolve_alias()` を `"*ref"` に適用する
- **THEN** `Ok(YamlValue::Integer(42))` を返す

#### Scenario: 未知のエイリアスでエラー
- **WHEN** アンカーマップが空の状態で `resolve_alias()` を `"*unknown"` に適用する
- **THEN** `Err(Cut)` を返す
