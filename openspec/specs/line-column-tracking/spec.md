## MODIFIED Requirements

### Requirement: InputStream tracks line and column positions
The `InputStream` trait SHALL expose the current byte offset (`position`), human-facing line number (`line`), human-facing column number (`column`), and the byte offset of the current line start (`line_start`) as the stream advances.

#### Scenario: Initial position at start of input
- **WHEN** a new stream is created for any non-empty input
- **THEN** `position()` is `0`
- **AND** `line()` is `1`
- **AND** `column()` is `1`
- **AND** `line_start()` is `0`

#### Scenario: ASCII advance updates column
- **WHEN** the stream consumes one ASCII character that is not a newline
- **THEN** `position()` increases by `1`
- **AND** `line()` is unchanged
- **AND** `column()` increases by `1`
- **AND** `line_start()` is unchanged

#### Scenario: Newline resets column and increments line
- **WHEN** the stream consumes a newline character
- **THEN** `position()` advances past the newline bytes
- **AND** `line()` increments by `1`
- **AND** `column()` becomes `1`
- **AND** `line_start()` becomes the byte offset immediately after the newline

#### Scenario: UTF-8 multibyte characters advance by bytes but count as one column
- **WHEN** the stream consumes a multibyte UTF-8 character such as `あ`
- **THEN** `position()` advances by the UTF-8 byte length of that character
- **AND** `line()` is unchanged
- **AND** `column()` increases by `1`
- **AND** `line_start()` is unchanged

#### Scenario: position snapshot carries line start
- **WHEN** parser code obtains the current input position snapshot
- **THEN** the snapshot includes `offset`, `line`, `column`, and `line_start`
- **AND** downstream helpers can compute visual indentation using `offset - line_start`
