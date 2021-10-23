use crate::core::Parser;
use std::fmt::Debug;
use std::ops::Add;
use crate::extension::parser::BasicCombinator;

impl<'a, I, A, B> Add<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Parser<'a, I, (A, B)>;

  fn add(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.and_then(rhs)
  }
}
