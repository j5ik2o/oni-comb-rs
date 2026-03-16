use crate::combinator::attempt::Attempt;
use crate::combinator::cut::Cut;
use crate::combinator::flat_map::FlatMap;
use crate::combinator::many::Many;
use crate::combinator::map::Map;
use crate::combinator::optional::Optional;
use crate::combinator::or::Or;
use crate::combinator::zip::Zip;
use crate::input::Input;
use crate::parser::Parser;

pub trait ParserExt<I: Input>: Parser<I> + Sized {
    fn map<F, O2>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Output) -> O2,
    {
        Map { parser: self, f }
    }

    fn zip<P2>(self, rhs: P2) -> Zip<Self, P2>
    where
        P2: Parser<I, Error = Self::Error>,
    {
        Zip {
            first: self,
            second: rhs,
        }
    }

    fn or<P2>(self, rhs: P2) -> Or<Self, P2>
    where
        P2: Parser<I, Output = Self::Output, Error = Self::Error>,
    {
        Or {
            left: self,
            right: rhs,
        }
    }

    fn attempt(self) -> Attempt<Self> {
        Attempt { parser: self }
    }

    fn cut(self) -> Cut<Self> {
        Cut { parser: self }
    }

    fn optional(self) -> Optional<Self> {
        Optional { parser: self }
    }

    fn many0(self) -> Many<Self> {
        Many { parser: self }
    }

    fn flat_map<F, P2>(self, f: F) -> FlatMap<Self, F>
    where
        P2: Parser<I, Error = Self::Error>,
        F: FnMut(Self::Output) -> P2,
    {
        FlatMap { parser: self, f }
    }

    fn and_then<F, P2>(self, f: F) -> FlatMap<Self, F>
    where
        P2: Parser<I, Error = Self::Error>,
        F: FnMut(Self::Output) -> P2,
    {
        FlatMap { parser: self, f }
    }
}

impl<I: Input, P: Parser<I> + Sized> ParserExt<I> for P {}
