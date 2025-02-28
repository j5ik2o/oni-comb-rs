use crate::core::{ParseResult, ParseState, Parser, ParserRunner};

impl<'a, I, A> ParserRunner<'a> for Parser<'a, I, A> {
  type Input = I;
  type Output = A;
  type P<'m, X, Y>
    = Parser<'m, X, Y>
  where
    X: 'm;

  #[inline]
  fn parse(&self, input: &'a [Self::Input]) -> ParseResult<'a, Self::Input, Self::Output> {
    // 初期状態を作成して実行
    let parse_state = ParseState::new(input, 0);
    self.run(&parse_state)
  }

  #[inline]
  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParseResult<'a, Self::Input, Self::Output> {
    // パーサー関数を直接呼び出し
    (self.method)(param)
  }
}
