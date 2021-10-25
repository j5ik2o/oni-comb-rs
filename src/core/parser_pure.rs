use crate::core::parser_runner::ParserRunner;

pub trait ParserPure<'a>: ParserRunner<'a> {
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a;
}
