use crate::core::{ParseState, ParsedResult, Parser, ParserRunner};

impl<'a, I, A> ParserRunner<'a> for Parser<'a, I, A> {
  type Input = I;
  type Output = A;
  type P<'m, X, Y>
  where
    X: 'm,
  = Parser<'m, X, Y>;

  fn parse(&self, input: &'a [Self::Input]) -> ParsedResult<'a, Self::Input, Self::Output> {
    let parse_state = ParseState::new(input, 0);
    self.run(&parse_state)
  }

  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParsedResult<'a, Self::Input, Self::Output> {
    (self.method)(param)
  }
}
