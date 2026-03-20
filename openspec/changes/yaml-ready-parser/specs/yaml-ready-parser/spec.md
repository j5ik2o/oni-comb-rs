## ADDED Requirements

### Requirement: 下流 grammar 実装は public combinator のメソッドチェインだけで記述できなければならない
parser モジュールは、layout-sensitive grammar を下流クレート側で public combinator のメソッドチェインのみで記述できなければならない。下流 grammar 実装では `parse_next` の直呼び、`checkpoint/reset` の直呼び、戻り値破棄、入力状態を読んだ手書き分岐を必要としてはならない。

#### Scenario: litmus grammar が命令型 escape hatch なしで記述できる
- **WHEN** parser モジュールだけを使って YAML 風の litmus grammar 群を実装する
- **THEN** 下流 grammar 実装には `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄が現れない

### Requirement: YAML-ready 判定は litmus grammar 群で検証されなければならない
parser モジュールは、YAML クレート本体ではなく litmus grammar 群で `YAML-ready` を検証しなければならない。litmus grammar には少なくとも block list、indent nesting、flow/block 切替、multiline block、document boundary、indent error を含めなければならない。

#### Scenario: block list grammar
- **WHEN** 行頭でのみ `- item` を受理する litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は list item の認識と非行頭での失敗を public combinator だけで表現できる

#### Scenario: indent nesting grammar
- **WHEN** インデント増減でネストが決まる litmus grammar を parser モジュールだけで記述する
- **THEN** grammar はネスト開始、継続、終了を手動 state 巻き戻しなしで表現できる

#### Scenario: flow and block switching grammar
- **WHEN** flow style と block style を切り替える litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は現在文脈に応じた分岐を public combinator だけで表現できる

#### Scenario: multiline block grammar
- **WHEN** `|` / `>` に相当する multiline block litmus grammar を parser モジュールだけで記述する
- **THEN** grammar はインデント付きの継続行を手動 `parse_next` なしで表現できる

#### Scenario: document boundary grammar
- **WHEN** `---` / `...` に相当する document boundary litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は document 開始と終了を public combinator だけで表現できる

#### Scenario: indent error grammar
- **WHEN** 期待インデントを満たさない入力を litmus grammar に与える
- **THEN** grammar は位置情報と文脈を持つエラーを返す
