use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<I, P> Parser<I> for Cut<P>
where
    I: Input,
    P: Parser<I>,
{
    type Output = P::Output;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        match self.parser.parse_next(input) {
            Ok(v) => Ok(v),
            Err(Fail::Backtrack(e)) => Err(Fail::Cut(e)),
            Err(other) => Err(other),
        }
    }
}
