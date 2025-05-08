use std::fmt;
use std::fmt::Display;

/// The enum type representing the parsing error.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ParseError<'a, I> {
  /// Error when the parser's condition does not match
  Mismatch {
    input: &'a [I],
    offset: usize,
    length: usize,
    message: String,
  },
  /// Error when conversion fails
  Conversion {
    input: &'a [I],
    offset: usize,
    length: usize,
    message: String,
  },
  /// Error when parsing is interrupted or incomplete
  Incomplete,
  /// Error when the result deviates from expectations
  Expect {
    offset: usize,
    inner: Box<ParseError<'a, I>>,
    message: String,
  },
  /// Custom error
  Custom {
    offset: usize,
    inner: Option<Box<ParseError<'a, I>>>,
    message: String,
  },
}

impl<'a, I> Display for ParseError<'a, I> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::Incomplete => write!(f, "Incomplete"),
      ParseError::Mismatch {
        ref message,
        ref offset,
        ..
      } => write!(f, "Mismatch at {}: {}", offset, message),
      ParseError::Conversion {
        ref message,
        ref offset,
        ..
      } => write!(f, "Conversion failed at {}: {}", offset, message),
      ParseError::Expect {
        ref message,
        ref offset,
        ref inner,
      } => write!(f, "{} at {}: {}", message, offset, inner),
      ParseError::Custom {
        ref message,
        ref offset,
        inner: Some(ref inner),
      } => write!(f, "{} at {}, (inner: {})", message, offset, inner),
      ParseError::Custom {
        ref message,
        ref offset,
        inner: None,
      } => write!(f, "{} at {}", message, offset),
    }
  }
}

impl<'a> ParseError<'a, char> {
  pub fn input_string(&self) -> Option<String> {
    self.input().map(|chars| String::from_iter(chars))
  }
}

impl<'a> ParseError<'a, u8> {
  pub fn input_string(&self) -> Option<String> {
    match self.input() {
      Some(bytes) => match std::str::from_utf8(bytes) {
        Ok(s) => Some(s.to_string()),
        Err(_) => Some("".to_string()),
      },
      None => None,
    }
  }
}

impl<'a, I> ParseError<'a, I> {
  pub fn input(&self) -> Option<&'a [I]> {
    match self {
      ParseError::Incomplete => None,
      ParseError::Mismatch {
        input, offset, length, ..
      } => Some(&input[*offset..(*offset + length)]),
      ParseError::Conversion {
        input, offset, length, ..
      } => Some(&input[*offset..(*offset + length)]),
      ParseError::Expect { ref inner, .. } => inner.input(),
      ParseError::Custom {
        inner: Some(ref inner), ..
      } => inner.input(),
      ParseError::Custom { inner: None, .. } => None,
    }
  }

  pub fn is_expect(&self) -> bool {
    match self {
      ParseError::Expect { .. } => true,
      _ => false,
    }
  }

  pub fn is_custom(&self) -> bool {
    match self {
      ParseError::Custom { .. } => true,
      _ => false,
    }
  }

  pub fn is_mismatch(&self) -> bool {
    match self {
      ParseError::Mismatch { .. } => true,
      _ => false,
    }
  }

  pub fn is_conversion(&self) -> bool {
    match self {
      ParseError::Conversion { .. } => true,
      _ => false,
    }
  }

  pub fn is_in_complete(&self) -> bool {
    match self {
      ParseError::Incomplete => true,
      _ => false,
    }
  }

  pub fn of_expect(offset: usize, inner: Box<ParseError<'a, I>>, message: String) -> Self {
    ParseError::Expect { offset, inner, message }
  }

  pub fn of_custom(offset: usize, inner: Option<Box<ParseError<'a, I>>>, message: String) -> Self {
    ParseError::Custom { offset, inner, message }
  }

  pub fn of_mismatch(input: &'a [I], offset: usize, length: usize, message: String) -> Self {
    ParseError::Mismatch {
      input,
      offset,
      length,
      message,
    }
  }

  pub fn of_conversion(input: &'a [I], offset: usize, length: usize, message: String) -> Self {
    ParseError::Conversion {
      input,
      offset,
      length,
      message,
    }
  }

  pub fn of_in_complete() -> Self {
    ParseError::Incomplete
  }
}
