#[derive(Debug, Clone)]
pub enum ParseError<'a, I> {
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
  InComplete,
}

impl<'a, I> ParseError<'a, I> {
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
    ParseError::InComplete
  }
}
