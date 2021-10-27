use crate::core::parse_error::ParseError;
use crate::core::parser_monad::ParserMonad;

pub trait Parsers {
  type P<'p, I, A>: ParserMonad<'p, Input = I, Output = A>
  where
    I: 'p,
    A: 'p;
  type PR<'p, I, A>: ParserMonad<'p, Input = I, Output = &'p A>
  where
    I: 'p,
    A: 'p;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a,
    'b: 'a;

  fn unit<'a, I>() -> Self::P<'a, I, ()> {
    Self::successful(|| ())
  }

  fn successful<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a;

  fn failed<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> ParseError<'a, I> + 'a,
    I: 'a,
    A: 'a;

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a,
    I: 'a,
    A: 'a;

  fn filter_ref<'a, I, A, F>(parser: Self::PR<'a, I, A>, f: F) -> Self::P<'a, I, &'a A>
  where
    F: Fn(&'a A) -> bool + 'a,
    I: 'a,
    A: 'a;

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a;

  fn flat_map_ref<'a, I, A, B, F>(parser: Self::PR<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(&'a A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a;

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a;

  fn map_ref<'a, I, A, B, F>(parser: Self::PR<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(&'a A) -> B + 'a,
    A: 'a,
    B: 'a;
}
