## ADDED Requirements

### Requirement: `recursive()` は combinator-chain の public API を維持する

`recursive(f)` SHALL continue to let downstream grammars define recursive parsers as public combinator chains without manual `parse_next` orchestration. The closure argument SHALL remain cloneable so grammars such as JSON, YAML, and arithmetic can reuse the recursive reference in multiple branches.

#### Scenario: recursive reference is reusable across grammar branches
- **WHEN** a grammar defines `recursive(|value| array(value.clone()).or(object(value)))`
- **THEN** the grammar compiles and executes without requiring imperative parser control flow outside combinator chaining

#### Scenario: existing recursive grammars keep their current call shape
- **WHEN** a downstream parser uses `recursive(|expr| { ... expr.clone() ... })`
- **THEN** the parser does not need a public API change to keep working

### Requirement: `recursive()` steady-state runtime SHALL not depend on `Box<dyn Parser>`

After initialization, `Recursive::parse_next` SHALL dispatch to the concrete recursive parser through typed runtime storage rather than through `Box<dyn Parser>`. The steady-state parse path MUST NOT require trait object dispatch to reach the recursive parser body.

#### Scenario: steady-state parse does not rely on trait object storage
- **WHEN** `recursive()` has finished building its parser graph and starts parsing input
- **THEN** the runtime reaches the recursive parser body without reading a `Box<dyn Parser>` from the recursive slot

### Requirement: `recursive()` steady-state runtime SHALL not depend on optional initialization checks

After `recursive(f)` returns a parser, steady-state `Recursive::parse_next` SHALL execute without optional parser-slot checks such as `Option<Box<_>>` unwraps on every recursive call.

#### Scenario: initialized recursive parser parses without per-call option unwrap
- **WHEN** an initialized recursive parser is called recursively many times during a parse
- **THEN** each recursive step executes without checking an `Option`-wrapped parser slot

### Requirement: internal recursive self references SHALL not create strong ownership cycles

`recursive()` SHALL separate the root owner of the recursive runtime allocation from parser-graph self references so that recursive references embedded inside the parser graph do not keep the runtime alive through a strong reference cycle.

#### Scenario: parser-graph self references are non-owning
- **WHEN** `recursive(f)` builds a parser graph that stores cloned recursive references inside combinators
- **THEN** those internal recursive references do not by themselves own the runtime allocation

#### Scenario: root parser keeps recursive runtime alive during parsing
- **WHEN** the root recursive parser value is retained and used to parse input
- **THEN** the recursive runtime allocation remains alive for the duration of the parse

### Requirement: `recursive()` SHALL preserve existing parse semantics

The runtime redesign of `recursive()` MUST preserve the current success/failure semantics of recursive grammars, including Backtrack/Cut propagation, nested recursion, and clone-based reuse inside the same grammar definition.

#### Scenario: recursive failure propagation is unchanged
- **WHEN** a recursive grammar previously returned `Fail::Backtrack` or `Fail::Cut` for the same input
- **THEN** the redesigned runtime returns the same `Fail` variant and preserves the same input rewind behavior

#### Scenario: nested recursive structures still parse successfully
- **WHEN** a grammar parses nested recursive inputs such as parenthesized expressions or nested JSON/YAML collections
- **THEN** the redesigned runtime accepts the same valid inputs as before
