## ADDED Requirements

### Requirement: parser は checkpoint 可能な layout state を扱えなければならない
parser モジュールは、入力位置だけでなく layout-sensitive grammar に必要な layout state を checkpoint / reset の対象として扱えなければならない (MUST)。対象には少なくとも期待インデント、flow level、simple-key 可否、block 文脈に相当する state を含めなければならない。

#### Scenario: or の失敗で layout state が巻き戻る
- **WHEN** 左枝が layout state を更新した後に Backtrack で失敗し、右枝へ分岐する
- **THEN** 右枝の開始時点では layout state が分岐前の値に復元されている

#### Scenario: attempt の失敗で layout state が巻き戻る
- **WHEN** parser が layout state を更新した後に失敗し、`attempt()` によって巻き戻される
- **THEN** 入力位置と layout state の両方が開始 checkpoint に復元されている

#### Scenario: state flag の更新も巻き戻る
- **WHEN** parser が simple-key 可否のような boolean flag を更新した後に Backtrack で失敗する
- **THEN** reset 後の flag 値は checkpoint 取得時点の値に復元されている

### Requirement: parser は layout-sensitive grammar を支える公開契約を提供しなければならない
parser モジュールは、layout-sensitive grammar が現在の layout state を parser モジュールの既存公開契約と downstream 側の合成だけで観測し、必要な範囲で scoped に更新できるようにしなければならない (MUST)。少なくとも行頭判定、期待インデント判定、flow/block 文脈判定、boolean flag 判定と一時更新を表現できなければならない。これは YAML 専用の Layout API を parser モジュールへ追加することを要求しない。

#### Scenario: 行頭判定
- **WHEN** grammar が「現在位置が行頭である」ことを要求する
- **THEN** grammar は parser モジュールの公開契約と downstream 側の合成だけでその条件を記述できる

#### Scenario: 期待インデント判定
- **WHEN** grammar が「現在位置のインデントが期待値以上である」ことを要求する
- **THEN** grammar は parser モジュールの公開契約と downstream 側の合成だけでその条件を記述できる

#### Scenario: flow/block 文脈判定
- **WHEN** grammar が flow style と block style で異なる分岐を選ぶ
- **THEN** grammar は現在文脈の判定を parser モジュールの公開契約と downstream 側の合成だけで記述できる

#### Scenario: state flag 判定
- **WHEN** grammar が simple-key 可否のような boolean flag の現在値を要求する
- **THEN** grammar は入力状態の手動巻き戻しに頼らず、parser モジュールの公開契約と downstream 側の合成だけでその条件を記述できる

#### Scenario: state flag の scoped 更新
- **WHEN** grammar が parser 実行中だけ simple-key 可否のような boolean flag を切り替える必要がある
- **THEN** grammar は手動 state 管理なしで parser モジュールの公開契約と downstream 側の合成だけを使ってその更新範囲を記述できる

### Requirement: parser は checkpoint 対象の state と downstream semantic data を分離しなければならない
parser モジュールは、backtrack の健全性に必要な layout state と、parse 後フェーズへ委譲可能な downstream semantic data を分離しなければならない (MUST)。downstream semantic data を常に checkpoint 対象へ含めることを要求してはならない。

#### Scenario: anchor table が checkpoint 対象でなくても layout-sensitive grammar は成立する
- **WHEN** downstream semantic data を parser core 外へ残した設計を採用する
- **THEN** litmus grammar 群の成立性は checkpoint 可能な layout state だけで担保される
