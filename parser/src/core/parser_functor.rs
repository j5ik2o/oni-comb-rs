use crate::core::parser_pure::ParserPure;

pub trait ParserFunctor<'a>: ParserPure<'a> {
  /// Returns a [Parser] that transforms the analysis results.<br/>
  /// 解析結果を変換する[Parser]を返す。
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: 'a,
    Self::Output: Clone + 'a,
    B: Clone + 'a;
}
