use alloc::boxed::Box;

use crate::fail::PResult;
use crate::input::Input;

pub trait Parser<I: Input> {
    type Output;
    type Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error>;
}

impl<I: Input, P: Parser<I> + ?Sized> Parser<I> for Box<P> {
    type Output = P::Output;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        (**self).parse_next(input)
    }
}
