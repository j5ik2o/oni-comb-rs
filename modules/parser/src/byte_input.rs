use crate::input::Input;

pub struct ByteInput<'a> {
  src: &'a [u8],
  offset: usize,
}

impl<'a> ByteInput<'a> {
  pub fn new(src: &'a [u8]) -> Self {
    Self { src, offset: 0 }
  }

  #[inline]
  pub fn peek_byte(&self) -> Option<u8> {
    self.src.get(self.offset).copied()
  }
}

impl<'a> Input for ByteInput<'a> {
  type Checkpoint = usize;
  type Slice = &'a [u8];
  type Token = u8;

  #[inline]
  fn next_token(&mut self) -> Option<u8> {
    let b = self.src.get(self.offset).copied()?;
    self.offset += 1;
    Some(b)
  }

  #[inline]
  fn peek_token(&self) -> Option<u8> {
    self.src.get(self.offset).copied()
  }

  #[inline]
  fn slice_since(&self, cp: usize) -> &'a [u8] {
    &self.src[cp..self.offset]
  }

  fn checkpoint(&self) -> usize {
    self.offset
  }

  fn reset(&mut self, cp: usize) {
    self.offset = cp;
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
}
