use crate::core::{ParseResult, ParseState};
use std::rc::Rc;

type Parse<'a, I, A> = dyn Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I, A> {
  // Rcを使用して、パーサーのクローンを効率的に行う
  // Boxに変更すると、クローン時に新しいBoxを作成する必要があり、
  // パーサーの組み合わせが多用されるため、パフォーマンスが低下する可能性がある
  pub(crate) method: Rc<Parse<'a, I, A>>,
}

impl<'a, I, A> Clone for Parser<'a, I, A> {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      method: self.method.clone(),
    }
  }
}

impl<'a, I, A> Parser<'a, I, A> {
  #[inline]
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    Parser { method: Rc::new(parse) }
  }
}
