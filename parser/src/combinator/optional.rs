use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Optional<P> {
    pub(crate) parser: P,
}

impl<I, P> Parser<I> for Optional<P>
where
    I: Input,
    P: Parser<I>,
{
    type Output = Option<P::Output>;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let cp = input.checkpoint();
        match self.parser.parse_next(input) {
            Ok(v) => Ok(Some(v)),
            Err(Fail::Backtrack(_)) => {
                input.reset(cp);
                Ok(None)
            }
            Err(Fail::Cut(e)) => Err(Fail::Cut(e)),
            Err(Fail::Incomplete) => Err(Fail::Incomplete),
            Err(Fail::ZeroProgress) => Err(Fail::ZeroProgress),
        }
    }
}
