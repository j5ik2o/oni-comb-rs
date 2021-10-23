use crate::core::{Parser, ParseResult, ParserRunner};
use crate::extension::parsers::OffsetParsers;
use crate::internal::ParsersImpl;

impl OffsetParsers for ParsersImpl {
    fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
        where
            A: 'a, {
        Parser::new(move |parse_state| match parser.run(parse_state) {
            ParseResult::Success { length, .. } => {
                let ps = parse_state.add_offset(length);
                ParseResult::successful(ps.last_offset().unwrap_or(0), length)
            }
            ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
    }

    fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
        where
            A: 'a, {
        Parser::new(move |parse_state| match parser.run(parse_state) {
            ParseResult::Success { length, .. } => {
                let ps = parse_state.add_offset(length);
                ParseResult::successful(ps.next_offset(), length)
            }
            ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
    }
}