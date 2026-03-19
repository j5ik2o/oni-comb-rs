## ADDED Requirements

### Requirement: YAML パーサーは YAML 1.2 Core Schema のスカラー型をパースする
null (`null`, `~`, 空), boolean (`true`/`false`), integer (10進/8進/16進), float (小数/指数/.inf/.nan), string (plain/single-quoted/double-quoted) を認識する。

#### Scenario: null リテラル
- **WHEN** `"null"` をスカラーとしてパースする
- **THEN** `YamlValue::Null` を返す

#### Scenario: boolean リテラル
- **WHEN** `"true"` をスカラーとしてパースする
- **THEN** `YamlValue::Bool(true)` を返す

#### Scenario: 整数 (10進)
- **WHEN** `"42"` をスカラーとしてパースする
- **THEN** `YamlValue::Integer(42)` を返す

#### Scenario: 整数 (16進)
- **WHEN** `"0xFF"` をスカラーとしてパースする
- **THEN** `YamlValue::Integer(255)` を返す

#### Scenario: 浮動小数点
- **WHEN** `"3.14"` をスカラーとしてパースする
- **THEN** `YamlValue::Float(3.14)` を返す

#### Scenario: 特殊浮動小数点 (.inf / .nan)
- **WHEN** `".inf"` をスカラーとしてパースする
- **THEN** `YamlValue::Float(f64::INFINITY)` を返す

#### Scenario: plain 文字列
- **WHEN** `"hello world"` をスカラーとしてパースする
- **THEN** `YamlValue::String("hello world")` を返す

### Requirement: YAML パーサーはブロックスタイルのマッピングとシーケンスをパースする
インデントベースのブロックマッピング (`key: value`) とブロックシーケンス (`- item`) をパースする。ネストはインデントレベルで判定する。

#### Scenario: ブロックマッピング
- **WHEN** `"key1: value1\nkey2: value2"` をパースする
- **THEN** 2エントリの `YamlValue::Mapping` を返す

#### Scenario: ネストしたブロックマッピング
- **WHEN** 以下をパースする:
  ```
  parent:
    child1: value1
    child2: value2
  ```
- **THEN** parent の値が2エントリの Mapping であるネスト構造を返す

#### Scenario: ブロックシーケンス
- **WHEN** `"- item1\n- item2\n- item3"` をパースする
- **THEN** 3要素の `YamlValue::Sequence` を返す

#### Scenario: シーケンスのネスト
- **WHEN** 以下をパースする:
  ```
  - - nested1
    - nested2
  - item2
  ```
- **THEN** 最初の要素が2要素の Sequence であるネスト構造を返す

### Requirement: YAML パーサーはフロースタイルをパースする
JSON 互換のフロースタイル: フローマッピング (`{key: value}`)、フローシーケンス (`[item1, item2]`)、およびそれらのネストをパースする。

#### Scenario: フローシーケンス
- **WHEN** `"[1, 2, 3]"` をパースする
- **THEN** 3要素の `YamlValue::Sequence` を返す

#### Scenario: フローマッピング
- **WHEN** `"{name: oni-comb, version: 2}"` をパースする
- **THEN** 2エントリの `YamlValue::Mapping` を返す

#### Scenario: フローとブロックの混在
- **WHEN** 以下をパースする:
  ```
  items: [1, 2, 3]
  nested:
    key: {a: 1, b: 2}
  ```
- **THEN** ブロックマッピング内にフローシーケンスとフローマッピングがネストした構造を返す

### Requirement: YAML パーサーはマルチライン文字列をパースする
リテラルブロック (`|`)、folded ブロック (`>`)、および chomping indicator (`-`, `+`) をサポートする。

#### Scenario: リテラルブロック (|)
- **WHEN** 以下をパースする:
  ```
  text: |
    line1
    line2
  ```
- **THEN** `YamlValue::String("line1\nline2\n")` を返す

#### Scenario: folded ブロック (>)
- **WHEN** 以下をパースする:
  ```
  text: >
    line1
    line2
  ```
- **THEN** `YamlValue::String("line1 line2\n")` を返す (改行が空白に折り畳まれる)

#### Scenario: strip chomping (|-)
- **WHEN** 以下をパースする:
  ```
  text: |-
    line1
    line2
  ```
- **THEN** `YamlValue::String("line1\nline2")` を返す（末尾改行なし）

### Requirement: YAML パーサーはアンカーとエイリアスをパースする
アンカー (`&name`) でノードに名前を付け、エイリアス (`*name`) で参照する。

#### Scenario: アンカーとエイリアスの基本
- **WHEN** 以下をパースする:
  ```
  default: &defaults
    adapter: postgres
  development:
    <<: *defaults
    database: dev_db
  ```
- **THEN** development が defaults の内容をマージした Mapping を返す

#### Scenario: 単純なエイリアス参照
- **WHEN** 以下をパースする:
  ```
  - &anchor value
  - *anchor
  ```
- **THEN** 2要素の Sequence で、両方とも `"value"` を返す

### Requirement: YAML パーサーはマルチドキュメントをパースする
`---` でドキュメント開始、`...` でドキュメント終了を示す。1入力に複数ドキュメントを含むことができる。

#### Scenario: 複数ドキュメント
- **WHEN** 以下をパースする:
  ```
  ---
  doc1: value1
  ---
  doc2: value2
  ```
- **THEN** 2つの `YamlValue` を含むドキュメントリストを返す

#### Scenario: ドキュメント終了マーカー
- **WHEN** 以下をパースする:
  ```
  ---
  data: value
  ...
  ```
- **THEN** 1つのドキュメントを返す

### Requirement: YAML パーサーはタグをパースする
`!!str`, `!!int`, `!!float` 等の Core Schema タグ、およびカスタムタグ (`!custom`) を認識する。

#### Scenario: Core Schema タグで型を強制する
- **WHEN** `"!!str 42"` をパースする
- **THEN** `YamlValue::String("42")` を返す（数値ではなく文字列として解釈）

#### Scenario: カスタムタグを保持する
- **WHEN** `"!custom value"` をパースする
- **THEN** タグ `!custom` と値 `"value"` を持つ Tagged ノードを返す

### Requirement: YAML パーサーはコメントをサポートする
`#` から行末まではコメントとして無視する。

#### Scenario: 行末コメント
- **WHEN** `"key: value # this is a comment"` をパースする
- **THEN** `key: value` のマッピングを返し、コメントは無視される

#### Scenario: 行全体がコメント
- **WHEN** 以下をパースする:
  ```
  # header comment
  key: value
  ```
- **THEN** `key: value` のマッピングを返す

### Requirement: YAML パーサーはエラー時に行/列情報を含む
パースエラーは行/列位置、期待されたトークン、コンテキスト (インデントレベル等) を含む。

#### Scenario: インデントエラー
- **WHEN** 以下をパースしてインデントが不正な場合:
  ```
  parent:
    child1: value1
   child2: value2
  ```
- **THEN** エラーは line=3 付近の位置情報と、期待されたインデントレベルの情報を含む
