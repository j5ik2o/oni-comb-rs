use crate::parse_error::ParseError;

#[derive(Debug, Clone)]
pub struct Location<'a, I> {
  pub(crate) input: &'a [I],
  pub(crate) offset: usize,
}

impl<'a, I> Location<'a, I> {
  pub fn new(input: &'a [I]) -> Self {
    Self { input, offset: 0 }
  }

  pub fn add_offset(&mut self, n: usize) {
    self.offset += n;
  }

  pub fn with_add_offset(self, n: usize) -> Self {
    Self {
      offset: self.offset + n,
      ..self
    }
  }

  pub fn to_error(self, msg: String) -> ParseError<'a, I> {
    ParseError::new(vec![(self, msg)])
  }
}
