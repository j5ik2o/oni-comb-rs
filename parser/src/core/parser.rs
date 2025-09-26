use crate::core::{parse_result::ParseResult, ParseState};
use std::rc::Rc;

pub struct Parser<'a, I, A> {
    runner: Rc<dyn Fn(&'a [I], ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a>,
}

impl<'a, I, A> Clone for Parser<'a, I, A> {
    fn clone(&self) -> Self {
        Self {
            runner: self.runner.clone(),
        }
    }
}

impl<'a, I, A> Parser<'a, I, A> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&'a [I], ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a,
    {
        Self { runner: Rc::new(f) }
    }

    pub fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
        let state = ParseState::new(input, 0);
        (self.runner)(input, state)
    }
}
