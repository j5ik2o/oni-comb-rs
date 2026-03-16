## ADDED Requirements

### Requirement: flat_map combinator for monadic parser composition
`ParserExt` SHALL provide a `.flat_map(f)` method that takes a closure `f: FnMut(Self::Output) -> P2` where `P2: Parser<I>`, and returns a `FlatMap<Self, F>` combinator. The combinator MUST first execute the original parser, then pass its output to `f` to obtain a second parser, and execute that second parser on the remaining input.

#### Scenario: flat_map succeeds when both parsers succeed
- **WHEN** the first parser succeeds with output `v` and `f(v)` returns a parser that also succeeds
- **THEN** `flat_map` returns `Ok` with the second parser's output, and input is advanced past both consumed portions

#### Scenario: flat_map fails when the first parser fails
- **WHEN** the first parser returns `Err(Fail::Backtrack(e))`
- **THEN** `flat_map` returns `Err(Fail::Backtrack(e))` without calling `f`

#### Scenario: flat_map propagates Cut from the first parser
- **WHEN** the first parser returns `Err(Fail::Cut(e))`
- **THEN** `flat_map` returns `Err(Fail::Cut(e))` without calling `f`

#### Scenario: flat_map propagates failure from the dynamically chosen parser
- **WHEN** the first parser succeeds but the parser returned by `f` fails with any `Fail` variant
- **THEN** `flat_map` returns that `Fail` variant unchanged

#### Scenario: flat_map works with same-type branches without Box
- **WHEN** `f` returns the same concrete parser type in all branches (e.g., all branches return `Tag`)
- **THEN** `flat_map` SHALL work without requiring `Box<dyn Parser>` or any heap allocation

#### Scenario: flat_map works with Box<dyn Parser> for heterogeneous branches
- **WHEN** `f` returns `Box<dyn Parser>` to unify different parser types across branches
- **THEN** `flat_map` SHALL execute the boxed parser correctly

### Requirement: FlatMap combinator type in combinator module
The `combinator` module SHALL contain a `FlatMap<P, F>` struct that implements `Parser<I>`. The struct fields MUST be private, consistent with other combinator types.

#### Scenario: FlatMap struct is defined with private fields
- **WHEN** a user inspects the `FlatMap` type
- **THEN** the struct has two private fields: the source parser and the closure

### Requirement: flat_map composes with existing combinators
`flat_map` MUST compose correctly with all existing combinators: `map`, `then`, `or`, `attempt`, `cut`, `optional`, `many0`.

#### Scenario: flat_map inside attempt downgrades Cut to Backtrack
- **WHEN** `flat_map` is wrapped in `.attempt()` and the dynamically chosen parser produces `Fail::Cut`
- **THEN** `attempt` downgrades it to `Fail::Backtrack` and rewinds input, as with any other parser

#### Scenario: flat_map result can be mapped
- **WHEN** `.flat_map(f).map(g)` is used
- **THEN** `g` transforms the output of the dynamically chosen parser

#### Scenario: flat_map can be used inside or
- **WHEN** `flat_map_parser.or(fallback)` is used and `flat_map_parser` backtracks
- **THEN** `or` rewinds and tries `fallback`
