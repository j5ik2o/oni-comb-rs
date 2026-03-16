# Tasks

## Phase 1: エラー型基盤

- [ ] `parser/src/error.rs`: `ParseError`, `Expected`, `MergeError`, `ContextError`
- [ ] `parser/src/lib.rs`: `error` モジュール登録
- [ ] `parser/src/fail.rs`: 変更なし（E はジェネリックのまま）

## Phase 2: text パーサーの移行

- [ ] `text/char.rs`: Error = String → ParseError
- [ ] `text/tag.rs`: Error = String → ParseError
- [ ] `text/satisfy.rs`: Error = String → ParseError
- [ ] `text/eof.rs`: Error = String → ParseError
- [ ] `text/take_while0.rs`: Error = String → ParseError
- [ ] `text/take_while1.rs`: Error = String → ParseError
- [ ] `text/identifier.rs`: Error = String → ParseError
- [ ] `text/integer.rs`: Error = String → ParseError
- [ ] `text/quoted_string.rs`: Error = String → ParseError
- [ ] `text/escaped.rs`: Error = String → ParseError
- [ ] `text/whitespace.rs`: 変更確認
- [ ] `text/lexeme.rs`: 変更確認

## Phase 3: コンビネータの強化

- [ ] `combinator/or.rs`: MergeError bound 追加、Backtrack マージロジック
- [ ] `combinator/context.rs`: Context 具象型 + Parser 実装 (新規)
- [ ] `parser_ext.rs`: `.context()` メソッド追加
- [ ] `combinator/recursive.rs`: Error 型の調整

## Phase 4: テスト更新

- [ ] 既存テストの Backtrack/Cut パターンマッチを ParseError に対応
- [ ] `tests/error_reporting.rs`: 位置・期待トークン・コンテキストの検証
- [ ] `tests/json_subset.rs`: JSON パースエラー時の位置と期待トークンの検証

## 完了確認

- [ ] `cargo test -p oni-comb-parser` 全テスト通過
- [ ] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
- [ ] COMMON.md, README.md のマイルストーン状態を更新
