## ADDED Requirements

### Requirement: 全 YAML パーサー関数は impl Parser を返す形式にする
`block_value`, `block_sequence`, `block_mapping`, `flow_value`, `flow_sequence`, `flow_mapping`, `yaml_scalar`, `block_scalar`, `yaml_document`, `yaml_documents` は全て `fn() -> impl Parser<YamlInput<'_>, Output = ..., Error = ParseError>` 形式とする。引数で `&mut StrInput` や `&mut ParseContext` を受け取らない。

#### Scenario: flow_sequence がコンビネータパイプラインで記述される
- **WHEN** `flow_sequence()` のソースコードを確認する
- **THEN** `char('[').zip_right(...).zip_left(...)` 形式のパイプラインで記述されており、`parse_next` の直接呼び出しがない

#### Scenario: block_value が with_indent と合成可能
- **WHEN** `with_indent(4, block_value())` を `YamlInput` に適用する
- **THEN** インデント4以上のブロック値が正しくパースされる

### Requirement: parse_next の直接呼び出しは公開 API のみ
`parse_next` を `&mut input` に対して直接呼び出すのは `parse()` と `parse_documents()` の2箇所のみとする。内部パーサーは全てコンビネータ合成で構築する。

#### Scenario: 内部モジュールに parse_next 呼び出しがない
- **WHEN** `block.rs`, `flow.rs`, `scalar.rs`, `multiline.rs`, `document.rs` のソースコードを検索する
- **THEN** `parse_next` の呼び出しが存在しない

### Requirement: 既存テストが全て通る
書き直し後も既存の35テスト（スカラー、フロー、ブロック、マルチライン、アンカー、ドキュメント、コメント、タグ）が全て通ること。

#### Scenario: 全テスト通過
- **WHEN** `cargo test -p oni-comb-yaml` を実行する
- **THEN** 全テストが通過し、failure が 0 件
