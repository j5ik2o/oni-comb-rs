## ADDED Requirements

### Requirement: predictive choice は先頭 byte に基づいて branch を選択する

The parser library SHALL provide a predictive choice combinator that inspects the next byte without consuming input and selects a branch parser before executing it. The first cut SHALL support `StrInputStream` and `ByteInputStream`.

#### Scenario: branch is selected from a leading byte
- **WHEN** a predictive choice parser is configured with branches for `b'n'`, `b't'`, and `b'f'`
- **THEN** input starting with `n`, `t`, or `f` selects the corresponding branch without first executing the other branch parsers

### Requirement: predictive choice SHALL not consume input while selecting a branch

Branch selection MUST be performed from a non-consuming peek of the next byte. The selected parser SHALL observe the same input position as if it had been called directly.

#### Scenario: selected branch sees original input position
- **WHEN** predictive choice selects a branch from the next byte
- **THEN** the branch parser starts from the original input position and consumes input only through its own parse logic

### Requirement: predictive choice SHALL return unmatched Backtrack when no branch matches

If no configured branch condition matches the next byte (or EOF), predictive choice MUST return `Fail::Backtrack` without executing any branch parser.

#### Scenario: unmatched leading byte returns Backtrack
- **WHEN** predictive choice is configured for `b'n'` and `b't'`, and the input starts with `x`
- **THEN** the parser returns `Err(Fail::Backtrack(_))`
- **AND** no branch parser is executed

### Requirement: predictive choice SHALL not fall back after selecting a branch

Once predictive choice has selected a branch, it MUST execute only that branch parser. If the selected parser returns `Backtrack`, `Cut`, or another failure variant, predictive choice SHALL propagate that result unchanged rather than trying another branch.

#### Scenario: selected branch Backtrack is propagated
- **WHEN** predictive choice selects the `b'n'` branch and that branch parser returns `Err(Fail::Backtrack(_))`
- **THEN** predictive choice returns the same `Err(Fail::Backtrack(_))`
- **AND** no other branch parser is tried

#### Scenario: selected branch Cut is propagated
- **WHEN** predictive choice selects a branch whose parser returns `Err(Fail::Cut(_))`
- **THEN** predictive choice returns `Err(Fail::Cut(_))`

### Requirement: predictive choice SHALL support lightweight byte predicates

The first cut SHALL support not only exact-byte branches but also lightweight byte predicates so grammars can express cases such as numeric dispatch (`'-'` or ASCII digit`)` declaratively.

#### Scenario: numeric branch can be selected by predicate
- **WHEN** a predictive choice parser has a predicate branch matching `b'-'` or any ASCII digit
- **THEN** inputs such as `-7` and `42` select the numeric branch without requiring a trial parse of unrelated branches

### Requirement: predictive choice SHALL preserve declarative grammar style

The predictive choice API MUST allow downstream parsers to remain written as public combinator chains and MUST NOT require direct `parse_next`, `checkpoint/reset`, or manual `peek_byte` dispatch in grammar definitions.

#### Scenario: JSON-like value grammar remains combinator-based
- **WHEN** a downstream grammar rewrites a JSON value choice using predictive choice
- **THEN** the grammar is still expressed as a public combinator composition rather than imperative input dispatch
