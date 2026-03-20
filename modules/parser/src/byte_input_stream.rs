use crate::input_position::InputPosition;
use crate::input_stream::InputStream;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ParserState {
  indent_depth: u8,
  context_depth: u8,
  flags: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ByteCheckpoint {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
  pub line_start: usize,
  state: ParserState,
}

impl PartialEq for ByteCheckpoint {
  fn eq(&self, other: &Self) -> bool {
    self.offset == other.offset
  }
}

impl Eq for ByteCheckpoint {}

impl PartialOrd for ByteCheckpoint {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ByteCheckpoint {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.offset.cmp(&other.offset)
  }
}

pub struct ByteInputStream<'a> {
  src: &'a [u8],
  offset: usize,
  line: usize,
  column: usize,
  line_start: usize,
  /// Parser-core state that participates in checkpoint/reset.
  /// Downstream semantic data is intentionally excluded from this snapshot.
  state: ParserState,
}

impl<'a> ByteInputStream<'a> {
  pub fn new(src: &'a [u8]) -> Self {
    Self {
      src,
      offset: 0,
      line: 1,
      column: 1,
      line_start: 0,
      state: ParserState::default(),
    }
  }

  #[inline]
  pub fn peek_byte(&self) -> Option<u8> {
    self.src.get(self.offset).copied()
  }

  #[inline]
  pub fn advance(&mut self, n: usize) {
    let end = self.offset + n;
    let slice = &self.src[self.offset..end];
    for &b in slice {
      if b == b'\n' {
        self.line += 1;
        self.column = 1;
      } else {
        self.column += 1;
      }
    }
    // Find last newline in the advanced range for line_start
    if let Some(pos) = slice.iter().rposition(|&b| b == b'\n') {
      self.line_start = self.offset + pos + 1;
    }
    self.offset = end;
  }
}

impl<'a> InputStream for ByteInputStream<'a> {
  type Checkpoint = ByteCheckpoint;
  #[cfg(feature = "alloc")]
  type Error = crate::error::ParseError;
  #[cfg(not(feature = "alloc"))]
  type Error = crate::error::MinimalError;
  type Slice = &'a [u8];
  type Token = u8;

  #[inline]
  fn next_token(&mut self) -> Option<u8> {
    let b = self.src.get(self.offset).copied()?;
    self.offset += 1;

    if b == b'\n' {
      self.line += 1;
      self.column = 1;
      self.line_start = self.offset;
    } else {
      self.column += 1;
    }

    Some(b)
  }

  #[inline]
  fn peek_token(&self) -> Option<u8> {
    self.src.get(self.offset).copied()
  }

  #[inline]
  fn slice_since(&self, cp: ByteCheckpoint) -> &'a [u8] {
    &self.src[cp.offset..self.offset]
  }

  fn checkpoint(&self) -> ByteCheckpoint {
    ByteCheckpoint {
      offset: self.offset,
      line: self.line,
      column: self.column,
      line_start: self.line_start,
      state: self.state,
    }
  }

  fn reset(&mut self, cp: ByteCheckpoint) {
    self.offset = cp.offset;
    self.line = cp.line;
    self.column = cp.column;
    self.line_start = cp.line_start;
    self.state = cp.state;
  }

  fn offset(&self) -> usize {
    self.offset
  }

  fn remaining(&self) -> &'a [u8] {
    &self.src[self.offset..]
  }

  fn is_eof(&self) -> bool {
    self.offset >= self.src.len()
  }

  fn line(&self) -> usize {
    self.line
  }

  fn column(&self) -> usize {
    self.column
  }

  fn line_start(&self) -> usize {
    self.line_start
  }

  fn position_after(&self, consumed: usize) -> InputPosition {
    let start = self.offset;
    let end = start + consumed;
    let slice = &self.src[start..end];
    let mut position = self.position();
    for (relative_offset, b) in slice.iter().enumerate() {
      if *b == b'\n' {
        position.line += 1;
        position.column = 1;
        position.line_start = start + relative_offset + 1;
      } else {
        position.column += 1;
      }
    }
    position.offset = end;
    position
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn checkpoint_restores_internal_parser_state() {
    let mut input = ByteInputStream::new(b"abc");
    let cp = input.checkpoint();
    input.state.indent_depth = 1;
    input.state.context_depth = 4;
    input.state.flags = 0b11;

    input.reset(cp);

    assert_eq!(input.state, ParserState::default());
  }
}
