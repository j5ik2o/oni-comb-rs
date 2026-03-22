## Why

`yaml-ready-parser` で parser core が layout-sensitive grammar を支えられることは実証できたが、まだ実際の downstream YAML parser crate は存在しない。次はその readiness を机上ではなく実コードで検証するために、YAML 全体ではなく最小の MVP subset を持つ parser を追加し、既存 public contract だけで成立することを確認する必要がある。

## What Changes

- `modules/yaml` crate を追加し、`oni-comb-yaml` として最小の downstream YAML parser を提供する
- single document を対象に、block mapping / block sequence / flow mapping / flow sequence を parse できる MVP grammar を追加する
- scalar は plain / single-quoted / double-quoted string、`null`、`bool`、10 進 integer に限定し、comment を無視できるようにする
- mapping key は MVP では supported scalar subset に限定し、explicit key (`? key`) や collection-valued key は対象外とする
- 実装は関数型・宣言的・public combinator chain のみで記述し、custom `Parser` 実装、`InputStream` wrapper、`parse_next` / `checkpoint/reset` / `next_token` 直呼び、戻り値破棄による命令型制御を用いない
- 既存 public contract だけでは declarative に表現できない grammar が見つかった場合は、無理に実装せず不足する generic capability または過大な scope をフィードバックする
- line / column / context を含む parse error を downstream YAML parser でも利用できることを確認する
- block scalar (`|`, `>`)、anchor / alias、tag、multi-document、merge key、advanced numeric schema は MVP の対象外とする

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `yaml-parser`: placeholder だった downstream YAML capability を、MVP subset の具体的な parser / AST / error 契約へ更新する

## Impact

- `modules/yaml`: 新規 crate、parser 実装、AST、tests の追加対象
- `Cargo.toml`: workspace member の追加が必要
- `openspec/specs/yaml-parser/spec.md`: MVP subset の requirement へ更新する
- `modules/parser/tests/yaml_ready_acceptance.rs`: parser core の成立性を示す参照資料として扱うが、この change では wrapper / imperative helper を production 実装へ持ち込まない
- public API: `oni-comb-yaml` の `parse` / `parse_value` 相当 API と YAML AST の公開が必要
- 表現不能ケース: parser core の generic capability 不足か、MVP scope の切り方の問題かを切り分けてフィードバックする必要がある
