use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct ChainR1<P, Op> {
    pub(crate) operand: P,
    pub(crate) operator: Op,
}

impl<I, P, Op, F> Parser<I> for ChainR1<P, Op>
where
    I: Input,
    P: Parser<I>,
    Op: Parser<I, Output = F, Error = P::Error>,
    F: FnMut(P::Output, P::Output) -> P::Output,
{
    type Output = P::Output;
    type Error = P::Error;

    fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
        let first = self.operand.parse_next(input)?;
        let mut operands = vec![first];
        let mut operators: Vec<F> = Vec::new();

        loop {
            let cp = input.checkpoint();
            match self.operator.parse_next(input) {
                Ok(f) => match self.operand.parse_next(input) {
                    Ok(v) => {
                        operators.push(f);
                        operands.push(v);
                    }
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

        // 右から畳む
        let mut acc = operands.pop().unwrap();
        while let Some(v) = operands.pop() {
            let mut f = operators.pop().unwrap();
            acc = f(v, acc);
        }
        Ok(acc)
    }
}
