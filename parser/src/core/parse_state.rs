#[derive(Debug, Clone, Copy)]
pub struct ParseState<'a, I> {
    input: &'a [I],
    offset: usize,
}

impl<'a, I> ParseState<'a, I> {
    pub fn new(input: &'a [I], offset: usize) -> Self {
        Self { input, offset }
    }

    pub fn current_offset(&self) -> usize {
        self.offset
    }

    pub fn advance_by(&self, count: usize) -> Self {
        Self {
            input: self.input,
            offset: self.offset + count,
        }
    }

    pub fn input(&self) -> &'a [I] {
        &self.input[self.offset..]
    }
}
