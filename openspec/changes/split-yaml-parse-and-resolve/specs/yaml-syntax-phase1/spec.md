## ADDED Requirements

### Requirement: YAML Phase 1 は syntax-only の parsing API を提供する
`modules/yaml` は、意味解決前の YAML 構文木を返す `parse_syntax(src)` と `parse_syntax_documents(src)` を提供しなければならない。これらの API は `Result<..., ParseError>` を返し、plain scalar の schema 解釈、alias 解決、merge key 適用、tag による型強制を行ってはならない。

#### Scenario: 単一ドキュメントを syntax tree として返す
- **WHEN** `"hello"` を `parse_syntax(src)` に渡す
- **THEN** plain scalar を表す syntax node を返す

#### Scenario: 複数ドキュメントを syntax tree の列として返す
- **WHEN** `"---\n[1, 2]\n---\n{name: oni-comb}"` を `parse_syntax_documents(src)` に渡す
- **THEN** 2 つの syntax document を返す

### Requirement: YAML Phase 1 は最小の syntax AST 形を固定する
Phase 1 の syntax AST は、少なくとも `YamlSyntaxDocument { root: YamlSyntaxNode }` を持ち、`YamlSyntaxNode` は `Scalar`、`Sequence`、`Mapping` を含まなければならない。`Scalar` は `Plain`、`SingleQuoted`、`DoubleQuoted` を区別しなければならない。`Sequence` と `Mapping` は flow style であることを保持しなければならない。

#### Scenario: plain scalar の node 形が固定される
- **WHEN** `"hello"` を `parse_syntax(src)` に渡す
- **THEN** `YamlSyntaxDocument` の `root` は `YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain(...))` に相当する構造を返す

#### Scenario: flow sequence の node 形が固定される
- **WHEN** `"[1, 2]"` を `parse_syntax(src)` に渡す
- **THEN** `YamlSyntaxDocument` の `root` は flow style を持つ `YamlSyntaxNode::Sequence` に相当する構造を返す

#### Scenario: flow mapping の node 形が固定される
- **WHEN** `"{name: oni-comb}"` を `parse_syntax(src)` に渡す
- **THEN** `YamlSyntaxDocument` の `root` は flow style を持つ `YamlSyntaxNode::Mapping` に相当する構造を返す

### Requirement: YAML Phase 1 の syntax AST は将来拡張に耐えなければならない
Phase 1 の syntax AST は、後続フェーズで `Tagged`、`Anchored`、`Alias` を追加しても既存の `Scalar`、`Sequence`、`Mapping` の意味を壊さない形で設計されなければならない。

#### Scenario: Phase 1 AST は unresolved node の追加余地を残す
- **WHEN** Phase 1 の syntax AST を設計する
- **THEN** 後続フェーズで `Tagged`、`Anchored`、`Alias` を additive に導入できる構造になっている

### Requirement: YAML Phase 1 は scalar を syntax-only で保持する
Phase 1 の syntax parser は plain scalar、single-quoted scalar、double-quoted scalar を構文上区別して保持しなければならない。plain scalar は parse 時点で `null`、`bool`、`int`、`float` に確定してはならない。

#### Scenario: plain scalar は未解決のまま保持される
- **WHEN** `"42"` を `parse_syntax(src)` に渡す
- **THEN** 数値に確定した値ではなく plain scalar の syntax node を返す

#### Scenario: single-quoted scalar を区別して保持する
- **WHEN** `"'hello'"` を `parse_syntax(src)` に渡す
- **THEN** single-quoted scalar の syntax node を返す

#### Scenario: double-quoted scalar を区別して保持する
- **WHEN** `"\"hello\""` を `parse_syntax(src)` に渡す
- **THEN** double-quoted scalar の syntax node を返す

### Requirement: YAML Phase 1 は flow sequence と flow mapping を syntax-only でパースする
Phase 1 の syntax parser は flow sequence (`[a, b]`) と flow mapping (`{a: b}`) をパースし、子要素も syntax node として保持しなければならない。

#### Scenario: flow sequence をパースする
- **WHEN** `"[1, 2, 3]"` を `parse_syntax(src)` に渡す
- **THEN** 3 要素の flow sequence syntax node を返す

#### Scenario: flow mapping をパースする
- **WHEN** `"{name: oni-comb, version: 2}"` を `parse_syntax(src)` に渡す
- **THEN** 2 エントリの flow mapping syntax node を返す

#### Scenario: flow syntax はネストできる
- **WHEN** `"{a: [1, 2], b: {c: true}}"` を `parse_syntax(src)` に渡す
- **THEN** mapping の中に nested flow sequence と flow mapping を含む syntax tree を返す

### Requirement: YAML Phase 1 は基本 comment と document marker をサポートする
Phase 1 の syntax parser は行コメントを無視し、`---` と `...` の基本的な document marker を扱わなければならない。

#### Scenario: 行コメントを無視する
- **WHEN** `"{key: value} # comment"` を `parse_syntax(src)` に渡す
- **THEN** comment を除いた syntax tree を返す

#### Scenario: 文書開始マーカーを扱う
- **WHEN** `"---\n[1, 2, 3]"` を `parse_syntax(src)` に渡す
- **THEN** 文書開始を許容しつつ本文の syntax tree を返す

#### Scenario: 文書終了マーカーを扱う
- **WHEN** `"---\n{name: oni-comb}\n..."` を `parse_syntax_documents(src)` に渡す
- **THEN** 文書終了マーカーを許容して 1 つの syntax document を返す

#### Scenario: コメントだけの入力を扱う
- **WHEN** `"# comment only"` を `parse_syntax_documents(src)` に渡す
- **THEN** コメントを無視し、0 個の syntax document もしくは空文書を一貫した規約で返す

#### Scenario: 空白だけの入力を扱う
- **WHEN** `"  \n\t"` を `parse_syntax_documents(src)` に渡す
- **THEN** 空白を無視し、0 個の syntax document もしくは空文書を一貫した規約で返す

### Requirement: YAML Phase 1 は未対応機能を明示的に除外する
Phase 1 の syntax parser は block mapping、block sequence、block scalar、anchor / alias 構文、tag 構文、merge key を実装対象に含めてはならない。これらに遭遇した場合、`parse_syntax` 系 API は `ParseError` を返して未対応であることを明示しなければならない。

#### Scenario: block mapping は Phase 1 の対象外である
- **WHEN** `"parent:\n  child: value"` を `parse_syntax(src)` に渡す
- **THEN** block mapping が未対応であることを示す `ParseError` を返す

#### Scenario: alias は Phase 1 の対象外である
- **WHEN** `"- *anchor"` を `parse_syntax(src)` に渡す
- **THEN** alias 構文が未対応であることを示す `ParseError` を返す

#### Scenario: merge key は Phase 1 の対象外である
- **WHEN** `"{<<: *defs}"` を `parse_syntax(src)` に渡す
- **THEN** merge key が未対応であることを示す `ParseError` を返す
