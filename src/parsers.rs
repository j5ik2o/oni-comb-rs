use crate::parse_error::ParseError;
use crate::parser::Parser;

pub trait Parsers {
  type P<'p, I, A>: Parser<'p, Input = I, Output = A>
  where
    I: 'p;

  fn run<'a, I, A>(parser: Self::P<'a, I, A>, input: &'a [I]) -> Result<A, ParseError<'a, I>>
  where
    I: Clone;

  fn unit<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + 'a, {
    Self::successful(())
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + 'a;

  fn failed<'a, I, A>(parser_error: ParseError<'a, I>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a;

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    I: Clone + 'a,
    A: 'a,
    B: 'a;

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    I: Clone + 'a,
    A: 'a,
    B: 'a;
}
