use crate::core::parser_runner::ParserRunner;

pub trait ParserPure<'a>: ParserRunner<'a> {
  /// Returns the specified value [Parser].<br/>
  /// 指定した値を返す[Parser]を返す。
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a;
}
