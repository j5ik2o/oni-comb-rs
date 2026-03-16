## ADDED Requirements

### Requirement: Detect URN
`Uri::is_urn()` は scheme が `"urn"`（大文字小文字不問）のとき `true` を返さなければならない（SHALL）。

#### Scenario: URN scheme
- **WHEN** `Uri::parse("urn:isbn:0451450523")` の結果に `is_urn()` を呼ぶ
- **THEN** `true` を返す

#### Scenario: Non-URN scheme
- **WHEN** `Uri::parse("http://example.com")` の結果に `is_urn()` を呼ぶ
- **THEN** `false` を返す

#### Scenario: Case-insensitive
- **WHEN** `Uri::parse("URN:example:resource")` の結果に `is_urn()` を呼ぶ
- **THEN** `true` を返す

### Requirement: Extract URN NID
`Uri::urn_nid()` は URN の Namespace Identifier を返さなければならない（SHALL）。path-rootless の最初の `:` までが NID。

#### Scenario: ISBN NID
- **WHEN** `Uri::parse("urn:isbn:0451450523")` の結果に `urn_nid()` を呼ぶ
- **THEN** `Some("isbn")` を返す

#### Scenario: Non-URN returns None
- **WHEN** `Uri::parse("http://example.com")` の結果に `urn_nid()` を呼ぶ
- **THEN** `None` を返す

### Requirement: Extract URN NSS
`Uri::urn_nss()` は URN の Namespace Specific String を返さなければならない（SHALL）。NID の `:` 以降が NSS。

#### Scenario: ISBN NSS
- **WHEN** `Uri::parse("urn:isbn:0451450523")` の結果に `urn_nss()` を呼ぶ
- **THEN** `Some("0451450523")` を返す

#### Scenario: Complex NSS
- **WHEN** `Uri::parse("urn:example:a123,z456")` の結果に `urn_nss()` を呼ぶ
- **THEN** `Some("a123,z456")` を返す
