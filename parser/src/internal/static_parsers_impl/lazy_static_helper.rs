use crate::core::{StaticParser, ParseResult};
use std::fmt::Debug;

/// Helper function to create a static string from a tag
/// This is used to avoid lifetime issues with lazy_static
pub fn lazy_static_str<'a>(s: &str) -> String {
    s.to_string()
}

/// Helper function to create a StaticParser that returns a static string
/// This is used to avoid lifetime issues with lazy_static
pub fn lazy_static_parser<'a>() -> StaticParser<'a, char, String> {
    StaticParser::new(move |parse_state| {
        let input = parse_state.input();
        let offset = parse_state.next_offset();
        
        if offset + 3 <= input.len() {
            if input[offset] == 'a' && input[offset + 1] == 'b' && input[offset + 2] == 'c' {
                ParseResult::successful("abc".to_string(), 3)
            } else {
                ParseResult::failed_with_uncommitted(
                    crate::core::ParseError::of_mismatch(input, offset, 0, "expected 'abc'".to_string())
                )
            }
        } else {
            ParseResult::failed_with_uncommitted(
                crate::core::ParseError::of_mismatch(input, offset, 0, "unexpected end of input".to_string())
            )
        }
    })
}
