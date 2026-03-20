## MODIFIED Requirements

### Requirement: YAML パーサー実装は yaml-ready-parser 通過後にのみ着手できる
YAML パーサー capability の実装は、parser モジュールが `yaml-ready-parser` capability の受け入れ条件を満たした後にのみ着手してよい (MUST)。`yaml-ready-parser` を満たさない段階では、`modules/yaml` 側で parser core の不足を補うための命令型 escape hatch、手動 checkpoint 管理、独自 layout state 管理を導入してはならない。

#### Scenario: readiness 未達では YAML 実装は完了扱いにならない
- **WHEN** `yaml-ready-parser` capability が未達の状態で `modules/yaml` の実装を進める
- **THEN** その実装は `yaml-parser` capability の達成として扱ってはならない

### Requirement: YAML パーサーは parser core の汎用 capability を組み合わせて実装されなければならない
YAML パーサーは YAML 固有機能を parser core に直接追加するのではなく、parser モジュールが提供する汎用 capability を組み合わせて実装されなければならない (MUST)。block / flow / indent / multiline block / document boundary のような YAML 構文要件は downstream 実装で汎用 capability を構成して満たし、anchors / aliases / tags のような YAML 固有の意味解釈は parser core の checkpoint model と切り離して設計しなければならない。

#### Scenario: YAML 固有要件を汎用 capability で満たす
- **WHEN** YAML パーサーが block / flow / indent / multiline block / document boundary を実装する
- **THEN** 実装は parser core の汎用 capability の組み合わせで成立し、YAML 特化 API を parser core に直載せしない

#### Scenario: YAML の難所も汎用 capability で満たす
- **WHEN** YAML パーサーが simple-key rollback、flow plain scalar boundary、block scalar header を実装する
- **THEN** 実装は命令型 escape hatch ではなく parser core の汎用 capability の組み合わせで成立する

#### Scenario: anchor と alias は parse 後フェーズで解釈できる
- **WHEN** YAML パーサーが anchor や alias を含む入力を扱う
- **THEN** parser はそれらをまず YAML 構文要素としてパースでき、意味解釈は parse 後の resolver / AST 構築フェーズへ委譲できる
