#[derive(Debug, Clone, Copy)]
pub struct ParseState<'a, I> {
    original: &'a [I],
    offset: usize,
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
