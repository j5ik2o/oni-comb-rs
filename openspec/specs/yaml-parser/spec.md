## Requirements

### Requirement: YAML parser change stays downstream-owned until proposed separately
The YAML parser capability SHALL be implemented as a dedicated downstream change and downstream crate, rather than by extending the parser core with YAML-specific APIs. The accepted MVP for this change MUST live in `modules/yaml` and provide a single-document YAML subset parser.

#### Scenario: yaml parser exists as its own downstream crate
- **WHEN** the `yaml-parser-mvp` change is implemented
- **THEN** the workspace contains a `modules/yaml` crate
- **AND** YAML-specific parsing logic and AST types live there instead of `modules/parser`

#### Scenario: parser core remains YAML-agnostic
- **WHEN** the MVP YAML parser is added
- **THEN** `modules/parser` does not gain YAML-specific layout primitives or YAML-specific public state types

### Requirement: YAML-specific behavior must compose from parser public contracts first
The downstream YAML parser SHALL express indentation-sensitive, flow/block, and simple-key behavior by composing existing parser public contracts with declarative combinator chains only. It MUST NOT rely on custom `Parser` implementations, `InputStream` wrappers, `parse_next` / `checkpoint` / `reset` / token-stepping direct calls, discarded parser results, `fn_parser`, or ad hoc parser-core API additions as its primary implementation strategy.

#### Scenario: internal helper stays declarative
- **WHEN** the YAML parser's top-level grammar is reviewed
- **THEN** block and flow productions are expressed as combinator pipelines
- **AND** helper abstractions, if any, are functions returning composed parsers rather than imperative parser objects

#### Scenario: declarative expression fails and implementation pauses
- **WHEN** a required YAML grammar slice cannot be expressed using public combinator chains alone
- **THEN** implementation stops for that slice
- **AND** the team records whether the blocker is MVP scope, parser-core generic capability, or spec ambiguity instead of introducing imperative fallback code

### Requirement: YAML parser MVP parses single-document block and flow collections
The MVP YAML parser SHALL parse one top-level YAML document containing block mappings, block sequences, flow mappings, and flow sequences, including nesting between block and flow forms. Mapping keys accepted by this MVP MUST be limited to the supported scalar subset. Explicit key syntax (`? key`) and collection-valued keys are out of scope for this change.

#### Scenario: block mapping with nested block sequence
- **WHEN** the parser reads:
  ```
  items:
    - milk
    - eggs
  ```
- **THEN** it returns a mapping whose `items` value is a two-element sequence

#### Scenario: flow sequence inside block mapping
- **WHEN** the parser reads:
  ```
  items: [one, two]
  nested:
    key: value
  ```
- **THEN** it returns a mapping that contains both a flow sequence value and a nested block mapping value

#### Scenario: flow mapping at top level
- **WHEN** the parser reads `{name: oni-comb, version: 2}`
- **THEN** it returns a mapping with two entries

### Requirement: YAML parser MVP parses the basic scalar subset and ignores comments
The MVP YAML parser SHALL support plain scalars, single-quoted scalars, double-quoted scalars, `null`, `true` / `false`, and decimal integers. It MUST ignore line comments introduced by `#` after a value boundary and outside quoted scalars in both block and flow forms.

#### Scenario: plain scalar
- **WHEN** the parser reads `title: hello world`
- **THEN** it returns a mapping whose value is the plain string `hello world`

#### Scenario: quoted scalars
- **WHEN** the parser reads `single: 'hello'\ndouble: "world"`
- **THEN** it returns string values for both quoted scalars

#### Scenario: null bool and integer
- **WHEN** the parser reads `a: null\nb: true\nc: 42`
- **THEN** it returns `Null`, `Bool(true)`, and `Integer(42)` values

#### Scenario: line comment is ignored
- **WHEN** the parser reads `key: value # comment`
- **THEN** it returns the same mapping result as `key: value`

#### Scenario: flow comment is ignored after a closed value
- **WHEN** the parser reads `items: [one, two] # comment`
- **THEN** it returns the same mapping result as `items: [one, two]`

### Requirement: YAML parser MVP exposes a minimal public AST and parse API
The MVP YAML parser SHALL expose a minimal public AST with variants for null, bool, integer, string, sequence, and mapping, plus parser entry points equivalent to `parse`, `parse_value`, `yaml`, and `yaml_value`.

#### Scenario: parse consumes the whole document
- **WHEN** the caller uses the full parse entry point on `key: value`
- **THEN** it returns a mapping value
- **AND** trailing non-comment text causes an error

#### Scenario: parse_value does not require EOF
- **WHEN** the caller uses the value-only parse entry point on `[one, two], trailing`
- **THEN** it returns the flow sequence value without requiring the trailing `, trailing` input to be consumed

### Requirement: YAML parser MVP returns location-aware parse errors
The MVP YAML parser SHALL return parse errors that preserve line, column, expected token, and parser context information supplied by the parser core position/error model.

#### Scenario: indentation mismatch reports location
- **WHEN** the parser reads:
  ```
  root:
    child: ok
   next: wrong
  ```
- **THEN** it returns an error near line 3 with indentation-related expectation information

#### Scenario: unterminated flow collection reports context
- **WHEN** the parser reads `items: [one, two`
- **THEN** it returns an error with location and expected closing-token context
