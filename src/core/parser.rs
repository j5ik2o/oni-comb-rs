use crate::core::{ParseResult, ParseState};

type Parse<'a, I, A> = dyn Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I, A> {
  pub(crate) method: Box<Parse<'a, I, A>>,
}

impl<'a, I, A> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    Parser {
      method: Box::new(parse),
    }
  }
}
