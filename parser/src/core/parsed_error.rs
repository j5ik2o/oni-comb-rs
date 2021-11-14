use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ParsedError<'a, I> {
  Mismatch {
    input: &'a [I],
    offset: usize,
    length: usize,
    message: String,
  },
  Conversion {
    input: &'a [I],
    offset: usize,
    length: usize,
    message: String,
  },
  Incomplete,
  Expect {
    offset: usize,
    inner: Box<ParsedError<'a, I>>,
    message: String,
  },
  Custom {
    offset: usize,
    inner: Option<Box<ParsedError<'a, I>>>,
    message: String,
  },
}

impl<'a, I> Display for ParsedError<'a, I> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParsedError::Incomplete => write!(f, "Incomplete"),
      ParsedError::Mismatch {
        ref message,
        ref offset,
        ..
      } => write!(f, "Mismatch at {}: {}", offset, message),
      ParsedError::Conversion {
        ref message,
        ref offset,
        ..
      } => write!(f, "Conversion failed at {}: {}", offset, message),
      ParsedError::Expect {
        ref message,
        ref offset,
        ref inner,
      } => write!(f, "{} at {}: {}", message, offset, inner),
      ParsedError::Custom {
        ref message,
        ref offset,
        inner: Some(ref inner),
      } => write!(f, "{} at {}, (inner: {})", message, offset, inner),
      ParsedError::Custom {
        ref message,
        ref offset,
        inner: None,
      } => write!(f, "{} at {}", message, offset),
    }
  }
}

impl<'a, I> ParsedError<'a, I> {
  pub fn is_expect(&self) -> bool {
    match self {
      ParsedError::Expect { .. } => true,
      _ => false,
    }
  }

  pub fn is_custom(&self) -> bool {
    match self {
      ParsedError::Custom { .. } => true,
      _ => false,
    }
  }

  pub fn is_mismatch(&self) -> bool {
    match self {
      ParsedError::Mismatch { .. } => true,
      _ => false,
    }
  }

  pub fn is_conversion(&self) -> bool {
    match self {
      ParsedError::Conversion { .. } => true,
      _ => false,
    }
  }

  pub fn is_in_complete(&self) -> bool {
    match self {
      ParsedError::Incomplete => true,
      _ => false,
    }
  }

  pub fn of_expect(offset: usize, inner: Box<ParsedError<'a, I>>, message: String) -> Self {
    ParsedError::Expect { offset, inner, message }
  }

  pub fn of_custom(offset: usize, inner: Option<Box<ParsedError<'a, I>>>, message: String) -> Self {
    ParsedError::Custom { offset, inner, message }
  }

  pub fn of_mismatch(input: &'a [I], offset: usize, length: usize, message: String) -> Self {
    ParsedError::Mismatch {
      input,
      offset,
      length,
      message,
    }
  }

  pub fn of_conversion(input: &'a [I], offset: usize, length: usize, message: String) -> Self {
    ParsedError::Conversion {
      input,
      offset,
      length,
      message,
    }
  }

  pub fn of_in_complete() -> Self {
    ParsedError::Incomplete
  }
}
