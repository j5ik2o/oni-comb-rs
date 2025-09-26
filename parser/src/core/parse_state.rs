/// A struct representing the current parsing state.
#[derive(Clone, Copy)]
pub struct ParseState<'a, I> {
  original: &'a [I],
  rest: &'a [I],
  offset: usize,
}

impl<'a, I> ParseState<'a, I> {
  /// Creates a new parsing state with the given input and offset.
  pub fn new(input: &'a [I], offset: usize) -> Self {
    assert!(offset <= input.len());
    let rest = &input[offset..];
    Self {
      original: input,
      rest,
      offset,
    }
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
  pub fn advance_by(&self, num_chars: usize) -> ParseState<'a, I> {
    assert!(num_chars <= self.rest.len());
    let new_offset = self.offset + num_chars;
    let new_rest = &self.rest[num_chars..];
    Self {
      original: self.original,
      rest: new_rest,
      offset: new_offset,
    }
  }

  /// Returns the slice of input starting from the current offset.
  pub fn input(&self) -> &'a [I] {
    self.rest
  }

  /// Returns a slice of the input with a specified length starting from the current offset.
  pub fn slice_with_len(&self, n: usize) -> &'a [I] {
    &self.rest[..n]
  }

  /// Returns the full original input slice.
  pub fn original(&self) -> &'a [I] {
    self.original
  }
}
