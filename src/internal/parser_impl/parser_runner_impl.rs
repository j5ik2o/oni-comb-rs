use crate::core::{ParseError, Parser, ParseResult, ParserRunner, ParseState};

impl<'a, I, A> ParserRunner<'a> for Parser<'a, I, A> {
  type Input = I;
  type Output = A;
  type P<'m, X, Y>
  where
    X: 'm,
  = Parser<'m, X, Y>;

  fn parse<'b>(&self, input: &'b [Self::Input]) -> Result<Self::Output, ParseError<'a, Self::Input>>
  where
    'b: 'a, {
    let parse_state = ParseState::new(input, 0);
    self.run(&parse_state).extract()
  }

  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParseResult<'a, Self::Input, Self::Output> {
    (self.method)(param)
  }
}
