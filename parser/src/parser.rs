use crate::fail::PResult;
use crate::input::Input;

pub trait Parser<I: Input> {
    type Output;
    type Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error>;
}
