## ADDED Requirements

### Requirement: YAML parser change stays downstream-owned until proposed separately
YAML-specific parser implementation, AST, and crate structure SHALL remain outside the parser core scope until a dedicated downstream YAML change is proposed and accepted.

#### Scenario: YAML-ready parser work finishes without adding a yaml crate
- **WHEN** the YAML-ready parser change is completed
- **THEN** the workspace still has no committed `modules/yaml` crate
- **AND** readiness is demonstrated by acceptance grammars and generic parser capabilities instead

#### Scenario: downstream YAML parser is proposed later as a separate change
- **WHEN** full YAML syntax support is started
- **THEN** it is tracked under a new change proposal with its own requirements, design, and tasks

### Requirement: YAML-specific behavior must compose from parser public contracts first
Any future downstream YAML parser SHALL first attempt to express indentation-sensitive, flow/block, and simple-key behavior by composing existing parser public contracts and downstream-owned helpers before requesting new generic primitives from the parser module.

#### Scenario: future YAML parser reuses generic input position and checkpointing
- **WHEN** a downstream YAML parser needs indentation-aware decisions
- **THEN** it can compute them from input position, checkpoint/reset, and downstream-owned parser state without requiring YAML-specific APIs in the parser core

#### Scenario: new parser primitive requires demonstrated insufficiency
- **WHEN** a downstream YAML grammar remains inexpressible after composing the existing public contract
- **THEN** a separate parser-core proposal can justify the minimum YAML-independent primitive needed
