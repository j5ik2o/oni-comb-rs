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
