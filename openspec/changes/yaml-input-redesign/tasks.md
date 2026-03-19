## 1. ExpectError 改修 (parser クレート)

- [ ] 1.1 `ExpectError` トレイトに `from_expected_at<I: Input>(input: &I, expected: Expected) -> Self` メソッドを追加する
- [ ] 1.2 `ParseError` に `from_expected_at` を実装する（`input.offset()`, `input.line()`, `input.column()` を取得）
- [ ] 1.3 `MinimalError` に `from_expected_at` を実装する（`input.offset()` のみ）
- [ ] 1.4 既存の `from_expected` に `#[deprecated]` を付与する
- [ ] 1.5 `ParseError::fill_location_from_src` を削除する
- [ ] 1.6 全プリミティブパーサー (`sym`, `satisfy`, `one_of`, `none_of`, `eof`, `any`, `not_a`, `take`, `take_while*`, `take_till*`, `seq`) の `from_expected` → `from_expected_at` を移行する
- [ ] 1.7 全コンビネータ (`not`, `repeat`, `map_res`, `context`) の `from_expected` → `from_expected_at` を移行する
- [ ] 1.8 テキストパーサー (`char`, `tag`, `integer`, `float`, `identifier`, `quoted_string`, `whitespace`, `escaped`) の移行
- [ ] 1.9 `StrCheckpoint` / `ByteCheckpoint` の `line_start` フィールドに doc comment を追加する
- [ ] 1.10 parser クレートの全テスト通過確認

## 2. 下流クレート移行

- [ ] 2.1 json クレート: `fill_location_from_src` 呼び出しを削除（エラーに自動で行/列が入る）
- [ ] 2.2 uri クレート: deprecated 警告を解消（`from_expected` → `from_expected_at`）
- [ ] 2.3 crond クレート: deprecated 警告を解消
- [ ] 2.4 全クレートビルド・テスト通過確認

## 3. YamlInput 型の実装

- [ ] 3.1 `yaml_input.rs` を新規作成。`YamlInput<'a>` 構造体（`StrInput<'a>` + `HashMap<String, YamlValue>` + `Vec<usize>`）を定義する
- [ ] 3.2 `Input` トレイトを `YamlInput` に実装し、全メソッドを内部の `StrInput` に委譲する
- [ ] 3.3 YAML 固有メソッド `set_anchor`, `get_anchor`, `push_indent`, `pop_indent`, `current_min_indent` を実装する
- [ ] 3.4 `YamlInput` の単体テスト（Input 委譲、アンカー保存/取得、インデントスタック push/pop）

## 4. YAML 固有コンビネータの実装

- [ ] 4.1 `yaml_combinators.rs` を新規作成。`with_indent(n, parser)` コンビネータを実装する（push → parse → pop、失敗時も pop）
- [ ] 4.2 `indent_guard()` コンビネータを実装する（column-1 >= current_min_indent なら Ok(()), そうでなければ Backtrack）
- [ ] 4.3 `save_anchor(parser)` コンビネータを実装する（&name 検出 → 内部パーサー実行 → アンカー保存 → 値返却）
- [ ] 4.4 `resolve_alias()` パーサーを実装する（*name → アンカーマップ参照 → クローン返却）
- [ ] 4.5 `with_tag(parser)` コンビネータを実装する（!tag / !!tag 検出 → 内部パーサー → apply_tag）
- [ ] 4.6 コンビネータの単体テスト

## 5. scalar.rs の書き直し

- [ ] 5.1 `yaml_scalar` を `fn yaml_scalar() -> impl Parser<YamlInput, Output = YamlValue, Error = ParseError>` に変更する
- [ ] 5.2 `parse_single_quoted` をパイプラインスタイルに書き直す（fn_parser 経由可）
- [ ] 5.3 テスト通過確認

## 6. flow.rs の書き直し

- [ ] 6.1 `flow_value` を `fn flow_value() -> impl Parser<YamlInput, ...>` に変更。`resolve_alias().or(flow_sequence()).or(flow_mapping()).or(yaml_scalar())` 形式にする
- [ ] 6.2 `flow_sequence` をパイプラインで書き直す
- [ ] 6.3 `flow_mapping` をパイプラインで書き直す
- [ ] 6.4 `ctx` 引数を全て削除する
- [ ] 6.5 テスト通過確認

## 7. multiline.rs の書き直し

- [ ] 7.1 `block_scalar` を `fn block_scalar() -> impl Parser<YamlInput, ...>` に変更する（fn_parser 経由可）
- [ ] 7.2 テスト通過確認

## 8. block.rs の書き直し

- [ ] 8.1 `block_value` を `fn block_value() -> impl Parser<YamlInput, ...>` に変更。`with_tag(save_anchor(indent_guard().zip_right(block_inner())))` 形式にする
- [ ] 8.2 `block_sequence` を `with_indent` + パイプラインで書き直す
- [ ] 8.3 `block_mapping` を `with_indent` + パイプラインで書き直す
- [ ] 8.4 `min_indent` と `ctx` 引数を全て削除する
- [ ] 8.5 テスト通過確認

## 9. document.rs の書き直し

- [ ] 9.1 `yaml_document` を `fn yaml_document() -> impl Parser<YamlInput, ...>` に変更する
- [ ] 9.2 `yaml_documents` を `fn yaml_documents() -> impl Parser<YamlInput, Output = Vec<YamlValue>, ...>` に変更する
- [ ] 9.3 テスト通過確認

## 10. 統合と最終確認

- [ ] 10.1 `lib.rs` を更新: `YamlInput::new(src)` を使い、公開 API を維持する
- [ ] 10.2 `common.rs` を `YamlInput` 対応に変更する
- [ ] 10.3 `context.rs` を削除し `YamlInput` に統合する
- [ ] 10.4 `docs/known-issues.md` を削除する（4件全て解決済み）
- [ ] 10.5 全テスト通過確認（parser + json + yaml + uri + crond）
- [ ] 10.6 `RUSTFLAGS="-D warnings" cargo clippy --workspace` 通過確認
