## ADDED Requirements

### Requirement: parser は checkpoint 可能な layout state を扱えなければならない
parser モジュールは、入力位置だけでなく layout-sensitive grammar に必要な layout state を checkpoint / reset の対象として扱えなければならない。対象には少なくとも期待インデント、flow level、simple-key 可否、block 文脈に相当する state を含めなければならない。

#### Scenario: or の失敗で layout state が巻き戻る
- **WHEN** 左枝が layout state を更新した後に Backtrack で失敗し、右枝へ分岐する
- **THEN** 右枝の開始時点では layout state が分岐前の値に復元されている

#### Scenario: attempt の失敗で layout state が巻き戻る
- **WHEN** parser が layout state を更新した後に失敗し、`attempt()` によって巻き戻される
- **THEN** 入力位置と layout state の両方が開始 checkpoint に復元されている

### Requirement: parser は layout state を観測する public capability を提供しなければならない
parser モジュールは、layout-sensitive grammar が現在の layout state を public API 経由で観測できる capability を提供しなければならない。少なくとも行頭判定、期待インデント判定、flow/block 文脈判定を表現できなければならない。

#### Scenario: 行頭判定
- **WHEN** grammar が「現在位置が行頭である」ことを要求する
- **THEN** grammar は入力位置を直接検査せず public capability だけでその条件を記述できる

#### Scenario: 期待インデント判定
- **WHEN** grammar が「現在位置のインデントが期待値以上である」ことを要求する
- **THEN** grammar は `column()` の手書き比較に依存せず public capability だけでその条件を記述できる

#### Scenario: flow/block 文脈判定
- **WHEN** grammar が flow style と block style で異なる分岐を選ぶ
- **THEN** grammar は現在文脈の判定を public capability だけで記述できる

### Requirement: parser は checkpoint 対象の state と semantic state を分離しなければならない
parser モジュールは、backtrack の健全性に必要な layout state と、後段処理に委譲可能な semantic state を分離しなければならない。semantic state を常に checkpoint 対象へ含めることを要求してはならない。

#### Scenario: anchor table が checkpoint 対象でなくても layout-sensitive grammar は成立する
- **WHEN** semantic state を parser core 外へ残した設計を採用する
- **THEN** litmus grammar 群の成立性は checkpoint 可能な layout state だけで担保される
