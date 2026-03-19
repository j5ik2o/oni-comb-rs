## ADDED Requirements

### Requirement: JSON パーサーは RFC 8259 の全データ型をパースする
`modules/json` クレートは null, boolean, number, string, array, object の全 JSON データ型をパースする。ネスト（再帰構造）を完全にサポートする。

#### Scenario: null をパースする
- **WHEN** `"null"` をパースする
- **THEN** `JsonValue::Null` を返す

#### Scenario: boolean をパースする
- **WHEN** `"true"` をパースする
- **THEN** `JsonValue::Bool(true)` を返す

#### Scenario: 数値 (整数) をパースする
- **WHEN** `"42"` をパースする
- **THEN** `JsonValue::Number(42.0)` を返す

#### Scenario: 数値 (小数・指数) をパースする
- **WHEN** `"1.5e10"` をパースする
- **THEN** `JsonValue::Number(1.5e10)` を返す

#### Scenario: 文字列をパースする
- **WHEN** `"\"hello\""` をパースする
- **THEN** `JsonValue::String("hello")` を返す

#### Scenario: 配列をパースする
- **WHEN** `"[1, \"two\", true, null]"` をパースする
- **THEN** 4要素の `JsonValue::Array` を返す

#### Scenario: オブジェクトをパースする
- **WHEN** `"{\"key\": \"value\"}"` をパースする
- **THEN** `JsonValue::Object` を返す

#### Scenario: 深いネストをパースする
- **WHEN** `"[[[[1]]]]"` をパースする
- **THEN** 4段ネストの配列を正しくパースする

### Requirement: JSON 文字列はエスケープシーケンスを完全にサポートする
`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX` (サロゲートペアを含む) を正しくデコードする。

#### Scenario: 基本エスケープシーケンス
- **WHEN** `"\"hello\\nworld\""` をパースする
- **THEN** `JsonValue::String("hello\nworld")` を返す

#### Scenario: Unicode エスケープ (BMP)
- **WHEN** `"\"\\u2192\""` をパースする
- **THEN** `JsonValue::String("→")` を返す

#### Scenario: サロゲートペア
- **WHEN** `"\"\\uD83D\\uDE00\""` をパースする
- **THEN** `JsonValue::String("😀")` を返す

#### Scenario: 不正なサロゲートペアでエラー
- **WHEN** `"\"\\uD83D\""` (ハイサロゲートのみ) をパースする
- **THEN** パースエラーを返す

### Requirement: JSON パーサーはエラー時に行/列情報を含む
パースエラーは位置 (行/列)、期待されたトークン、コンテキストを含む。

#### Scenario: エラーに行/列が含まれる
- **WHEN** `"{\n  \"key\": }"` をパースしてエラーになる
- **THEN** エラーは line=2 付近の位置情報を含む

#### Scenario: エラーに期待トークンが含まれる
- **WHEN** `"[1, ]"` をパースしてエラーになる
- **THEN** エラーは期待されたトークン（JSON value）の情報を含む

### Requirement: JSON パーサーは先頭/末尾の空白を許容する
RFC 8259 に従い、トップレベル値の前後に空白を許容する。

#### Scenario: 前後の空白を許容する
- **WHEN** `"  { \"a\": 1 }  "` をパースする
- **THEN** 正常にパースできる

#### Scenario: 入力全体を消費する
- **WHEN** `"{} trailing"` をパースする
- **THEN** trailing テキストがあるためエラーを返す
