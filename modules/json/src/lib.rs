//! RFC 8259 compliant JSON parser built on oni-comb-parser.

mod parser;
mod value;

pub use parser::{json, json_value, parse, parse_value};
pub use value::JsonValue;
