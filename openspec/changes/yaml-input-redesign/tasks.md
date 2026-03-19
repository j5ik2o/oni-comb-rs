## 1. YamlInput 型の実装

- [ ] 1.1 `yaml_input.rs` を新規作成。`YamlInput<'a>` 構造体（`StrInput<'a>` + `HashMap<String, YamlValue>` + `Vec<usize>`）を定義する
- [ ] 1.2 `Input` トレイトを `YamlInput` に実装し、全メソッドを内部の `StrInput` に委譲する
- [ ] 1.3 YAML 固有メソッド `set_anchor`, `get_anchor`, `push_indent`, `pop_indent`, `current_min_indent` を実装する
- [ ] 1.4 `YamlInput` の単体テスト（Input 委譲、アンカー保存/取得、インデントスタック push/pop）

## 2. YAML 固有コンビネータの実装

- [ ] 2.1 `yaml_combinators.rs` を新規作成。`with_indent(n, parser)` コンビネータを実装する（push → parse → pop、失敗時も pop）
- [ ] 2.2 `indent_guard()` コンビネータを実装する（column-1 >= current_min_indent なら Ok(()), そうでなければ Backtrack）
- [ ] 2.3 `save_anchor(parser)` コンビネータを実装する（&name 検出 → 内部パーサー実行 → アンカー保存 → 値返却）
- [ ] 2.4 `resolve_alias()` パーサーを実装する（*name → アンカーマップ参照 → クローン返却）
- [ ] 2.5 コンビネータの単体テスト

## 3. scalar.rs の書き直し

- [ ] 3.1 `yaml_scalar` を `fn yaml_scalar() -> impl Parser<YamlInput, Output = YamlValue, Error = ParseError>` に変更する
- [ ] 3.2 `parse_single_quoted` をパイプラインスタイルに書き直す（fn_parser 経由可）
- [ ] 3.3 テスト通過確認

## 4. flow.rs の書き直し

- [ ] 4.1 `flow_value` を `fn flow_value() -> impl Parser<YamlInput, ...>` に変更。`resolve_alias().or(flow_sequence()).or(flow_mapping()).or(yaml_scalar())` 形式にする
- [ ] 4.2 `flow_sequence` をパイプラインで書き直す: `char('[').zip_right(ws(flow_value()).sep_by0(ws(char(',')))).zip_left(ws(char(']')))`
- [ ] 4.3 `flow_mapping` をパイプラインで書き直す: member の sep_by0 パターン
- [ ] 4.4 `ctx` 引数を全て削除し、`YamlInput` 経由でアクセスするようにする
- [ ] 4.5 テスト通過確認

## 5. multiline.rs の書き直し

- [ ] 5.1 `block_scalar` を `fn block_scalar() -> impl Parser<YamlInput, ...>` に変更する（fn_parser 経由可、内部ロジックはインデント検出のため手続き的でも可）
- [ ] 5.2 テスト通過確認

## 6. block.rs の書き直し

- [ ] 6.1 `block_value` を `fn block_value() -> impl Parser<YamlInput, ...>` に変更。`save_anchor(indent_guard().zip_right(block_inner()))` 形式にする
- [ ] 6.2 `block_sequence` を `with_indent` + パイプラインで書き直す
- [ ] 6.3 `block_mapping` を `with_indent` + パイプラインで書き直す
- [ ] 6.4 `min_indent` と `ctx` 引数を全て削除する
- [ ] 6.5 テスト通過確認

## 7. document.rs の書き直し

- [ ] 7.1 `yaml_document` を `fn yaml_document() -> impl Parser<YamlInput, ...>` に変更する
- [ ] 7.2 `yaml_documents` を `fn yaml_documents() -> impl Parser<YamlInput, Output = Vec<YamlValue>, ...>` に変更する
- [ ] 7.3 テスト通過確認

## 8. lib.rs と common.rs の更新

- [ ] 8.1 `lib.rs` を更新: `YamlInput::new(src)` を使い、`parse` と `parse_documents` の公開 API を維持する
- [ ] 8.2 `common.rs` の `skip_inline_ws`, `skip_ws_and_comments`, `current_indent` を `YamlInput` 対応に変更する
- [ ] 8.3 `context.rs` を削除し、`YamlInput` に統合する
- [ ] 8.4 全35テスト通過確認
- [ ] 8.5 `RUSTFLAGS="-D warnings" cargo clippy --workspace` 通過確認
