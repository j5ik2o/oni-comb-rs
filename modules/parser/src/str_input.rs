use crate::input::Input;

#[derive(Debug, Clone, Copy)]
pub struct StrCheckpoint {
  pub offset: usize,
  pub line: usize,
  /// Column in char (codepoint) units. 1-origin.
  pub column: usize,
  /// Byte offset of the start of the current line. Used for extracting
  /// the full line text around an error position (`&src[line_start..offset]`).
  /// Intentionally in byte units (unlike `column` which is char units).
  pub line_start: usize,
}

impl PartialEq for StrCheckpoint {
  fn eq(&self, other: &Self) -> bool {
    self.offset == other.offset
  }
}

impl Eq for StrCheckpoint {}

impl PartialOrd for StrCheckpoint {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for StrCheckpoint {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.offset.cmp(&other.offset)
  }
}

pub struct StrInput<'a> {
  src: &'a str,
  offset: usize,
  line: usize,
  column: usize,
  /// Byte offset of the start of the current line. Stored in Checkpoint
  /// for O(1) reset. Reserved for future error reporting (e.g. extracting
  /// the full line text around an error position).
  line_start: usize,
}

impl<'a> StrInput<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      src,
      offset: 0,
      line: 1,
      column: 1,
      line_start: 0,
    }
  }

  pub fn advance(&mut self, n: usize) {
    let end = self.offset + n;
    let slice = &self.src[self.offset..end];
    for c in slice.chars() {
      if c == '\n' {
        self.line += 1;
        self.column = 1;
      } else {
        self.column += 1;
      }
    }
    self.offset = end;
    // line_start: find last newline in advanced range
    if let Some(pos) = slice.rfind('\n') {
      self.line_start = (end - n) + pos + 1;
    }
  }

  pub(crate) fn as_str(&self) -> &'a str {
    &self.src[self.offset..]
  }

  /// 次のバイトを消費せずに覗く。EOF なら `None`。
  #[inline]
  pub fn peek_byte(&self) -> Option<u8> {
    self.src.as_bytes().get(self.offset).copied()
  }
}

impl<'a> Input for StrInput<'a> {
  type Checkpoint = StrCheckpoint;
  #[cfg(feature = "alloc")]
  type Error = crate::error::ParseError;
  #[cfg(not(feature = "alloc"))]
  type Error = crate::error::MinimalError;
  type Slice = &'a str;
  type Token = char;

  #[inline]
  fn next_token(&mut self) -> Option<char> {
    let b = *self.src.as_bytes().get(self.offset)?;
    let c = if b.is_ascii() {
      self.offset += 1;
      b as char
    } else {
      let c = self.as_str().chars().next()?;
      self.offset += c.len_utf8();
      c
    };

    if c == '\n' {
      self.line += 1;
      self.column = 1;
      self.line_start = self.offset;
    } else {
      self.column += 1;
    }

    Some(c)
  }

  #[inline]
  fn peek_token(&self) -> Option<char> {
    let b = *self.src.as_bytes().get(self.offset)?;
    if b.is_ascii() {
      return Some(b as char);
    }

    self.as_str().chars().next()
  }

  #[inline]
  fn slice_since(&self, cp: StrCheckpoint) -> &'a str {
    &self.src[cp.offset..self.offset]
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    StrCheckpoint {
      offset: self.offset,
      line: self.line,
      column: self.column,
      line_start: self.line_start,
    }
  }

  fn reset(&mut self, checkpoint: Self::Checkpoint) {
    self.offset = checkpoint.offset;
    self.line = checkpoint.line;
    self.column = checkpoint.column;
    self.line_start = checkpoint.line_start;
  }

  fn offset(&self) -> usize {
    self.offset
  }

  fn remaining(&self) -> &'a str {
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
}
