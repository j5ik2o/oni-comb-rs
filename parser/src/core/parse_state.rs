/// A struct representing the current parsing state.
#[derive(Clone)]
pub struct ParseState<'a, I> {
  input: &'a [I],
  offset: usize,
}

impl<'a, I> ParseState<'a, I> {
  /// Creates a new parsing state with the given input and offset.
  pub fn new(input: &'a [I], offset: usize) -> Self {
    Self { input, offset }
  }

  /// Returns the offset of the previous position, or None if at the beginning.
  pub fn last_offset(&self) -> Option<usize> {
    if self.offset > 0 {
      Some(self.offset - 1)
    } else {
      None
    }
  }

  /// Returns the current offset.
  pub fn current_offset(&self) -> usize {
    self.offset
  }

  /// Creates a new parse state with an offset increased by the specified number of characters.
  pub fn add_offset(&self, num_chars: usize) -> ParseState<'a, I> {
    Self::new(self.input, self.offset + num_chars)
  }

  /// Returns the slice of input starting from the current offset.
  pub fn input(&self) -> &'a [I] {
    &self.input[self.offset..]
  }

  /// Returns a slice of the input with a specified length starting from the current offset.
  pub fn slice_with_len(&self, n: usize) -> &'a [I] {
    &self.input[self.offset..self.offset + n]
  }
}
