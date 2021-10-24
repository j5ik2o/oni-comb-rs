use std::fmt::Debug;

pub trait Element: Debug {
  fn is_ascii_zero(&self) -> bool;
  fn is_ascii_space(&self) -> bool;
  fn is_ascii_multi_space(&self) -> bool;
  fn is_ascii_whitespace(&self) -> bool;

  fn is_ascii(&self) -> bool;
  fn is_ascii_uppercase(&self) -> bool;
  fn is_ascii_lowercase(&self) -> bool;

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
  fn is_ascii_zero(&self) -> bool {
    *self == b'0'
  }

  fn is_ascii_space(&self) -> bool {
    *self == b' ' || *self == b'\t'
  }

  fn is_ascii_multi_space(&self) -> bool {
    self.is_ascii_space() || *self == b'\n' || *self == b'\r'
  }

  fn is_ascii_whitespace(&self) -> bool {
    u8::is_ascii_whitespace(self)
  }

  fn is_ascii(&self) -> bool {
    u8::is_ascii(self)
  }

  fn is_ascii_uppercase(&self) -> bool {
    u8::is_ascii_uppercase(self)
  }

  fn is_ascii_lowercase(&self) -> bool {
    u8::is_ascii_lowercase(self)
  }

  fn is_ascii_alpha(&self) -> bool {
    u8::is_ascii_alphabetic(self)
  }

  fn is_ascii_digit(&self) -> bool {
    u8::is_ascii_digit(self)
  }

  fn is_ascii_alpha_digit(&self) -> bool {
    u8::is_ascii_alphanumeric(self)
  }

  fn is_ascii_hex_digit(&self) -> bool {
    u8::is_ascii_hexdigit(self)
  }

  fn is_ascii_oct_digit(&self) -> bool {
    matches!(*self, b'0'..=b'8')
  }

  fn is_ascii_punctuation(&self) -> bool {
    u8::is_ascii_punctuation(self)
  }

  fn is_ascii_graphic(&self) -> bool {
    u8::is_ascii_graphic(self)
  }

  fn is_ascii_control(&self) -> bool {
    u8::is_ascii_control(self)
  }
}

impl Element for char {
  fn is_ascii_zero(&self) -> bool {
    *self == '0'
  }

  fn is_ascii_space(&self) -> bool {
    *self == ' ' || *self == '\t'
  }

  fn is_ascii_multi_space(&self) -> bool {
    self.is_ascii_space() || *self == '\n' || *self == '\r'
  }

  fn is_ascii_whitespace(&self) -> bool {
    char::is_ascii_whitespace(self)
  }

  fn is_ascii(&self) -> bool {
    char::is_ascii(self)
  }

  fn is_ascii_uppercase(&self) -> bool {
    char::is_ascii_uppercase(self)
  }

  fn is_ascii_lowercase(&self) -> bool {
    char::is_ascii_lowercase(self)
  }

  fn is_ascii_alpha(&self) -> bool {
    char::is_ascii_alphabetic(self)
  }

  fn is_ascii_digit(&self) -> bool {
    char::is_ascii_digit(self)
  }

  fn is_ascii_alpha_digit(&self) -> bool {
    char::is_ascii_alphanumeric(self)
  }

  fn is_ascii_hex_digit(&self) -> bool {
    char::is_ascii_hexdigit(self)
  }

  fn is_ascii_oct_digit(&self) -> bool {
    matches!(*self, '0'..='8')
  }

  fn is_ascii_punctuation(&self) -> bool {
    char::is_ascii_punctuation(self)
  }

  fn is_ascii_graphic(&self) -> bool {
    char::is_ascii_graphic(self)
  }

  fn is_ascii_control(&self) -> bool {
    char::is_ascii_control(self)
  }
}
