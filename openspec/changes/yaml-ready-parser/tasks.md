## 1. Acceptance Criteria First

- [x] 1.1 `YAML-ready` の禁止事項をテスト方針として文書化する（top-level の下流 grammar 定義で `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、litmus grammar 段階での `fn_parser` 使用を許可しない。既存公開契約を使う helper parser / `InputStream` wrapper の内部実装は別扱いとする）
- [x] 1.2 block list / indent nesting / flow-block switching / multiline block / block scalar header / document boundary / simple-key gating / simple-key backtrack / flow plain scalar boundary / indent error の litmus grammar 一覧を確定する
- [x] 1.3 litmus grammar 群を parser モジュール単体の acceptance test として追加する

## 2. State Model Redesign

- [x] 2.1 parser core の state と parse 後に扱う downstream semantic data を分離する設計を確定する
- [x] 2.2 `or`、`attempt`、`optional`、`many*`、`sep_by*` で layout state も安全に巻き戻る checkpoint モデルを実装する
- [x] 2.3 既存の `InputStream` / `Checkpoint` API を新しい state model に合わせて更新する

## 3. Position and Error Model

- [x] 3.1 `line`、`column`、`line_start`、`offset`、`span` の責務と単位をコード上の API とコメントに反映する
- [x] 3.2 `ParseError` を生成時点で line/column/context を保持できる設計へ変更する
- [x] 3.3 `ExpectError`、`MergeError`、`ContextError` の実装を、新しい位置文脈モデルに合わせて更新する
- [x] 3.4 既存の line/column と error reporting のテストを新契約に合わせて更新する

## 4. Layout-sensitive Capabilities

- [x] 4.1 行頭判定、期待インデント判定、flow/block 文脈判定、boolean flag 判定を、parser モジュールへの YAML 専用 Layout API 追加ではなく、既存の公開契約と downstream 側の合成で実現できることを設計に反映する
- [x] 4.2 litmus grammar で表現不能なケースが残る場合に限り、YAML 非依存の最小 generic primitive / combinator の追加要否を評価する。既存 capability の組み合わせで充足できる場合は追加しない
- [x] 4.3 litmus grammar が新 capability だけで記述できることを確認する（simple-key backtrack / flow plain scalar boundary / block scalar header を含む）

## 5. Readiness Verification

- [x] 5.1 top-level litmus grammar 実装から命令型 escape hatch が消えていることを review で確認する（`fn_parser` を含む。既存公開契約を使う helper parser / `InputStream` wrapper の内部実装は別扱い）
- [x] 5.2 `parser` モジュール単体で `YAML-ready` 受け入れ条件を満たしたことを docs と OpenSpec に反映する
- [x] 5.3 `docs/known-issues.md` を更新し、`line_start` 単体ではなく checkpoint 可能な layout context の観点で整理し直す
- [x] 5.4 `yaml-parser` capability に `yaml-ready-parser` 通過後でなければ実装着手不可であることを明文化する
- [x] 5.5 `fn_parser` を導入する場合は、宣言的実装が先に存在することと性能根拠があることを review で確認する
