use std::fmt::{Debug, Display};

/// A Element.
pub trait Element: PartialEq + PartialOrd + Display + Copy + Debug {
  /// Convert to a character.
  fn to_char(self) -> char;
  /// Check if it is an ASCII space.
  fn is_ascii_space(&self) -> bool;
  /// Check if it is an ASCII space including newlines.
  fn is_ascii_multi_space(&self) -> bool;
  /// Check if it is an ASCII whitespace.
  fn is_ascii_whitespace(&self) -> bool;

  /// Check if it is an ASCII character.
  fn is_ascii(&self) -> bool;
  /// Check if it is an uppercase ASCII alphabet character.
  fn is_ascii_alpha_uppercase(&self) -> bool;
  /// Check if it is a lowercase ASCII alphabet character.
  fn is_ascii_alpha_lowercase(&self) -> bool;

  /// Check if it is an ASCII alphabet character.
  fn is_ascii_alpha(&self) -> bool;

  /// Check if it is an ASCII digit.
  fn is_ascii_digit(&self) -> bool;
  /// Check if it is the digit zero.
  fn is_ascii_digit_zero(&self) -> bool;
  /// Check if it is a non-zero digit.
  fn is_ascii_digit_non_zero(&self) -> bool;
  /// Check if it is an ASCII alphanumeric character.
  fn is_ascii_alpha_digit(&self) -> bool;

  /// Check if it is an ASCII hexadecimal digit.
  fn is_ascii_hex_digit(&self) -> bool;
  /// Check if it is an ASCII octal digit.
  fn is_ascii_oct_digit(&self) -> bool;

  /// Check if it is an ASCII punctuation character.
  fn is_ascii_punctuation(&self) -> bool;
  /// Check if it is an ASCII graphic character.
  fn is_ascii_graphic(&self) -> bool;
  /// Check if it is an ASCII control character.
  fn is_ascii_control(&self) -> bool;
}

impl Element for u8 {
  fn to_char(self) -> char {
    char::from(self)
  }

  fn is_ascii_space(&self) -> bool {
    matches!(*self, b' ' | b'\t')
  }

  fn is_ascii_multi_space(&self) -> bool {
    self.is_ascii_space() || matches!(*self, b'\n' | b'\r')
  }

  fn is_ascii_whitespace(&self) -> bool {
    self.is_ascii_multi_space() || *self == b'\x0C'
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

  fn is_ascii_digit_zero(&self) -> bool {
    *self == b'0'
  }

  fn is_ascii_digit_non_zero(&self) -> bool {
    !self.is_ascii_digit_zero()
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

  fn is_ascii_space(&self) -> bool {
    matches!(*self, ' ' | '\t')
  }

  fn is_ascii_multi_space(&self) -> bool {
    self.is_ascii_space() || matches!(*self, '\n' | '\r')
  }

  fn is_ascii_whitespace(&self) -> bool {
    self.is_ascii_multi_space() || *self == '\x0C'
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

  fn is_ascii_digit_zero(&self) -> bool {
    *self == '0'
  }

  fn is_ascii_digit_non_zero(&self) -> bool {
    !self.is_ascii_digit_zero()
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

impl<'a> Element for &'a char {
  fn to_char(self) -> char {
    *self
  }

  fn is_ascii_space(&self) -> bool {
    matches!(**self, ' ' | '\t')
  }

  fn is_ascii_multi_space(&self) -> bool {
    self.is_ascii_space() || matches!(**self, '\n' | '\r')
  }

  fn is_ascii_whitespace(&self) -> bool {
    self.is_ascii_multi_space() || **self == '\x0C'
  }

  fn is_ascii(&self) -> bool {
    **self as u32 <= 0x7F
  }

  fn is_ascii_alpha_uppercase(&self) -> bool {
    matches!(**self, 'A'..='Z')
  }

  fn is_ascii_alpha_lowercase(&self) -> bool {
    matches!(**self, 'a'..='z')
  }

  fn is_ascii_alpha(&self) -> bool {
    matches!(**self, 'A'..='Z' | 'a'..='z')
  }

  fn is_ascii_digit(&self) -> bool {
    matches!(**self, '0'..='9')
  }

  fn is_ascii_digit_zero(&self) -> bool {
    **self == '0'
  }

  fn is_ascii_digit_non_zero(&self) -> bool {
    !self.is_ascii_digit_zero()
  }

  fn is_ascii_alpha_digit(&self) -> bool {
    matches!(**self, '0'..='9' | 'A'..='Z' | 'a'..='z')
  }

  fn is_ascii_hex_digit(&self) -> bool {
    matches!(**self, '0'..='9' | 'A'..='F' | 'a'..='f')
  }

  fn is_ascii_oct_digit(&self) -> bool {
    matches!(**self, '0'..='8')
  }

  fn is_ascii_punctuation(&self) -> bool {
    matches!(**self, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~')
  }

  fn is_ascii_graphic(&self) -> bool {
    matches!(**self, '!'..='~')
  }

  fn is_ascii_control(&self) -> bool {
    matches!(**self, '\0'..='\x1F' | '\x7F')
  }
}
