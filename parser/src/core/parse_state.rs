#[derive(Debug)]
pub struct ParseState<'a, I> {
    original: &'a [I],
    offset: usize,
}

impl<'a, I> Copy for ParseState<'a, I> {}

impl<'a, I> Clone for ParseState<'a, I> {
    fn clone(&self) -> Self {
        Self {
            original: self.original,
            offset: self.offset,
        }
    }
}

impl<'a, I> ParseState<'a, I> {
    pub fn new(input: &'a [I], offset: usize) -> Self {
        assert!(offset <= input.len());
        Self {
            original: input,
            offset,
        }
    }

    pub fn current_offset(&self) -> usize {
        self.offset
    }

    pub fn advance_by(&self, count: usize) -> Self {
        Self::new(self.original, self.offset + count)
    }

    pub fn input(&self) -> &'a [I] {
        &self.original[self.offset..]
    }

    pub fn original(&self) -> &'a [I] {
        self.original
    }

    pub fn len(&self) -> usize {
        self.original.len() - self.offset
    }
}
