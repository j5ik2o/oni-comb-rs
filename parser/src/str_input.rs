use crate::input::Input;

pub struct StrInput<'a> {
    src: &'a str,
    offset: usize,
}

impl<'a> StrInput<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, offset: 0 }
    }

    pub(crate) fn advance(&mut self, n: usize) {
        self.offset += n;
    }
}

impl<'a> Input for StrInput<'a> {
    type Checkpoint = usize;
    type Slice<'s> = &'s str where Self: 's;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.offset
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        self.offset = checkpoint;
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn remaining(&self) -> &str {
        &self.src[self.offset..]
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.src.len()
    }
}
