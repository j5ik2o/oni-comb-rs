## MODIFIED Requirements

### Requirement: Checkpoint は line/column を含み reset で復元される
`checkpoint()` は現在の offset、line、column に加えて、行アンカーのような位置関連 state を保存しなければならない (MUST)。`reset(cp)` は保存された位置関連 state を O(1) で復元しなければならない。ここで行アンカーは列番号ではなく、現在行の先頭を指す anchor として扱わなければならない。

#### Scenario: checkpoint と reset で位置 state が復元される
- **WHEN** 複数行入力で checkpoint を取得し、さらに数トークン消費した後に reset する
- **THEN** reset 後の offset、line、column、行アンカーは checkpoint 時点の値に復元される

#### Scenario: 行アンカーと column は別責務で保持される
- **WHEN** マルチバイト文字を含む行で checkpoint を取得する
- **THEN** column は人間向け列番号を表し、行アンカーは行スライス抽出のための anchor として独立に扱われる

#### Scenario: Checkpoint の Ord は位置比較に使える
- **WHEN** 異なる位置で取得した 2 つの checkpoint を比較する
- **THEN** 比較結果は grammar の backtrack 判定に使える安定した順序を提供する

### Requirement: ParseError は line/column 情報を含む
`ParseError` はエラー生成時点で line、column、および必要な位置文脈を保持しなければならない (MUST)。公開 API は後付け全走査に依存せず、主要経路で生成されたエラーから直接位置情報を取得できなければならない。この requirement は位置情報の保持を扱い、layout 文脈の合成責務は `expect-error-trait` 側で定義する。

#### Scenario: エラー生成時点で行列が埋まる
- **WHEN** parser が複数行入力の途中で失敗する
- **THEN** 返される `ParseError` は追加のソース全文走査なしで line と column を保持している

#### Scenario: layout-sensitive grammar の位置診断
- **WHEN** 期待インデントを満たさない入力で grammar が失敗する
- **THEN** `ParseError` は失敗位置の line/column を反映する

## ADDED Requirements

### Requirement: 位置情報モデルは span 抽出に必要な責務を明示しなければならない
line、column、行アンカー、offset、span は、それぞれの責務と単位が明確でなければならない (MUST)。parser モジュールは「人間向け位置」と「入力スライス抽出のための anchor」を混同してはならない。

#### Scenario: 行スライス抽出の前提が明確である
- **WHEN** エラー行の抜粋や multiline block の行切り出しを設計する
- **THEN** 実装者は line、column、行アンカー、offset のどれを使うべきか仕様から判断できる
