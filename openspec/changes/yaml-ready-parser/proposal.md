## Why

oni-comb-parser は JSON などの直列的な文法には十分使えるが、YAML 1.2 のような layout-sensitive grammar を combinator として自然に記述できる保証がまだない。YAML 実装を先に始めると、下流クレート側で `parse_next`、`checkpoint/reset`、戻り値破棄、手動状態管理を濫用して parser core の不足を埋める設計破綻が起きるため、まず parser モジュール単体の `YAML-ready` 条件を定義し、その条件を満たすように設計を鍛え直す必要がある。

## What Changes

- YAML パーサーはまだ実装しない。代わりに parser モジュール単体に対する `YAML-ready` の受け入れ条件を定義する
- 下流 grammar 実装では `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄を禁止し、public combinator のメソッドチェインのみで文法を記述できることを契約にする
- layout-sensitive grammar に必要な parser capability を定義する。対象には layout context、checkpoint 対象、行頭/インデント/flow-block 文脈観測、位置情報と診断モデルを含む
- YAML 本体の代わりに、YAML 実装に必要な能力を検証する litmus grammar 群を定義し、parser モジュール単体の acceptance criteria とする
- **BREAKING**: 既存の位置情報モデルとエラーモデルは、`YAML-ready` 条件を満たすために責務・単位・生成タイミングの再設計対象にする

## Capabilities

### New Capabilities
- `yaml-ready-parser`: parser モジュール単体が layout-sensitive grammar を public combinator だけで記述できることを定義する
- `layout-sensitive-parsing`: 行頭、インデント、flow/block 文脈、checkpoint 可能な layout context を扱う parser capability を定義する

### Modified Capabilities
- `line-column-tracking`: line/column/line_start/span の責務と単位を、YAML-ready の位置情報モデルに合わせて見直す
- `expect-error-trait`: エラー生成時点で位置情報と文脈を取得できるよう要件を見直す

## Impact

- `modules/parser`: InputStream、Checkpoint、error model、public combinator 設計、テスト方針の再設計対象
- `docs/known-issues.md`: `line_start` 問題だけでなく、checkpoint 可能な layout context 欠如を主要論点として整理し直す必要がある
- `openspec/specs/line-column-tracking/spec.md`: 位置情報の責務・単位・公開契約の変更が必要
- `openspec/specs/expect-error-trait/spec.md`: エラー生成 API の変更が必要
- 将来の `modules/yaml`: parser readiness 通過後に初めて着手する対象
