use crate::core::{ParseResult, ParseState};
use std::marker::PhantomData;
use std::rc::Rc;

pub(crate) trait ParserCallable<'a, I: 'a, A: 'a>: 'a {
  fn call(&self, state: &ParseState<'a, I>) -> ParseResult<'a, I, A>;
}

pub(crate) struct DynamicParser<'a, I: 'a, A: 'a> {
  runner: Rc<dyn ParserCallable<'a, I, A> + 'a>,
}

impl<'a, I: 'a, A: 'a> Clone for DynamicParser<'a, I, A> {
  fn clone(&self) -> Self {
    Self {
      runner: self.runner.clone(),
    }
  }
}

impl<'a, I: 'a, A: 'a> DynamicParser<'a, I, A> {
  pub fn new<F>(func: F) -> Self
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    struct ClosureRunner<'a, I, A, F>
    where
      F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
      func: F,
      _phantom: PhantomData<(&'a I, A)>,
    }

    impl<'a, I: 'a, A: 'a, F> ParserCallable<'a, I, A> for ClosureRunner<'a, I, A, F>
    where
      F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a,
    {
      #[inline]
      fn call(&self, state: &ParseState<'a, I>) -> ParseResult<'a, I, A> {
        (self.func)(state)
      }
    }

    let runner = ClosureRunner {
      func,
      _phantom: PhantomData,
    };

    DynamicParser {
      runner: Rc::new(runner),
    }
  }

  #[inline]
  pub fn call(&self, state: &ParseState<'a, I>) -> ParseResult<'a, I, A> {
    self.runner.call(state)
  }
}

pub(crate) enum ParserMethod<'a, I: 'a, A: 'a> {
  Function(fn(&ParseState<'a, I>) -> ParseResult<'a, I, A>),
  Dynamic(DynamicParser<'a, I, A>),
}

impl<'a, I: 'a, A: 'a> Clone for ParserMethod<'a, I, A> {
  fn clone(&self) -> Self {
    match self {
      ParserMethod::Function(f) => ParserMethod::Function(*f),
      ParserMethod::Dynamic(inner) => ParserMethod::Dynamic(inner.clone()),
    }
  }
}

pub struct Parser<'a, I: 'a, A: 'a> {
  pub(crate) method: Rc<ParserMethod<'a, I, A>>,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A: 'a> Clone for Parser<'a, I, A> {
  fn clone(&self) -> Self {
    Self {
      method: self.method.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I: 'a, A: 'a> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    Parser {
      method: Rc::new(ParserMethod::Dynamic(DynamicParser::new(parse))),
      _phantom: PhantomData,
    }
  }

  pub fn from_fn(parse: fn(&ParseState<'a, I>) -> ParseResult<'a, I, A>) -> Parser<'a, I, A> {
    Parser {
      method: Rc::new(ParserMethod::Function(parse)),
      _phantom: PhantomData,
    }
  }

  #[inline]
  pub(crate) fn call(&self, state: &ParseState<'a, I>) -> ParseResult<'a, I, A> {
    match &*self.method {
      ParserMethod::Function(f) => f(state),
      ParserMethod::Dynamic(dynamic) => dynamic.call(state),
    }
  }

  #[inline]
  pub(crate) fn method_ptr(&self) -> *const ParserMethod<'a, I, A> {
    Rc::as_ptr(&self.method)
  }
}
