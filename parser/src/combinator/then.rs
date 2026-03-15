use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct Then<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<I, P1, P2> Parser<I> for Then<P1, P2>
where
    I: Input,
    P1: Parser<I>,
    P2: Parser<I, Error = P1::Error>,
{
    type Output = (P1::Output, P2::Output);
    type Error = P1::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let a = self.first.parse_next(input)?;
        let b = self.second.parse_next(input)?;
        Ok((a, b))
    }
}
