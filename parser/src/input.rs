pub trait Input {
    type Checkpoint: Copy + Eq + Ord;
    type Slice<'a>
    where
        Self: 'a;

    fn checkpoint(&self) -> Self::Checkpoint;
    fn reset(&mut self, checkpoint: Self::Checkpoint);
    fn offset(&self) -> usize;
    fn remaining(&self) -> Self::Slice<'_>;
    fn is_eof(&self) -> bool;
}
