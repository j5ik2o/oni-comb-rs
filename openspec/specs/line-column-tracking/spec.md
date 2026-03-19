## ADDED Requirements

### Requirement: Input トレイトは line と column を提供する
`Input` トレイトに `line() -> usize` と `column() -> usize` メソッドを追加する。初期値は line=1, column=1 (1-origin)。

#### Scenario: 初期状態は line=1, column=1
- **WHEN** `StrInput::new("hello")` を作成する
- **THEN** `line()` は 1、`column()` は 1 を返す

#### Scenario: ByteInput の初期状態も line=1, column=1
- **WHEN** `ByteInput::new(b"hello")` を作成する
- **THEN** `line()` は 1、`column()` は 1 を返す

### Requirement: next_token で改行を検出し line/column を更新する
`next_token` が `\n` を返した時、line をインクリメントし column を 1 にリセットする。`\n` 以外のトークンでは column をインクリメントする。

#### Scenario: StrInput で改行を越える
- **WHEN** `StrInput::new("ab\ncd")` から3トークン (`a`, `b`, `\n`) を消費する
- **THEN** `line()` は 2、`column()` は 1 を返す

#### Scenario: StrInput の column は char 単位で数える
- **WHEN** `StrInput::new("café")` から3トークン (`c`, `a`, `f`) を消費する
- **THEN** `column()` は 4 を返す（`é` は1 char なので次に消費すれば column=5）

#### Scenario: ByteInput の column は byte 単位で数える
- **WHEN** `ByteInput::new(b"abcd")` から2トークン (`a`, `b`) を消費する
- **THEN** `column()` は 3 を返す

#### Scenario: 複数行を越える
- **WHEN** `StrInput::new("a\nb\nc")` から全5トークンを消費する
- **THEN** 最終状態は `line()` = 3、`column()` = 2

### Requirement: Checkpoint は line/column を含み reset で復元される
`checkpoint()` は現在の offset, line, column, line_start を保存する。`reset(cp)` で全て復元される。

#### Scenario: checkpoint と reset で行/列が復元される
- **WHEN** `StrInput::new("ab\ncd")` で2トークン消費後に checkpoint を取り、さらに2トークン (`\n`, `c`) 消費後に reset する
- **THEN** reset 後 `line()` は 1、`column()` は 3、`offset()` は 2

#### Scenario: Checkpoint の Ord は offset で比較する
- **WHEN** offset=5 の checkpoint と offset=3 の checkpoint を比較する
- **THEN** offset=5 の方が大きい（`cp1 > cp2`）

### Requirement: ParseError は line/column 情報を含む
`ParseError` に `line` と `column` フィールドを追加する。エラー生成時に Input の現在位置から取得する。

#### Scenario: エラーメッセージに行/列が含まれる
- **WHEN** `StrInput::new("ab\nxy")` の3行目でパースエラーが発生する
- **THEN** `ParseError` の `line` は 2、`column` はエラー発生位置の列を反映する
