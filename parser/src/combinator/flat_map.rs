use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct FlatMap<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<I, P, F, P2> Parser<I> for FlatMap<P, F>
where
    I: Input,
    P: Parser<I>,
    P2: Parser<I, Error = P::Error>,
    F: FnMut(P::Output) -> P2,
{
    type Output = P2::Output;
    type Error = P::Error;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let v = self.parser.parse_next(input)?;
        let mut p2 = (self.f)(v);
        p2.parse_next(input)
    }
}
