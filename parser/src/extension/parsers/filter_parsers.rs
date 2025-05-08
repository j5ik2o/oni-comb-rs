use crate::core::Parsers;

pub trait FilterParsers: Parsers {
  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a;
}
