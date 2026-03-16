use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct ChainL1<P, Op> {
    pub(crate) operand: P,
    pub(crate) operator: Op,
}

impl<I, P, Op, F> Parser<I> for ChainL1<P, Op>
where
    I: Input,
    P: Parser<I>,
    Op: Parser<I, Output = F, Error = P::Error>,
    F: FnMut(P::Output, P::Output) -> P::Output,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let mut acc = self.operand.parse_next(input)?;
        loop {
            let cp = input.checkpoint();
            match self.operator.parse_next(input) {
                Ok(mut f) => match self.operand.parse_next(input) {
                    Ok(rhs) => acc = f(acc, rhs),
                    Err(Fail::Backtrack(_)) => {
                        input.reset(cp);
                        break;
                    }
                    Err(e) => return Err(e),
                },
                Err(Fail::Backtrack(_)) => {
                    input.reset(cp);
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(acc)
    }
}
