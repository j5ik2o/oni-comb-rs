use crate::core::{ParseResult, ParseState};
use std::marker::PhantomData;
use std::rc::Rc;

type Parse<'a, I, A> = dyn Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I, A> {
  pub(crate) method: Rc<Parse<'a, I, A>>,
  _phantom: PhantomData<&'a I>,
}

impl<'a, I, A> Clone for Parser<'a, I, A> {
  fn clone(&self) -> Self {
    Self {
      method: self.method.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    Parser {
      method: Rc::new(parse),
      _phantom: PhantomData,
    }
  }
}
