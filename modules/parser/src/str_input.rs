use crate::input::Input;

pub struct StrInput<'a> {
  src: &'a str,
  offset: usize,
}

impl<'a> StrInput<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, offset: 0 }
  }

  pub(crate) fn advance(&mut self, n: usize) {
    self.offset += n;
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
  type Token = char;
  type Slice = &'a str;
  type Checkpoint = usize;

  #[inline]
  fn next_token(&mut self) -> Option<char> {
    let c = self.as_str().chars().next()?;
    self.offset += c.len_utf8();
    Some(c)
  }

  #[inline]
  fn peek_token(&self) -> Option<char> {
    self.as_str().chars().next()
  }

  #[inline]
  fn slice_since(&self, cp: usize) -> &'a str {
    &self.src[cp..self.offset]
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    self.offset
  }

  fn reset(&mut self, checkpoint: Self::Checkpoint) {
    self.offset = checkpoint;
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
}
