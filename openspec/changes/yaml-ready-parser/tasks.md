## 1. Acceptance Criteria First

- [x] 1.1 `YAML-ready` の禁止事項をテスト方針として文書化する（下流 grammar 実装で `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、litmus grammar 段階での `fn_parser` 使用を許可しない）
- [x] 1.2 block list / indent nesting / flow-block switching / multiline block / block scalar header / document boundary / simple-key gating / simple-key backtrack / flow plain scalar boundary / indent error の litmus grammar 一覧を確定する
- [x] 1.3 litmus grammar 群を parser モジュール単体の acceptance test として追加する

## 2. State Model Redesign

- [ ] 2.1 parser core の state と parse 後に扱う downstream semantic data を分離する設計を確定する
- [ ] 2.2 `or`、`attempt`、`optional`、`many*`、`sep_by*` で layout state も安全に巻き戻る checkpoint モデルを実装する
- [ ] 2.3 既存の `InputStream` / `Checkpoint` API を新しい state model に合わせて更新する

## 3. Position and Error Model

- [ ] 3.1 `line`、`column`、`line_start`、`offset`、`span` の責務と単位をコード上の API とコメントに反映する
- [ ] 3.2 `ParseError` を生成時点で line/column/context を保持できる設計へ変更する
- [ ] 3.3 `ExpectError`、`MergeError`、`ContextError` の実装を、新しい位置文脈モデルに合わせて更新する
- [ ] 3.4 既存の line/column と error reporting のテストを新契約に合わせて更新する

## 4. Layout-sensitive Capabilities

- [ ] 4.1 行頭判定、期待インデント判定、flow/block 文脈判定、boolean flag 判定を public capability として設計する
- [ ] 4.2 `guard` だけでは表現できない layout-sensitive grammar 用 primitive / combinator を追加する（scoped flag 更新を含む）
- [ ] 4.3 litmus grammar が新 capability だけで記述できることを確認する（simple-key backtrack / flow plain scalar boundary / block scalar header を含む）

## 5. Readiness Verification

- [ ] 5.1 litmus grammar 実装から命令型 escape hatch が消えていることを review で確認する（`fn_parser` を含む）
- [ ] 5.2 `parser` モジュール単体で `YAML-ready` 受け入れ条件を満たしたことを docs と OpenSpec に反映する
- [ ] 5.3 `docs/known-issues.md` を更新し、`line_start` 単体ではなく checkpoint 可能な layout context の観点で整理し直す
- [x] 5.4 `yaml-parser` capability に `yaml-ready-parser` 通過後でなければ実装着手不可であることを明文化する
- [ ] 5.5 `fn_parser` を導入する場合は、宣言的実装が先に存在することと性能根拠があることを review で確認する
