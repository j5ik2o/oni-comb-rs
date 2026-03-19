## 1. ExpectError 改修 (parser クレート)

- [x] 1.1 `ExpectError` トレイトに `from_expected_with_location(position, line, column, expected)` メソッドを追加する
- [x] 1.2 `ParseError` に `from_expected_with_location` を実装する
- [x] 1.3 `MinimalError` に `from_expected_with_location` を実装する
- [x] 1.4 既存の `from_expected` に `#[deprecated]` を付与する
- [x] 1.5 `ParseError::fill_location_from_src` を削除する
- [x] 1.6 全プリミティブパーサーの `from_expected` → `from_expected_with_location` を移行する
- [x] 1.7 全コンビネータの移行
- [x] 1.8 テキストパーサーの移行
- [x] 1.9 `StrCheckpoint` / `ByteCheckpoint` の `line_start` フィールドに doc comment を追加する
- [x] 1.10 parser クレートの全テスト通過確認

## 2. 下流クレート移行

- [x] 2.1 json クレート: `fill_location_from_src` 削除、`from_expected_with_location` に移行
- [x] 2.2 uri クレート: `from_expected` → `from_expected_with_location` 移行
- [x] 2.3 crond クレート: 同上
- [x] 2.4 全クレートビルド・テスト通過確認

## 3. YamlInput 型の実装

- [x] 3.1 `yaml_input.rs` を新規作成。`YamlInput<'a>` 構造体（`StrInput<'a>` + `HashMap<String, YamlValue>` + `Vec<usize>`）を定義する
- [x] 3.2 `Input` トレイトを `YamlInput` に実装し、全メソッドを内部の `StrInput` に委譲する
- [x] 3.3 YAML 固有メソッド `set_anchor`, `get_anchor`, `push_indent`, `pop_indent`, `current_min_indent` を実装する
- [x] 3.4 `YamlInput` の単体テスト（5テスト通過）

## 4. YAML 固有コンビネータの実装

- [x] 4.1 `yaml_combinators.rs` に `with_indent(n, parser)` コンビネータを実装
- [x] 4.2 `indent_guard()` コンビネータを実装
- [x] 4.3 `save_anchor(parser)` コンビネータを実装
- [x] 4.4 `resolve_alias()` パーサーを実装
- [x] 4.5 `with_tag(parser)` コンビネータを実装
- [x] 4.6 コンビネータの単体テスト（9テスト通過）

## 5. scalar.rs の書き直し

- [x] 5.1 `yaml_scalar` を `YamlInput` 対応に変更（fn_parser + inner_mut 経由）
- [x] 5.2 `parse_single_quoted` を YamlInput 対応に変更
- [x] 5.3 テスト通過確認

## 6. flow.rs の書き直し

- [x] 6.1 `flow_value` を `YamlInput` 対応に変更。ctx パラメータ除去
- [x] 6.2 `flow_sequence` を YamlInput 対応に変更（inner_mut 経由で char パーサー使用）
- [x] 6.3 `flow_mapping` を同様に変更
- [x] 6.4 `ctx` 引数を全て削除
- [x] 6.5 テスト通過確認

## 7. multiline.rs の書き直し

- [x] 7.1 `block_scalar` を `YamlInput` 対応に変更
- [x] 7.2 テスト通過確認

## 8. block.rs の書き直し

- [x] 8.1 `block_value` を YamlInput 対応に変更。ctx と min_indent を除去し input.current_min_indent()/push_indent/pop_indent を使用
- [x] 8.2 `block_sequence` を push_indent/pop_indent で書き直し
- [x] 8.3 `block_mapping` を同様に書き直し
- [x] 8.4 `min_indent` と `ctx` 引数を全て削除
- [x] 8.5 テスト通過確認

## 9. document.rs の書き直し

- [x] 9.1 `yaml_document` を YamlInput 対応に変更（ctx 除去）
- [x] 9.2 `yaml_documents` を同様に変更
- [x] 9.3 テスト通過確認

## 10. 統合と最終確認

- [x] 10.1 `lib.rs` を更新: `YamlInput::new(src)` を使用、公開 API 維持
- [x] 10.2 `common.rs` を `YamlInput` 対応に変更
- [x] 10.3 `context.rs` を削除し `YamlInput` に統合
- [x] 10.4 `docs/known-issues.md` を削除（4件全て解決済み）
- [x] 10.5 全テスト通過確認（49テスト: 14 + 35）
- [x] 10.6 `RUSTFLAGS="-D warnings" cargo clippy --workspace` 通過確認
