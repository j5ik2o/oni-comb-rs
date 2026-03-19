## ADDED Requirements

### Requirement: YAML parser capability は resolved API と syntax API を分離して提供する
`modules/yaml` は、最終的な `YamlValue` を返す resolved API と、意味解決前の syntax tree を返す syntax API を分離して提供しなければならない。`parse(src)` と `parse_documents(src)` は resolved API の責務を維持し、`parse_syntax(src)` と `parse_syntax_documents(src)` は additive な低レベル API として導入されなければならない。

#### Scenario: syntax API の導入は resolved API の責務を弱めない
- **WHEN** `parse_syntax(src)` が追加される
- **THEN** `parse(src)` は syntax-only の結果へ置き換わらず、最終的な `YamlValue` を返す高レベル API のままである

#### Scenario: syntax API と resolved API は異なる責務を持つ
- **WHEN** `"42"` を `parse_syntax(src)` に渡す
- **THEN** plain scalar の syntax node を返す
- **WHEN** `"42"` を `parse(src)` に渡す
- **THEN** `YamlValue::Integer(42)` を返す
