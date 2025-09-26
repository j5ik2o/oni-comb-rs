use crate::core::{CommittedStatus, ParseError, ParseResult, ParseState};
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

    pub fn run(&self, input: &'a [I], state: ParseState<'a, I>) -> ParseResult<'a, I, A> {
        (self.runner)(input, state)
    }

    pub fn map<B, F>(self, f: F) -> Parser<'a, I, B>
    where
        F: Fn(A) -> B + 'a,
        A: 'a,
        B: 'a,
        I: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| parser.run(input, state).map(|value| f(value)))
    }

    pub fn flat_map<B, F>(self, f: F) -> Parser<'a, I, B>
    where
        F: Fn(A) -> Parser<'a, I, B> + 'a,
        A: 'a,
        B: 'a,
        I: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| {
            let original_state = state;
            parser
                .run(input, state)
                .flat_map(|value, _length, next_state| {
                    if let Some(next_state) = next_state {
                        f(value).run(input, next_state)
                    } else {
                        ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            original_state.current_offset(),
                            Some(original_state.input()),
                            "flat_map requires state information",
                        ))
                    }
                })
        })
    }

    pub fn attempt(self) -> Parser<'a, I, A>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| parser.run(input, state).with_uncommitted())
    }

    /// Convert the parser into one that never commits on failure, yielding `None` instead.
    pub fn optional(self) -> Parser<'a, I, Option<A>>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| match parser.run(input, state) {
            ParseResult::Success {
                value,
                length,
                state: Some(next_state),
            } => ParseResult::Success {
                value: Some(value),
                length,
                state: Some(next_state),
            },
            ParseResult::Success { state: None, .. } => {
                ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(state.input()),
                    "optional: parser did not return state",
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
                        value: None,
                        length: 0,
                        state: Some(state),
                    }
                }
            }
        })
    }

    /// Replace non-committed failure with a given default value.
    pub fn unwrap_or(self, default: A) -> Parser<'a, I, A>
    where
        I: 'a,
        A: Clone + 'a,
    {
        let optional_parser = self.optional();
        Parser::new(move |input, state| {
            let fallback = default.clone();
            optional_parser
                .run(input, state)
                .map(|value| value.unwrap_or(fallback))
        })
    }

    /// Replace non-committed failure by evaluating a lazy default.
    pub fn unwrap_or_else<F>(self, f: F) -> Parser<'a, I, A>
    where
        F: Fn() -> A + 'a,
        I: 'a,
        A: 'a,
    {
        let optional_parser = self.optional();
        Parser::new(move |input, state| {
            optional_parser.run(input, state).map(|value| match value {
                Some(v) => v,
                None => f(),
            })
        })
    }

    /// Fold a sequence of parsers with logical OR semantics.
    pub fn or_list<Itr>(parsers: Itr) -> Parser<'a, I, A>
    where
        I: 'a,
        A: 'a,
        Itr: IntoIterator<Item = Parser<'a, I, A>>,
    {
        let mut iter = parsers.into_iter();
        match iter.next() {
            Some(first) => iter.fold(first, |acc, parser| acc.or(parser)),
            None => Parser::new(|_, state| {
                ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(state.input()),
                    "or_list: empty iterator",
                ))
            }),
        }
    }

    /// Repeatedly apply the parser until `end` succeeds, returning collected values with the terminator.
    pub fn many_till<B>(self, end: Parser<'a, I, B>) -> Parser<'a, I, (Vec<A>, B)>
    where
        I: 'a,
        A: 'a,
        B: 'a,
    {
        let element_parser = self.clone();
        let end_parser = end.clone();
        Parser::new(move |input, state| {
            let mut items = Vec::new();
            let mut total_length = 0usize;
            let mut current_state = state;

            loop {
                match end_parser.clone().run(input, current_state) {
                    ParseResult::Success {
                        value: end_value,
                        length: end_length,
                        state: Some(next_state),
                    } => {
                        total_length += end_length;
                        return ParseResult::Success {
                            value: (items, end_value),
                            length: total_length,
                            state: Some(next_state),
                        };
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            current_state.current_offset(),
                            Some(current_state.input()),
                            "many_till: end parser did not return state",
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
                        }
                    }
                }

                match element_parser.clone().run(input, current_state) {
                    ParseResult::Success {
                        value,
                        length,
                        state: Some(next_state),
                    } => {
                        if length == 0
                            && next_state.current_offset() == current_state.current_offset()
                        {
                            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                                current_state.current_offset(),
                                Some(current_state.input()),
                                "many_till: parser did not advance state",
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
                            "many_till: parser did not return state",
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
        })
    }

    /// Skip input using the parser until `end` succeeds, returning the terminator's value.
    pub fn skip_till<B>(self, end: Parser<'a, I, B>) -> Parser<'a, I, B>
    where
        I: 'a,
        A: 'a,
        B: 'a,
    {
        self.many_till(end).map(|(_, end_value)| end_value)
    }

    /// Evaluate the parser repeatedly, discarding results until it fails without commitment.
    pub fn skip_many0(self) -> Parser<'a, I, ()>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| {
            let mut total_length = 0usize;
            let mut current_state = state;

            loop {
                let snapshot_state = current_state;
                match parser.run(input, current_state) {
                    ParseResult::Success {
                        length,
                        state: Some(next_state),
                        ..
                    } => {
                        let offset_before = snapshot_state.current_offset();
                        if length == 0 && next_state.current_offset() == offset_before {
                            current_state = next_state;
                            break;
                        }
                        total_length += length;
                        current_state = next_state;
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            snapshot_state.current_offset(),
                            Some(snapshot_state.input()),
                            "skip_many0: parser did not return state",
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
                            current_state = snapshot_state;
                            break;
                        }
                    }
                }
            }

            ParseResult::Success {
                value: (),
                length: total_length,
                state: Some(current_state),
            }
        })
    }

    /// Evaluate the parser at least once, discarding the matched input.
    pub fn skip_many1(self) -> Parser<'a, I, ()>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| match parser.run(input, state) {
            ParseResult::Success {
                length,
                state: Some(next_state),
                ..
            } => {
                if length == 0 && next_state.current_offset() == state.current_offset() {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        state.current_offset(),
                        Some(state.input()),
                        "skip_many1: parser did not advance state",
                    ));
                }

                match parser.clone().skip_many0().run(input, next_state) {
                    ParseResult::Success {
                        length: rest_length,
                        state: Some(final_state),
                        ..
                    } => ParseResult::Success {
                        value: (),
                        length: length + rest_length,
                        state: Some(final_state),
                    },
                    ParseResult::Success { state: None, .. } => {
                        ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            next_state.current_offset(),
                            Some(next_state.input()),
                            "skip_many1: continuation did not return state",
                        ))
                    }
                    ParseResult::Failure {
                        error,
                        committed_status,
                    } => ParseResult::Failure {
                        error,
                        committed_status,
                    },
                }
            }
            ParseResult::Success { state: None, .. } => {
                ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(state.input()),
                    "skip_many1: parser did not return state",
                ))
            }
            ParseResult::Failure {
                error,
                committed_status,
            } => ParseResult::Failure {
                error,
                committed_status,
            },
        })
    }

    pub fn or(self, other: Parser<'a, I, A>) -> Parser<'a, I, A>
    where
        I: 'a,
        A: 'a,
    {
        let first = self.clone();
        let second = other.clone();
        Parser::new(move |input, state| match first.run(input, state) {
            success @ ParseResult::Success { .. } => success,
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
                    second.run(input, state)
                }
            }
        })
    }

    pub fn or_else<F>(self, f: F) -> Parser<'a, I, A>
    where
        F: Fn() -> Parser<'a, I, A> + 'a,
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| match parser.run(input, state) {
            success @ ParseResult::Success { .. } => success,
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
                    f().run(input, state)
                }
            }
        })
    }

    pub fn filter<P>(self, predicate: P) -> Parser<'a, I, A>
    where
        P: Fn(&A) -> bool + 'a,
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| {
            let original_state = state;
            match parser.run(input, state) {
                ParseResult::Success {
                    value,
                    length,
                    state: next_state,
                } => {
                    if predicate(&value) {
                        ParseResult::Success {
                            value,
                            length,
                            state: next_state,
                        }
                    } else {
                        let (error_offset, remainder) = next_state
                            .map(|s| (s.current_offset(), Some(s.input())))
                            .unwrap_or_else(|| {
                                (
                                    original_state.current_offset(),
                                    Some(original_state.input()),
                                )
                            });
                        ParseResult::failed(
                            ParseError::of_custom(error_offset, remainder, "predicate failed"),
                            CommittedStatus::Uncommitted,
                        )
                    }
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => ParseResult::Failure {
                    error,
                    committed_status,
                },
            }
        })
    }

    pub fn many0(self) -> Parser<'a, I, Vec<A>>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| {
            let mut items = Vec::new();
            let mut total_length = 0usize;
            let mut current_state = state;

            loop {
                let snapshot_state = current_state;
                match parser.run(input, current_state) {
                    ParseResult::Success {
                        value,
                        length,
                        state: Some(next_state),
                    } => {
                        let offset_before = snapshot_state.current_offset();
                        if length == 0 && next_state.current_offset() == offset_before {
                            current_state = next_state;
                            break;
                        }
                        total_length += length;
                        current_state = next_state;
                        items.push(value);
                    }
                    ParseResult::Success { state: None, .. } => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            snapshot_state.current_offset(),
                            Some(snapshot_state.input()),
                            "many0 inner parser did not return state",
                        ));
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
                            current_state = snapshot_state;
                            break;
                        }
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

    pub fn many1(self) -> Parser<'a, I, Vec<A>>
    where
        I: 'a,
        A: 'a,
    {
        let parser = self.clone();
        Parser::new(move |input, state| {
            let original_state = state;
            match parser.run(input, state) {
                ParseResult::Success {
                    value,
                    length,
                    state: Some(next_state),
                } => {
                    let mut items = vec![value];
                    let mut total_length = length;
                    let mut current_state = next_state;

                    loop {
                        let snapshot_state = current_state;
                        match parser.run(input, current_state) {
                            ParseResult::Success {
                                value,
                                length,
                                state: Some(next_state),
                            } => {
                                let offset_before = snapshot_state.current_offset();
                                if length == 0 && next_state.current_offset() == offset_before {
                                    current_state = next_state;
                                    break;
                                }
                                total_length += length;
                                current_state = next_state;
                                items.push(value);
                            }
                            ParseResult::Success { state: None, .. } => {
                                return ParseResult::failed_with_uncommitted(
                                    ParseError::of_custom(
                                        snapshot_state.current_offset(),
                                        Some(snapshot_state.input()),
                                        "many1 inner parser did not return state",
                                    ),
                                );
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
                                    current_state = snapshot_state;
                                    break;
                                }
                            }
                        }
                    }

                    ParseResult::Success {
                        value: items,
                        length: total_length,
                        state: Some(current_state),
                    }
                }
                ParseResult::Success { state: None, .. } => {
                    ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        original_state.current_offset(),
                        Some(original_state.input()),
                        "many1 inner parser did not return state",
                    ))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => ParseResult::Failure {
                    error,
                    committed_status,
                },
            }
        })
    }
}
