use std::fmt::Debug;

pub trait Element: Debug {
  fn to_char(self) -> char;
  fn is_ascii_zero(&self) -> bool;
  fn is_ascii_space(&self) -> bool;
  fn is_ascii_multi_space(&self) -> bool;
  fn is_ascii_whitespace(&self) -> bool;

  fn is_ascii(&self) -> bool;
  fn is_ascii_alpha_uppercase(&self) -> bool;
  fn is_ascii_alpha_lowercase(&self) -> bool;

  fn is_ascii_alpha(&self) -> bool;
  fn is_ascii_digit(&self) -> bool;
  fn is_ascii_alpha_digit(&self) -> bool;

  fn is_ascii_hex_digit(&self) -> bool;
  fn is_ascii_oct_digit(&self) -> bool;

  fn is_ascii_punctuation(&self) -> bool;
  fn is_ascii_graphic(&self) -> bool;
  fn is_ascii_control(&self) -> bool;
}

impl Element for u8 {

  fn to_char(self) -> char {
    char::from(self)
  }

  fn is_ascii_zero(&self) -> bool {
    *self == b'0'
  }

  fn is_ascii_space(&self) -> bool {
    matches!(*self, b' ' | b'\t')
  }

  fn is_ascii_multi_space(&self) -> bool {
    matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
  }

  fn is_ascii_whitespace(&self) -> bool {
    matches!(*self, b'\t' | b'\n' | b'\x0C' | b'\r' | b' ')
  }

  fn is_ascii(&self) -> bool {
    *self & 128 == 0
  }

  fn is_ascii_alpha_uppercase(&self) -> bool {
    matches!(*self, b'A'..=b'Z')
  }

  fn is_ascii_alpha_lowercase(&self) -> bool {
    matches!(*self, b'a'..=b'z')
  }

  fn is_ascii_alpha(&self) -> bool {
    matches!(*self, b'A'..=b'Z' | b'a'..=b'z')
  }

  fn is_ascii_digit(&self) -> bool {
    matches!(*self, b'0'..=b'9')
  }

  fn is_ascii_alpha_digit(&self) -> bool {
    matches!(*self, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z')
  }

  fn is_ascii_hex_digit(&self) -> bool {
    matches!(*self, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')
  }

  fn is_ascii_oct_digit(&self) -> bool {
    matches!(*self, b'0'..=b'8')
  }

  fn is_ascii_punctuation(&self) -> bool {
    matches!(*self, b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~')
  }

  fn is_ascii_graphic(&self) -> bool {
    matches!(*self, b'!'..=b'~')
  }

  fn is_ascii_control(&self) -> bool {
    matches!(*self, b'\0'..=b'\x1F' | b'\x7F')
  }
}

impl Element for char {
  fn to_char(self) -> char {
    self
  }

  fn is_ascii_zero(&self) -> bool {
    *self == '0'
  }

  fn is_ascii_space(&self) -> bool {
    matches!(*self, ' ' | '\t')
  }

  fn is_ascii_multi_space(&self) -> bool {
    matches!(*self, ' ' | '\t' | '\n' | '\r')
  }

  fn is_ascii_whitespace(&self) -> bool {
    matches!(*self, '\t' | '\n' | '\x0C' | '\r' | ' ')
  }

  fn is_ascii(&self) -> bool {
    *self as u32 <= 0x7F
  }

  fn is_ascii_alpha_uppercase(&self) -> bool {
    matches!(*self, 'A'..='Z')
  }

  fn is_ascii_alpha_lowercase(&self) -> bool {
    matches!(*self, 'a'..='z')
  }

  fn is_ascii_alpha(&self) -> bool {
    matches!(*self, 'A'..='Z' | 'a'..='z')
  }

  fn is_ascii_digit(&self) -> bool {
    matches!(*self, '0'..='9')
  }

  fn is_ascii_alpha_digit(&self) -> bool {
    matches!(*self, '0'..='9' | 'A'..='Z' | 'a'..='z')
  }

  fn is_ascii_hex_digit(&self) -> bool {
    matches!(*self, '0'..='9' | 'A'..='F' | 'a'..='f')
  }

  fn is_ascii_oct_digit(&self) -> bool {
    matches!(*self, '0'..='8')
  }

  fn is_ascii_punctuation(&self) -> bool {
    matches!(*self, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~')
  }

  fn is_ascii_graphic(&self) -> bool {
    matches!(*self, '!'..='~')
  }

  fn is_ascii_control(&self) -> bool {
    matches!(*self, '\0'..='\x1F' | '\x7F')
  }
}
