use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Many<P> {
    pub(crate) parser: P,
}

impl<I, P> Parser<I> for Many<P>
where
    I: Input,
    P: Parser<I>,
{
    type Output = Vec<P::Output>;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let mut items = Vec::new();
        loop {
            let cp = input.checkpoint();
            match self.parser.parse_next(input) {
                Ok(v) => {
                    if input.checkpoint() == cp {
                        return Err(Fail::ZeroProgress);
                    }
                    items.push(v);
                }
                Err(Fail::Backtrack(_)) => {
                    input.reset(cp);
                    return Ok(items);
                }
                Err(Fail::Cut(e)) => return Err(Fail::Cut(e)),
                Err(Fail::Incomplete) => return Err(Fail::Incomplete),
                Err(Fail::ZeroProgress) => return Err(Fail::ZeroProgress),
            }
        }
    }
}
