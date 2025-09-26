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

pub fn or<'a, I, A>(left: Parser<'a, I, A>, right: Parser<'a, I, A>) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    left.or(right)
}

pub fn or_else<'a, I, A, F>(parser: Parser<'a, I, A>, fallback: F) -> Parser<'a, I, A>
where
    F: Fn() -> Parser<'a, I, A> + 'a,
    I: 'a,
    A: 'a,
{
    parser.or_else(fallback)
}

pub fn optional<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Option<A>>
where
    I: 'a,
    A: 'a,
{
    parser.optional()
}

pub fn unwrap_or<'a, I, A>(parser: Parser<'a, I, A>, default: A) -> Parser<'a, I, A>
where
    I: 'a,
    A: Clone + 'a,
{
    parser.unwrap_or(default)
}

pub fn unwrap_or_else<'a, I, A, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, A>
where
    F: Fn() -> A + 'a,
    I: 'a,
    A: 'a,
{
    parser.unwrap_or_else(f)
}

pub fn choice<'a, I, A, It>(parsers: It) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
    It: IntoIterator<Item = Parser<'a, I, A>>,
{
    Parser::or_list(parsers)
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

pub fn peek<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
{
    let probe = parser.clone();
    Parser::new(move |input, state| {
        let original_state = state;
        match probe.run(input, state) {
            ParseResult::Success { value, .. } => ParseResult::Success {
                value,
                length: 0,
                state: Some(original_state),
            },
            ParseResult::Failure { error, .. } => {
                ParseResult::failed(error, CommittedStatus::Uncommitted)
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

pub fn skip_many0<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, ()>
where
    I: 'a,
    A: 'a,
{
    parser.skip_many0()
}

pub fn skip_many1<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, ()>
where
    I: 'a,
    A: 'a,
{
    parser.skip_many1()
}

pub fn chain_left1<'a, I, A, OpFn>(
    element: Parser<'a, I, A>,
    operator: Parser<'a, I, OpFn>,
) -> Parser<'a, I, A>
where
    I: 'a,
    A: 'a,
    OpFn: Fn(A, A) -> A + 'a,
{
    let element_parser = element.clone();
    let operator_parser = operator.clone();

    Parser::new(move |input, state| match element_parser.run(input, state) {
        ParseResult::Success {
            mut value,
            length,
            state: Some(mut current_state),
        } => {
            let mut total_length = length;

            loop {
                let operator_start_state = current_state;
                match operator_parser.run(input, current_state) {
                    ParseResult::Success {
                        value: combine,
                        length: op_length,
                        state: Some(next_state),
                    } => {
                        if op_length == 0
                            && next_state.current_offset() == operator_start_state.current_offset()
                        {
                            current_state = next_state;
                            break;
                        }

                        total_length += op_length;

                        match element_parser.run(input, next_state) {
                            ParseResult::Success {
                                value: rhs,
                                length: rhs_length,
                                state: Some(final_state),
                            } => {
                                if rhs_length == 0
                                    && final_state.current_offset() == next_state.current_offset()
                                {
                                    return ParseResult::failed_with_uncommitted(
                                        ParseError::of_custom(
                                            next_state.current_offset(),
                                            Some(next_state.input()),
                                            "chain_left1 element parser did not advance state",
                                        ),
                                    );
                                }

                                total_length += rhs_length;
                                current_state = final_state;
                                value = combine(value, rhs);
                            }
                            ParseResult::Success { state: None, .. } => {
                                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                                    next_state.current_offset(),
                                    Some(next_state.input()),
                                    "chain_left1 element parser did not return state",
                                ))
                            }
                            ParseResult::Failure {
                                error,
                                committed_status,
                            } => {
                                return ParseResult::Failure {
                                    error,
                                    committed_status: committed_status
                                        .or(CommittedStatus::Committed),
                                };
                            }
                        }
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            operator_start_state.current_offset(),
                            Some(operator_start_state.input()),
                            "chain_left1 operator parser did not return state",
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
            }

            ParseResult::Success {
                value,
                length: total_length,
                state: Some(current_state),
            }
        }
        ParseResult::Success { state: None, .. } => {
            ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(state.input()),
                "chain_left1 element parser did not return state",
            ))
        }
        failure => failure,
    })
}

pub fn chain_left0<'a, I, A, OpFn>(
    element: Parser<'a, I, A>,
    operator: Parser<'a, I, OpFn>,
) -> Parser<'a, I, Option<A>>
where
    I: 'a,
    A: 'a,
    OpFn: Fn(A, A) -> A + 'a,
{
    chain_left1(element, operator).optional()
}

pub fn chain_right1<'a, I, A, OpFn>(
    element: Parser<'a, I, A>,
    operator: Parser<'a, I, OpFn>,
) -> Parser<'a, I, A>
where
    I: 'a,
    A: Clone + 'a,
    OpFn: Fn(A, A) -> A + 'a,
{
    let element_parser = element.clone();
    let operator_parser = operator.clone();

    Parser::new(move |input, state| {
        chain_right1_internal(input, &element_parser, &operator_parser, state)
    })
}

pub fn chain_right0<'a, I, A, OpFn>(
    element: Parser<'a, I, A>,
    operator: Parser<'a, I, OpFn>,
) -> Parser<'a, I, Option<A>>
where
    I: 'a,
    A: Clone + 'a,
    OpFn: Fn(A, A) -> A + 'a,
{
    chain_right1(element, operator).optional()
}

pub fn elm<'a, I>(expected: I) -> Parser<'a, I, I>
where
    I: PartialEq + Copy + Debug + 'a,
{
    Parser::new(move |_: &'a [I], state: ParseState<'a, I>| {
        let slice = state.input();
        match slice.first().copied() {
            Some(found) if found == expected => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, found, 1)
            }
            Some(found) => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!("elm: expected {:?} but found {:?}", expected, found),
            )),
            None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                None,
                format!("elm: expected {:?} but reached end of input", expected),
            )),
        }
    })
}

pub fn seq<'a, I>(expected: Vec<I>) -> Parser<'a, I, &'a [I]>
where
    I: PartialEq + Debug + 'a,
{
    Parser::new(move |_: &'a [I], state: ParseState<'a, I>| {
        let expected_slice = expected.as_slice();
        let slice = state.input();
        let expected_len = expected_slice.len();

        if slice.len() < expected_len {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!(
                    "seq: expected sequence {:?} but input shorter ({} < {})",
                    expected_slice,
                    slice.len(),
                    expected_len,
                ),
            ));
        }

        if slice.starts_with(expected_slice) {
            let next_state = state.advance_by(expected_len);
            ParseResult::successful_with_state(next_state, &slice[..expected_len], expected_len)
        } else {
            ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!("seq: expected prefix {:?}", expected_slice),
            ))
        }
    })
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

pub fn one_of<'a>(candidates: Vec<u8>) -> Parser<'a, u8, u8> {
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        match slice.first().copied() {
            Some(found) => {
                if candidates.contains(&found) {
                    let next_state = state.advance_by(1);
                    ParseResult::successful_with_state(next_state, found, 1)
                } else {
                    ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        state.current_offset(),
                        Some(slice),
                        format!("one_of: byte {} not in set", found),
                    ))
                }
            }
            None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                None,
                "one_of: reached end of input",
            )),
        }
    })
}

fn take_while_internal<'a, F>(
    predicate: F,
    require_one: bool,
    label: &'static str,
) -> Parser<'a, u8, &'a [u8]>
where
    F: Fn(u8) -> bool + 'a,
{
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        let mut len = 0usize;
        while len < slice.len() && predicate(slice[len]) {
            len += 1;
        }

        if require_one && len == 0 {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!("{}: predicate rejected at start", label),
            ));
        }

        let next_state = state.advance_by(len);
        ParseResult::successful_with_state(next_state, &slice[..len], len)
    })
}

pub fn take<'a, I>(count: usize) -> Parser<'a, I, &'a [I]>
where
    I: 'a,
{
    Parser::new(move |_: &'a [I], state: ParseState<'a, I>| {
        let slice = state.input();
        if slice.len() < count {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!(
                    "take: expected at least {} elements but found {}",
                    count,
                    slice.len(),
                ),
            ));
        }

        let next_state = state.advance_by(count);
        ParseResult::successful_with_state(next_state, &slice[..count], count)
    })
}

pub fn take_while<'a, F>(predicate: F) -> Parser<'a, u8, &'a [u8]>
where
    F: Fn(u8) -> bool + 'a,
{
    take_while_internal(predicate, false, "take_while")
}

pub fn take_while0<'a, F>(predicate: F) -> Parser<'a, u8, &'a [u8]>
where
    F: Fn(u8) -> bool + 'a,
{
    take_while_internal(predicate, false, "take_while0")
}

pub fn take_while1<'a, F>(predicate: F) -> Parser<'a, u8, &'a [u8]>
where
    F: Fn(u8) -> bool + 'a,
{
    take_while_internal(predicate, true, "take_while1")
}

fn take_until_internal<'a>(
    target: Vec<u8>,
    require_match: bool,
    label: &'static str,
) -> Parser<'a, u8, &'a [u8]> {
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        let target_len = target.len();

        if target_len == 0 {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!("{}: target pattern must not be empty", label),
            ));
        }

        if slice.len() < target_len {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                format!(
                    "{}: input shorter than target ({} < {})",
                    label,
                    slice.len(),
                    target_len,
                ),
            ));
        }

        let mut index = 0usize;
        while index + target_len <= slice.len() {
            if slice[index..index + target_len] == target[..] {
                if require_match && index == 0 {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        state.current_offset(),
                        Some(slice),
                        format!("{}: found target at start", label),
                    ));
                }

                let next_state = state.advance_by(index);
                return ParseResult::successful_with_state(next_state, &slice[..index], index);
            }
            index += 1;
        }

        ParseResult::failed_with_uncommitted(ParseError::of_custom(
            state.current_offset(),
            Some(slice),
            format!("{}: target sequence not found", label),
        ))
    })
}

pub fn take_until<'a>(target: Vec<u8>) -> Parser<'a, u8, &'a [u8]> {
    take_until_internal(target, false, "take_until")
}

pub fn take_until1<'a>(target: Vec<u8>) -> Parser<'a, u8, &'a [u8]> {
    take_until_internal(target, true, "take_until1")
}

pub fn many_till<'a, I, A, B>(
    parser: Parser<'a, I, A>,
    end: Parser<'a, I, B>,
) -> Parser<'a, I, (Vec<A>, B)>
where
    I: 'a,
    A: 'a,
    B: 'a,
{
    parser.many_till(end)
}

pub fn skip_till<'a, I, A, B>(parser: Parser<'a, I, A>, end: Parser<'a, I, B>) -> Parser<'a, I, B>
where
    I: 'a,
    A: 'a,
    B: 'a,
{
    parser.skip_till(end)
}

pub fn repeat<'a, I, A>(parser: Parser<'a, I, A>, count: usize) -> Parser<'a, I, Vec<A>>
where
    I: 'a,
    A: 'a,
{
    let element_parser = parser.clone();
    Parser::new(move |input, state| {
        if count == 0 {
            return ParseResult::Success {
                value: Vec::new(),
                length: 0,
                state: Some(state),
            };
        }

        let mut items = Vec::with_capacity(count);
        let mut total_length = 0usize;
        let mut current_state = state;

        for _ in 0..count {
            match element_parser.run(input, current_state) {
                ParseResult::Success {
                    value,
                    length,
                    state: Some(next_state),
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "repeat element parser did not advance state",
                        ));
                    }
                    total_length += length;
                    current_state = next_state;
                    items.push(value);
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "repeat element parser did not return state",
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
            value: items,
            length: total_length,
            state: Some(current_state),
        }
    })
}

pub fn repeat_sep<'a, I, A, B>(
    parser: Parser<'a, I, A>,
    separator: Parser<'a, I, B>,
    count: usize,
) -> Parser<'a, I, Vec<A>>
where
    I: 'a,
    A: 'a,
    B: 'a,
{
    let element_parser = parser.clone();
    let separator_parser = separator.clone();

    Parser::new(move |input, state| {
        if count == 0 {
            return ParseResult::Success {
                value: Vec::new(),
                length: 0,
                state: Some(state),
            };
        }

        let mut values = Vec::with_capacity(count);
        let mut total_length = 0usize;
        let mut current_state = state;

        for index in 0..count {
            match element_parser.run(input, current_state) {
                ParseResult::Success {
                    value,
                    length,
                    state: Some(next_state),
                } => {
                    if length == 0 && next_state.current_offset() == current_state.current_offset()
                    {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "repeat_sep element parser did not advance state",
                        ));
                    }
                    total_length += length;
                    current_state = next_state;
                    values.push(value);
                }
                ParseResult::Success { state: None, .. } => {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        current_state.current_offset(),
                        Some(current_state.input()),
                        "repeat_sep element parser did not return state",
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

            if index + 1 < count {
                match separator_parser.run(input, current_state) {
                    ParseResult::Success {
                        length,
                        state: Some(next_state),
                        ..
                    } => {
                        if length == 0
                            && next_state.current_offset() == current_state.current_offset()
                        {
                            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                                current_state.current_offset(),
                                Some(current_state.input()),
                                "repeat_sep separator did not advance state",
                            ));
                        }
                        total_length += length;
                        current_state = next_state;
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "repeat_sep separator did not return state",
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
        }

        ParseResult::Success {
            value: values,
            length: total_length,
            state: Some(current_state),
        }
    })
}

pub fn take_while1_fold<'a, F, Init, Fold, R>(
    predicate: F,
    init: Init,
    fold: Fold,
) -> Parser<'a, u8, R>
where
    F: Fn(u8) -> bool + 'a,
    Init: Fn(u8) -> R + 'a,
    Fold: Fn(R, u8) -> R + 'a,
    R: 'a,
{
    Parser::new(move |_: &'a [u8], state: ParseState<'a, u8>| {
        let slice = state.input();
        if slice.is_empty() {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                "take_while1_fold: empty input",
            ));
        }

        let mut len = 0usize;
        while len < slice.len() && predicate(slice[len]) {
            len += 1;
        }

        if len == 0 {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                "take_while1_fold: predicate rejected at start",
            ));
        }

        let mut acc = init(slice[0]);
        for &byte in &slice[1..len] {
            acc = fold(acc, byte);
        }

        let next_state = state.advance_by(len);
        ParseResult::successful_with_state(next_state, acc, len)
    })
}

fn chain_right1_internal<'a, I, A, OpFn>(
    input: &'a [I],
    element: &Parser<'a, I, A>,
    operator: &Parser<'a, I, OpFn>,
    state: ParseState<'a, I>,
) -> ParseResult<'a, I, A>
where
    I: 'a,
    A: Clone + 'a,
    OpFn: Fn(A, A) -> A + 'a,
{
    match element.run(input, state) {
        ParseResult::Success {
            value,
            length,
            state: Some(next_state),
        } => {
            let mut total_length = length;

            match operator.run(input, next_state) {
                ParseResult::Success {
                    value: combine,
                    length: op_length,
                    state: Some(op_state),
                } => {
                    if op_length == 0 && op_state.current_offset() == next_state.current_offset() {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            next_state.current_offset(),
                            Some(next_state.input()),
                            "chain_right1 operator did not advance state",
                        ));
                    }

                    total_length += op_length;

                    match chain_right1_internal(input, element, operator, op_state) {
                        ParseResult::Success {
                            value: rhs,
                            length: rhs_length,
                            state: Some(final_state),
                        } => {
                            total_length += rhs_length;
                            ParseResult::Success {
                                value: combine(value, rhs),
                                length: total_length,
                                state: Some(final_state),
                            }
                        }
                        ParseResult::Success { state: None, .. } => {
                            ParseResult::failed_with_uncommitted(ParseError::of_custom(
                                op_state.current_offset(),
                                Some(op_state.input()),
                                "chain_right1 recursive parser did not return state",
                            ))
                        }
                        ParseResult::Failure {
                            error,
                            committed_status,
                        } => ParseResult::Failure {
                            error,
                            committed_status: committed_status.or(CommittedStatus::Committed),
                        },
                    }
                }
                ParseResult::Success { state: None, .. } => {
                    ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        next_state.current_offset(),
                        Some(next_state.input()),
                        "chain_right1 operator did not return state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => {
                    if committed_status.is_committed() {
                        ParseResult::Failure {
                            error,
                            committed_status,
                        }
                    } else {
                        ParseResult::Success {
                            value,
                            length: total_length,
                            state: Some(next_state),
                        }
                    }
                }
            }
        }
        ParseResult::Success { state: None, .. } => {
            ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(state.input()),
                "chain_right1 element parser did not return state",
            ))
        }
        failure => failure,
    }
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

pub fn separated_list0<'a, I, A, B>(
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

    Parser::new(move |input, state| match element_parser.run(input, state) {
        ParseResult::Success {
            value,
            length,
            state: Some(next_state),
        } => {
            let mut values = vec![value];
            let mut total_length = length;
            let mut current_state = next_state;

            loop {
                match separator_parser.run(input, current_state) {
                    ParseResult::Success {
                        length: sep_length,
                        state: Some(after_sep),
                        ..
                    } => {
                        if sep_length == 0
                            && after_sep.current_offset() == current_state.current_offset()
                        {
                            break;
                        }
                        total_length += sep_length;
                        current_state = after_sep;
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "separated_list0 separator did not return state",
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
                        length: elem_length,
                        state: Some(next_state),
                    } => {
                        if elem_length == 0
                            && next_state.current_offset() == current_state.current_offset()
                        {
                            break;
                        }
                        total_length += elem_length;
                        current_state = next_state;
                        values.push(value);
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "separated_list0 element did not return state",
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
        }
        ParseResult::Success { state: None, .. } => {
            ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(state.input()),
                "separated_list0 element parser did not return state",
            ))
        }
        ParseResult::Failure {
            error,
            committed_status,
        } => {
            if committed_status.is_committed() {
                ParseResult::Failure {
                    error,
                    committed_status,
                }
            } else {
                ParseResult::Success {
                    value: Vec::new(),
                    length: 0,
                    state: Some(state),
                }
            }
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
