#[derive(Debug)]
pub struct ParseError<'a, I> {
    pub offset: usize,
    pub message: String,
    pub remainder: Option<&'a [I]>,
}

impl<'a, I> Clone for ParseError<'a, I> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            message: self.message.clone(),
            remainder: self.remainder,
        }
    }
}

impl<'a, I> ParseError<'a, I> {
    pub fn new(offset: usize, message: impl Into<String>, remainder: Option<&'a [I]>) -> Self {
        Self {
            offset,
            message: message.into(),
            remainder,
        }
    }

    pub fn of_custom(
        offset: usize,
        remainder: Option<&'a [I]>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(offset, message, remainder)
    }

    pub fn of_in_complete() -> Self {
        Self::new(0, "incomplete input", None)
    }
}
