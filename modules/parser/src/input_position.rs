#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputPosition {
  /// Byte offset from the start of the input.
  pub offset: usize,
  /// Human-facing line number (1-origin).
  pub line: usize,
  /// Human-facing column number (1-origin).
  pub column: usize,
  /// Byte offset of the start of the current line.
  pub line_start: usize,
}

impl InputPosition {
  pub const fn new(offset: usize, line: usize, column: usize, line_start: usize) -> Self {
    Self {
      offset,
      line,
      column,
      line_start,
    }
  }

  pub const fn offset_only(offset: usize) -> Self {
    Self {
      offset,
      line: 0,
      column: 0,
      line_start: offset,
    }
  }
}
