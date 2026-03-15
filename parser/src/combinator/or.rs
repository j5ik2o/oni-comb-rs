use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Or<P1, P2> {
    pub(crate) left: P1,
    pub(crate) right: P2,
}

impl<I, P1, P2> Parser<I> for Or<P1, P2>
where
    I: Input,
    P1: Parser<I>,
    P2: Parser<I, Output = P1::Output, Error = P1::Error>,
{
    type Output = P1::Output;
    type Error = P1::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let cp = input.checkpoint();
        match self.left.parse_next(input) {
            Ok(v) => Ok(v),
            Err(Fail::Backtrack(_)) => {
                input.reset(cp);
                self.right.parse_next(input)
            }
            Err(Fail::Cut(e)) => Err(Fail::Cut(e)),
            Err(Fail::Incomplete) => Err(Fail::Incomplete),
            Err(Fail::ZeroProgress) => Err(Fail::ZeroProgress),
        }
    }
}
