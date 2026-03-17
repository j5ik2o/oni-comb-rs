## ADDED Requirements

### Requirement: many0_fold は初期値と関数でゼロ個以上の要素を畳み込む
`many0_fold(init, f)` は、パーサーが Backtrack するまで繰り返し実行し、各成功結果を `f(acc, item)` で畳み込む。Backtrack で停止し、累積値を返す。Cut / Incomplete / ZeroProgress はそのまま伝播する。アロケーションは発生しない。

#### Scenario: 要素が0個の場合は初期値を返す
- **WHEN** パーサーが最初の試行で Backtrack する
- **THEN** 初期値がそのまま返される

#### Scenario: 要素が複数ある場合は畳み込み結果を返す
- **WHEN** パーサーが3回成功して4回目で Backtrack する
- **THEN** `f(f(f(init, item1), item2), item3)` の結果が返される

#### Scenario: Cut エラーはそのまま伝播する
- **WHEN** パーサーの実行中に Cut エラーが発生する
- **THEN** Cut エラーがそのまま返される（畳み込み途中の値は破棄）

#### Scenario: ZeroProgress は検出してエラーにする
- **WHEN** パーサーが入力を消費せずに成功する
- **THEN** ZeroProgress エラーが返される（無限ループ防止）

### Requirement: many1_fold は初期値と関数で1個以上の要素を畳み込む
`many1_fold(init, f)` は、最初の要素は必須とし、以降は `many0_fold` と同じ振る舞いをする。最初の要素取得に失敗した場合はそのエラーをそのまま返す。

#### Scenario: 要素が1個の場合
- **WHEN** パーサーが1回成功して2回目で Backtrack する
- **THEN** `f(init, item1)` の結果が返される

#### Scenario: 要素が0個の場合はエラー
- **WHEN** パーサーが最初の試行で Backtrack する
- **THEN** Backtrack エラーがそのまま返される

### Requirement: sep_by0_fold は初期値と関数でセパレータ区切りのゼロ個以上の要素を畳み込む
`sep_by0_fold(sep, init, f)` は、セパレータで区切られた要素をゼロ個以上畳み込む。セパレータの出力は破棄する。

#### Scenario: 要素が0個の場合は初期値を返す
- **WHEN** 最初の要素パーサーが Backtrack する
- **THEN** 初期値がそのまま返される

#### Scenario: セパレータ後の要素が失敗した場合はセパレータ前まで巻き戻す
- **WHEN** セパレータ成功後に要素パーサーが Backtrack する
- **THEN** セパレータ消費前の checkpoint まで巻き戻し、それまでの畳み込み結果を返す

#### Scenario: 要素が複数ある場合は畳み込み結果を返す
- **WHEN** `1,2,3` を digit パーサーと `,` セパレータで畳み込む
- **THEN** `f(f(f(init, 1), 2), 3)` の結果が返される

### Requirement: sep_by1_fold は初期値と関数でセパレータ区切りの1個以上の要素を畳み込む
`sep_by1_fold(sep, init, f)` は、最初の要素は必須とし、以降は `sep_by0_fold` と同じ振る舞いをする。

#### Scenario: 要素が0個の場合はエラー
- **WHEN** 最初の要素パーサーが Backtrack する
- **THEN** Backtrack エラーがそのまま返される

#### Scenario: 要素が1個の場合
- **WHEN** 要素パーサーが1回成功し、セパレータが Backtrack する
- **THEN** `f(init, item1)` の結果が返される

### Requirement: fold 系コンビネータは alloc 不要で動作する
fold 系コンビネータ（`many0_fold`, `many1_fold`, `sep_by0_fold`, `sep_by1_fold`）は `Vec`, `Box`, `String` 等のヒープアロケーションを一切使用しない。

#### Scenario: core-only 環境でコンパイルできる
- **WHEN** `alloc` クレートを使用せずにビルドする
- **THEN** fold 系コンビネータのみを使用するコードがコンパイルに成功する

### Requirement: 既存の many0 / many1 は内部的に fold で実装される
既存の `many0()` / `many1()` は内部的に `ManyFold` を使用して実装される。外部 API（戻り値が `Vec<T>`）は変更しない。

#### Scenario: many0 の振る舞いが既存と同一
- **WHEN** 既存の `many0` テストスイートを実行する
- **THEN** すべてのテストが変更なしで通過する

#### Scenario: many0 の戻り値型が impl Parser で隠される
- **WHEN** `parser.many0()` を呼び出す
- **THEN** 戻り値は `impl Parser<I, Output = Vec<P::Output>, Error = P::Error>` として扱える
