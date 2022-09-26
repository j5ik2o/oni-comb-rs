/// 現在の解析状態を示す構造体。
#[derive(Clone)]
pub struct ParseState<'a, I> {
  input: &'a [I],
  offset: usize,
}

impl<'a, I> ParseState<'a, I> {
  pub fn new(input: &'a [I], offset: usize) -> Self {
    Self { input, offset }
  }

  pub fn last_offset(&self) -> Option<usize> {
    if self.offset > 0 {
      Some(self.offset - 1)
    } else {
      None
    }
  }

  pub fn next_offset(&self) -> usize {
    self.offset
  }

  pub fn add_offset(&self, num_chars: usize) -> ParseState<'a, I> {
    Self::new(self.input, self.offset + num_chars)
  }

  pub fn input(&self) -> &'a [I] {
    &self.input[self.offset..]
  }

  pub fn slice_with_len(&self, n: usize) -> &'a [I] {
    &self.input[self.offset..self.offset + n]
  }
}
