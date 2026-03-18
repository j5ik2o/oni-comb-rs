#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod byte_input;
pub mod combinator;
pub mod error;
pub mod fail;
pub mod input;
pub mod ops;
pub mod parser;
pub mod parser_ext;
pub mod prelude;
pub mod primitive;
pub mod str_input;
pub mod text;
