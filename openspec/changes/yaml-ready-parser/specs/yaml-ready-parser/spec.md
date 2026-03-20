## ADDED Requirements

### Requirement: litmus grammar の公開面は declarative でなければならない
parser モジュールは、layout-sensitive grammar の top-level litmus grammar 定義を、下流クレート側で public combinator のメソッドチェインと下流所有 helper parser の組み合わせで記述できなければならない (MUST)。top-level litmus grammar 定義では `parse_next` の直呼び、`checkpoint/reset` の直呼び、戻り値破棄、入力状態を読んだ手書き分岐を必要としてはならない。一方で、既存の公開契約を組み合わせるための下流所有 helper parser / `InputStream` wrapper が内部実装として `Parser` / `InputStream` を直接扱うことまでは禁止しない。`fn_parser` は parser capability の不足を補う手段として使ってはならず、宣言的実装が先に成立した後に同値な振る舞いを保った局所最適化としてのみ使ってよい。

#### Scenario: litmus grammar が命令型 escape hatch なしで記述できる
- **WHEN** parser モジュールだけを使って YAML 風の litmus grammar 群を実装する
- **THEN** top-level litmus grammar 定義には `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、`fn_parser` による escape hatch が現れない

#### Scenario: 下流 helper parser は公開契約の合成をカプセル化できる
- **WHEN** 下流クレートが checkpoint 可能な state や位置情報 API を使う helper parser / `InputStream` wrapper を定義する
- **THEN** その helper は parser モジュールの既存公開契約の上に構築され、top-level litmus grammar は引き続き declarative に記述できる

#### Scenario: `fn_parser` は capability 実現の代替手段ではない
- **WHEN** litmus grammar の成立前に `fn_parser` で layout-sensitive grammar を実装しようとする
- **THEN** その実装は `YAML-ready` 判定の根拠として扱ってはならない

#### Scenario: `fn_parser` は局所最適化としてのみ許可される
- **WHEN** 同じ grammar に対して public combinator だけの宣言的実装が先に存在し、その後で性能上の根拠を伴って一部を `fn_parser` に置き換える
- **THEN** `fn_parser` の利用は `YAML-ready` 契約違反ではなく、optimization escape hatch として扱われる

### Requirement: YAML-ready 判定は litmus grammar 群で検証されなければならない
parser モジュールは、YAML クレート本体ではなく litmus grammar 群で `YAML-ready` を検証しなければならない (MUST)。litmus grammar には少なくとも block list、indent nesting、flow/block 切替、multiline block、block scalar header、document boundary、simple-key gating、simple-key backtrack、flow plain scalar boundary、indent error を含めなければならない。

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

#### Scenario: block scalar header grammar
- **WHEN** chomping indicator や indentation indicator を伴う block scalar header litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は header の分岐とその後の block 本体の取り扱いを public combinator だけで表現できる

#### Scenario: document boundary grammar
- **WHEN** `---` / `...` に相当する document boundary litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は document 開始と終了を public combinator だけで表現できる

#### Scenario: simple-key gating grammar
- **WHEN** simple-key 可否のような scoped boolean flag を使う litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は flag の判定と一時更新を public combinator だけで表現できる

#### Scenario: simple-key backtrack grammar
- **WHEN** 左枝が simple-key 可否のような scoped boolean flag を更新した後に失敗し、右枝へ分岐する litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は flag の巻き戻しと分岐選択を手動 state 管理なしで public combinator だけで表現できる

#### Scenario: flow plain scalar boundary grammar
- **WHEN** flow context において plain scalar が `,`、`]`、`}`、`:` 境界で停止する litmus grammar を parser モジュールだけで記述する
- **THEN** grammar は flow delimiter と scalar の停止条件を手書き分岐なしで public combinator だけで表現できる

#### Scenario: indent error grammar
- **WHEN** 期待インデントを満たさない入力を litmus grammar に与える
- **THEN** grammar は位置情報と文脈を持つエラーを返す
