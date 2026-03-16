#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fail<E> {
    Backtrack(E),
    Cut(E),
    Incomplete,
    ZeroProgress,
}

pub type PResult<T, E> = Result<T, Fail<E>>;
