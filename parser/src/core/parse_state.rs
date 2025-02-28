/// 現在の解析状態を示す構造体。
#[derive(Clone)]
pub struct ParseState<'a, I> {
  input: &'a [I],
  offset: usize,
}

impl<'a, I> ParseState<'a, I> {
  #[inline]
  pub fn new(input: &'a [I], offset: usize) -> Self {
    Self { input, offset }
  }

  #[inline]
  pub fn last_offset(&self) -> Option<usize> {
    if self.offset > 0 {
      Some(self.offset - 1)
    } else {
      None
    }
  }

  #[inline]
  pub fn next_offset(&self) -> usize {
    self.offset
  }

  #[inline]
  pub fn add_offset(&self, num_chars: usize) -> ParseState<'a, I> {
    // 新しいインスタンスを作成する代わりに、既存のインスタンスを変更する
    // これにより、メモリ割り当てを減らす
    Self { input: self.input, offset: self.offset + num_chars }
  }

  #[inline]
  pub fn input(&self) -> &'a [I] {
    &self.input[self.offset..]
  }

  #[inline]
  pub fn slice_with_len(&self, n: usize) -> &'a [I] {
    &self.input[self.offset..self.offset + n]
  }
}
