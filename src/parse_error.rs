use crate::location::Location;

#[derive(Debug, Clone)]
pub struct ParseError<'a, I>(Vec<(Location<'a, I>, String)>);

impl<'a, I> ParseError<'a, I> {
  pub fn new(value: Vec<(Location<'a, I>, String)>) -> Self {
    Self(value)
  }
}
