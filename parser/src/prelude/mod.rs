use crate::core::{CommittedStatus, ParseError, ParseResult, ParseState, Parser};
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

pub fn map<'a, I, A, B, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, B>
where
    F: Fn(A) -> B + 'a,
    I: 'a,
    A: 'a,
    B: 'a,
{
    parser.map(f)
}

pub fn flat_map<'a, I, A, B, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, B>
where
    F: Fn(A) -> Parser<'a, I, B> + 'a,
    I: 'a,
    A: 'a,
    B: 'a,
{
    parser.flat_map(f)
}

pub fn filter<'a, I, A, P>(parser: Parser<'a, I, A>, predicate: P) -> Parser<'a, I, A>
where
    P: Fn(&A) -> bool + 'a,
    I: 'a,
    A: 'a,
{
    parser.filter(predicate)
}

pub fn attempt<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    parser.attempt()
}

pub fn exists<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, bool>
where
    I: 'a,
    A: 'a,
{
    let probe = parser.clone();
    Parser::new(move |input, state| {
        let original_state = state;
        match probe.run(input, state) {
            ParseResult::Success { .. } => {
                ParseResult::successful_with_state(original_state, true, 0)
            }
            ParseResult::Failure { .. } => {
                ParseResult::successful_with_state(original_state, false, 0)
            }
        }
    })
}

pub fn not<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, ()>
where
    I: 'a,
    A: 'a,
{
    let probe = parser.clone();
    Parser::new(move |input, state| {
        let original_state = state;
        match probe.run(input, state) {
            ParseResult::Success { .. } => ParseResult::failed(
                ParseError::of_custom(
                    original_state.current_offset(),
                    Some(original_state.input()),
                    "unexpected successful match",
                ),
                CommittedStatus::Uncommitted,
            ),
            ParseResult::Failure { .. } => {
                ParseResult::successful_with_state(original_state, (), 0)
            }
        }
    })
}

pub fn skip_left<'a, I, A, B>(left: Parser<'a, I, A>, right: Parser<'a, I, B>) -> Parser<'a, I, B>
where
    I: 'a,
    A: 'a,
    B: 'a,
{
    left.flat_map(move |_| right.clone())
}

pub fn skip_right<'a, I, A, B>(left: Parser<'a, I, A>, right: Parser<'a, I, B>) -> Parser<'a, I, A>
where
    I: 'a,
    A: Clone + 'a,
    B: 'a,
{
    left.flat_map(move |value| {
        let right_clone = right.clone();
        right_clone.map(move |_| value.clone())
    })
}

pub fn surround<'a, I, L, A, R>(
    left: Parser<'a, I, L>,
    center: Parser<'a, I, A>,
    right: Parser<'a, I, R>,
) -> Parser<'a, I, A>
where
    I: 'a,
    L: 'a,
    A: Clone + 'a,
    R: 'a,
{
    skip_right(skip_left(left, center), right)
}

pub fn many0<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Vec<A>>
where
    I: 'a,
    A: 'a,
{
    parser.many0()
}

pub fn many1<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Vec<A>>
where
    I: 'a,
    A: 'a,
{
    parser.many1()
}

pub fn byte<'a>(value: u8) -> Parser<'a, u8, u8> {
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        match slice.first() {
            Some(&found) if found == value => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, found, 1)
            }
            Some(&found) => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!("expected byte {} but found {}", value, found),
            )),
            None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                None,
                format!("expected byte {} but reached end of input", value),
            )),
        }
    })
}

pub fn take_while1<'a, F>(predicate: F) -> Parser<'a, u8, &'a [u8]>
where
    F: Fn(u8) -> bool + 'a,
{
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        let mut len = 0usize;
        while len < slice.len() && predicate(slice[len]) {
            len += 1;
        }

        if len == 0 {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                "take_while1: predicate rejected at start",
            ));
        }

        let next_state = state.advance_by(len);
        ParseResult::successful_with_state(next_state, &slice[..len], len)
    })
}

pub fn separated_list1<'a, I, A, B>(
    element: Parser<'a, I, A>,
    separator: Parser<'a, I, B>,
) -> Parser<'a, I, Vec<A>>
where
    I: 'a,
    A: 'a,
    B: 'a,
{
    let element_parser = element.clone();
    let separator_parser = separator.clone();

    Parser::new(move |input, state| {
        let mut values = Vec::new();
        let mut total_length = 0usize;
        let mut current_state = state;

        match element_parser.run(input, current_state) {
            ParseResult::Success {
                value,
                length,
                state: Some(next_state),
            } => {
                total_length += length;
                current_state = next_state;
                values.push(value);
            }
            ParseResult::Success { state: None, .. } => {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    current_state.current_offset(),
                    Some(current_state.input()),
                    "separated_list1 element parser did not advance state",
                ))
            }
            ParseResult::Failure {
                error,
                committed_status,
            } => {
                return ParseResult::Failure {
                    error,
                    committed_status,
                }
            }
        }

        loop {
            match separator_parser.run(input, current_state) {
                ParseResult::Success {
                    length,
                    state: Some(next_state),
                    ..
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        break;
                    }
                    total_length += length;
                    current_state = next_state;
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "separated_list1 separator did not advance state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => {
                    if committed_status.is_committed() {
                        return ParseResult::Failure {
                            error,
                            committed_status,
                        };
                    } else {
                        break;
                    }
                }
            }

            match element_parser.run(input, current_state) {
                ParseResult::Success {
                    value,
                    length,
                    state: Some(next_state),
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        break;
                    }
                    total_length += length;
                    current_state = next_state;
                    values.push(value);
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "separated_list1 element did not advance state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => {
                    return ParseResult::Failure {
                        error,
                        committed_status,
                    };
                }
            }
        }

        ParseResult::Success {
            value: values,
            length: total_length,
            state: Some(current_state),
        }
    })
}

pub fn separated_fold1<'a, I, A, B, R, Init, Fold>(
    element: Parser<'a, I, A>,
    separator: Parser<'a, I, B>,
    init: Init,
    fold: Fold,
) -> Parser<'a, I, R>
where
    I: 'a,
    A: 'a,
    B: 'a,
    R: 'a,
    Init: Fn(A) -> R + 'a,
    Fold: Fn(R, A) -> R + 'a,
{
    let element_parser = element.clone();
    let separator_parser = separator.clone();

    Parser::new(move |input, state| {
        let mut total_length = 0usize;
        let mut current_state = state;

        let mut acc = match element_parser.run(input, current_state) {
            ParseResult::Success {
                value,
                length,
                state: Some(next_state),
            } => {
                total_length += length;
                current_state = next_state;
                init(value)
            }
            ParseResult::Success { state: None, .. } => {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    current_state.current_offset(),
                    Some(current_state.input()),
                    "separated_fold1 element parser did not advance state",
                ))
            }
            ParseResult::Failure {
                error,
                committed_status,
            } => {
                return ParseResult::Failure {
                    error,
                    committed_status,
                }
            }
        };

        loop {
            match separator_parser.run(input, current_state) {
                ParseResult::Success {
                    length,
                    state: Some(next_state),
                    ..
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        break;
                    }
                    total_length += length;
                    current_state = next_state;
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "separated_fold1 separator did not advance state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => {
                    if committed_status.is_committed() {
                        return ParseResult::Failure {
                            error,
                            committed_status,
                        };
                    } else {
                        break;
                    }
                }
            }

            match element_parser.run(input, current_state) {
                ParseResult::Success {
                    value,
                    length,
                    state: Some(next_state),
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        break;
                    }
                    total_length += length;
                    current_state = next_state;
                    acc = fold(acc, value);
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "separated_fold1 element did not advance state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => {
                    return ParseResult::Failure {
                        error,
                        committed_status,
                    };
                }
            }
        }

        ParseResult::Success {
            value: acc,
            length: total_length,
            state: Some(current_state),
        }
    })
}
