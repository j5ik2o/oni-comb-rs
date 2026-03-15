use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Attempt<P> {
    pub(crate) parser: P,
}

impl<I, P> Parser<I> for Attempt<P>
where
    I: Input,
    P: Parser<I>,
{
    type Output = P::Output;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let cp = input.checkpoint();
        match self.parser.parse_next(input) {
            Ok(v) => Ok(v),
            Err(Fail::Cut(e)) => {
                input.reset(cp);
                Err(Fail::Backtrack(e))
            }
            Err(Fail::Backtrack(e)) => {
                input.reset(cp);
                Err(Fail::Backtrack(e))
            }
            Err(Fail::Incomplete) => Err(Fail::Incomplete),
            Err(Fail::ZeroProgress) => Err(Fail::ZeroProgress),
        }
    }
}
