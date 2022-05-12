use crate::core::parser_runner::ParserRunner;

pub trait ParserFilter<'a>: ParserRunner<'a> {
  /// 解析結果をフィルターする[Parser]を返す。
  fn with_filter<F>(self, f: F) -> Self
  where
    F: Fn(&Self::Output) -> bool + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    Self: Sized;

  /// 解析結果をフィルターする[Parser]を返す。
  fn with_filter_not<F>(self, f: F) -> Self
  where
    F: Fn(&Self::Output) -> bool + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    Self: Sized, {
    self.with_filter(move |e| !f(e))
  }
}
