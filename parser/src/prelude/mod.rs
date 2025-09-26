use crate::core::{CommittedStatus, ParseError, ParseResult, Parser};
use std::fmt::{Debug, Display};

pub type ParseResultType<'a, I, A> = ParseResult<'a, I, A>;

pub fn unit<'a, I>() -> Parser<'a, I, ()>
where
    I: 'a,
{
    Parser::new(|_, state| ParseResult::successful_with_state(state, (), 0))
}

pub fn empty<'a, I>() -> Parser<'a, I, ()>
where
    I: 'a,
{
    unit()
}

pub fn successful<'a, I, A>(value: A) -> Parser<'a, I, A>
where
    I: 'a,
    A: Clone + 'a,
{
    Parser::new(move |_, state| ParseResult::successful_with_state(state, value.clone(), 0))
}

pub fn successful_lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
where
    I: 'a,
    F: Fn() -> A + 'a,
    A: 'a,
{
    Parser::new(move |_, state| ParseResult::successful_with_state(state, f(), 0))
}

pub fn failed<'a, I, A>(
    error: ParseError<'a, I>,
    committed_status: CommittedStatus,
) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    Parser::new(move |_, _| ParseResult::failed(error.clone(), committed_status))
}

pub fn failed_with_commit<'a, I, A>(error: ParseError<'a, I>) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    failed(error, CommittedStatus::Committed)
}

pub fn failed_with_uncommit<'a, I, A>(error: ParseError<'a, I>) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    failed(error, CommittedStatus::Uncommitted)
}

pub fn end<'a, I>() -> Parser<'a, I, ()>
where
    I: Debug + Display + 'a,
{
    Parser::new(|input, state| {
        if state.current_offset() == input.len() {
            ParseResult::successful_with_state(state, (), 0)
        } else {
            ParseResult::failed(
                ParseError::of_custom(
                    state.current_offset(),
                    Some(&input[state.current_offset()..]),
                    "expected end of input",
                ),
                CommittedStatus::Uncommitted,
            )
        }
    })
}

pub fn begin<'a, I>() -> Parser<'a, I, ()>
where
    I: Debug + Display + 'a,
{
    Parser::new(|_, state| {
        if state.current_offset() == 0 {
            ParseResult::successful_with_state(state, (), 0)
        } else {
            ParseResult::failed(
                ParseError::of_custom(state.current_offset(), None, "expected beginning"),
                CommittedStatus::Uncommitted,
            )
        }
    })
}
