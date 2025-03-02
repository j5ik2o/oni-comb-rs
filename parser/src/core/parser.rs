use crate::core::{ParseResult, ParseState};
use std::rc::Rc;

type Parse<'a, I, A> = dyn Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I: Clone, A> {
  pub(crate) method: Rc<Parse<'a, I, A>>,
}

impl<'a, I: Clone, A> Clone for Parser<'a, I, A> {
  fn clone(&self) -> Self {
    Self {
      method: self.method.clone(),
    }
  }
}

impl<'a, I: Clone, A> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    Parser { method: Rc::new(parse) }
  }
}
