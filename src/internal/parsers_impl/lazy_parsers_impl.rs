use std::fmt::Debug;
use crate::core::{Parser, ParserRunner};
use crate::extension::parsers::LazyParsers;
use crate::internal::ParsersImpl;

impl LazyParsers for ParsersImpl {
    fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
        where
            F: Fn() -> Self::P<'a, I, A> + 'a,
            A: Debug + 'a, {
        Parser::new(move |parse_state| {
            let parser = f();
            parser.run(parse_state)
        })
    }
}