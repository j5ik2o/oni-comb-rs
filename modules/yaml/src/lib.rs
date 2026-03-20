//! MVP YAML parser built on oni-comb-parser.

mod parser;
mod value;

pub use parser::{parse, parse_value, yaml, yaml_value};
pub use value::YamlValue;
