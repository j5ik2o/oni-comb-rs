use crate::error::ContextError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Context<P> {
    pub(crate) parser: P,
    pub(crate) label: &'static str,
}

impl<I, P> Parser<I> for Context<P>
where
    I: Input,
    P: Parser<I>,
    P::Error: ContextError,
{
    type Output = P::Output;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        match self.parser.parse_next(input) {
            Ok(v) => Ok(v),
            Err(Fail::Backtrack(e)) => {
                Err(Fail::Backtrack(e.add_context(self.label)))
            }
            Err(Fail::Cut(e)) => {
                Err(Fail::Cut(e.add_context(self.label)))
            }
            Err(e) => Err(e),
        }
    }
}
