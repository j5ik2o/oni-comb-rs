## ADDED Requirements

### Requirement: parser は layout-sensitive grammar を支える公開契約を提供しなければならない
parser module は、layout-sensitive grammar が必要とする位置・巻き戻し・文脈復元を、parser モジュールの既存公開契約と downstream 側の合成だけで扱える形で提供しなければならない (MUST)。この requirement は parser module に YAML 専用 Layout API を追加することを要求しない。

#### Scenario: current input position can distinguish line head
- **WHEN** downstream grammar が現在位置の byte offset と current line start offset を読む
- **THEN** その grammar は現在トークンが行頭にあるかを判定できる

#### Scenario: checkpoint and reset rewind parser-core context
- **WHEN** parser が branch 前に checkpoint を取り、左分岐で parser-core 内部状態を更新した後に backtrack する
- **THEN** reset 後の右分岐は offset だけでなく、checkpoint 時点の parser-core 文脈を観測する

#### Scenario: downstream helper can compute indentation from public position data
- **WHEN** downstream helper parser が current offset と current line start offset を取得する
- **THEN** helper は YAML 専用 API なしで current line の visual indentation を計算できる

#### Scenario: parser-core tracks generic checkpointable state without exposing YAML type
- **WHEN** parser-core が indentation depth、context depth、flag 群のような内部文脈を checkpoint に含める
- **THEN** それらは generic parser state として rewind され、公開 API に YAML 固有型を露出しない
