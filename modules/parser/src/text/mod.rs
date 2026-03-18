pub mod char;
#[cfg(feature = "alloc")]
pub mod escaped;
pub mod float;
pub mod identifier;
pub mod integer;
pub mod lexeme;
#[cfg(feature = "alloc")]
pub mod quoted_string;
#[cfg(feature = "regex")]
pub mod regex;
pub mod tag;
pub mod take_while;
pub mod whitespace;

// primitive/ から re-export（後方互換性）
pub mod eof {
  pub use crate::primitive::eof::{eof, Eof};

  use crate::str_input::StrInput;

  pub fn str_eof<'a>() -> Eof<StrInput<'a>> {
    eof()
  }
}

pub mod satisfy {
  pub use crate::primitive::satisfy::{satisfy, Satisfy};

  use crate::str_input::StrInput;

  pub fn str_satisfy<'a, F: FnMut(char) -> bool>(f: F) -> Satisfy<F, StrInput<'a>> {
    satisfy(f)
  }
}
