use crate::location::Location;

#[derive(Clone)]
pub struct ParseState<'a, I> {
  pub(crate) location: Location<'a, I>,
}

impl<'a, I: Clone> ParseState<'a, I> {
  pub fn new(location: Location<'a, I>) -> Self {
    Self { location }
  }

  pub fn offset(&self) -> usize {
    self.location.offset
  }

  pub fn advance_by(&self, num_chars: usize) -> ParseState<'a, I> {
    let mut l = self.location.clone();
    l.add_offset(num_chars);
    Self { location: l }
  }

  pub fn input(&self) -> &'a [I] {
    &self.location.input[self.location.offset..]
  }

  pub fn slice(&self, n: usize) -> &'a [I] {
    &self.location.input[self.location.offset..self.location.offset + n]
  }
}
