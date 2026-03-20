## ADDED Requirements

### Requirement: litmus grammar の公開面は declarative でなければならない
YAML-ready litmus grammar の top-level 定義は public combinator のメソッドチェインと downstream helper parser の公開面だけで記述されなければならない (MUST)。top-level litmus grammar は `parse_next` の直接呼び出し、`checkpoint/reset` の手動制御、入力状態を読んだ後に結果を捨てて自前分岐する実装、`fn_parser` のような命令型 escape hatch に依存してはならない。

#### Scenario: block sequence entry grammar stays declarative at top level
- **WHEN** downstream litmus grammar が block sequence entry (`- item`) を表現する
- **THEN** top-level grammar definition は `char('-')`, whitespace parser, scalar parser, context/attempt/cut などの公開 combinator 連結だけで書かれている

#### Scenario: mapping value grammar stays declarative at top level
- **WHEN** downstream litmus grammar が `key: value` の value 部分を表現する
- **THEN** top-level grammar definition は `.zip()`, `.zip_right()`, `.or()`, `.context()` などの public combinator chain だけで構築される

#### Scenario: top-level litmus grammar does not use fn_parser
- **WHEN** acceptance 用の downstream litmus grammar を review する
- **THEN** top-level grammar definitions に `fn_parser` 相当の命令型 escape hatch が存在しない

### Requirement: 下流 helper parser は公開契約の合成をカプセル化できる
downstream-owned helper parser や `InputStream` wrapper は、top-level litmus grammar の declarative 公開面を保つために、既存の公開契約を内部で合成して stateful adaptation をカプセル化してよい (MUST)。ただし、それは parser core に YAML 専用 primitive/combinator を追加する理由にはならない。

#### Scenario: helper parser encapsulates expected-indent state update
- **WHEN** downstream helper parser が expected indent を保存・更新しながら inner parser を呼び出す
- **THEN** top-level litmus grammar はその helper を通常の parser combinator として宣言的に組み合わせられる

#### Scenario: input wrapper encapsulates flow/block context
- **WHEN** downstream `InputStream` wrapper が flow level や simple-key-allowed flag を checkpoint/reset と一緒に管理する
- **THEN** top-level litmus grammar は flow/plain scalar の分岐を helper parser 経由で declarative に記述できる

### Requirement: acceptance litmus は YAML-ready 条件を実証しなければならない
acceptance litmus tests は、parser module の既存公開契約と downstream 側の合成だけで YAML に必要な主要分岐が表現可能であることを示さなければならない (MUST)。少なくとも block sequence / block mapping の indentation 判定、flow vs block context 切替、simple key 可否、line-head 判定を含む。

#### Scenario: line-head sensitive branch is expressible
- **WHEN** litmus grammar が line head でのみ許可される token を扱う
- **THEN** acceptance test は downstream helper が current position 情報から line-head 判定を行い、top-level grammar が declarative にその helper を合成できることを示す

#### Scenario: flow plain scalar restriction is expressible
- **WHEN** litmus grammar が flow context では `,]}` などで plain scalar を停止し、block context では改行で停止する必要がある
- **THEN** acceptance test は downstream helper / wrapper によってその差分を表現できることを示す

#### Scenario: simple-key allowance is expressible
- **WHEN** litmus grammar が simple key が許可される場所と禁止される場所を区別する
- **THEN** acceptance test は downstream-owned state を checkpoint/reset と整合的に扱うことで、その制約を表現できることを示す
