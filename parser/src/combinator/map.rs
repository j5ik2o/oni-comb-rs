use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<I, P, F, O2> Parser<I> for Map<P, F>
where
    I: Input,
    P: Parser<I>,
    F: FnMut(P::Output) -> O2,
{
    type Output = O2;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        match self.parser.parse_next(input) {
            Ok(v) => Ok((self.f)(v)),
            Err(e) => Err(e),
        }
    }
}
