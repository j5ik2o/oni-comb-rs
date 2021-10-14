#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

// https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing/Parsers.scala
use regex::Regex;

pub struct Location {
  input: String,
  offset: u32,
}

pub struct ParseError {
  stack: Vec<(Location, String)>,
}

pub trait Parser {
  type Elm;

  fn or<A, P>(&self, p: P) -> P
  where
    P: Parser<Elm = A>;

  fn pure<A, P>(a: A) -> P
  where
    P: Parser<Elm = A>;

  fn map<B, P, F>(self, f: F) -> P
  where
    P: Parser<Elm = B>,
    F: FnOnce(Self::Elm) -> B,
    Self: Sized, {
    self.flat_map(|e| Self::pure(f(e)))
  }

  fn flat_map<B, P, F>(self, f: F) -> P
  where
    P: Parser<Elm = B>,
    F: FnOnce(Self::Elm) -> P;
}

pub trait Parsers {
  type P<A>;

  fn run<A>(&self, p: Self::P<A>, input: &str) -> Result<A, ParseError>;

  fn string(&self, s: String) -> Self::P<String>;

  fn char(&self, c: char) -> Self::P<char> {
    let s = c.to_string();
    let p = self.string(s);
    self.map(p, |e| e.chars().nth(0).unwrap())
  }

  fn default_succeed<A>(&self, a: A) -> Self::P<A> {
    self.map(self.string("".to_string()), move |_| a)
  }

  fn succeed<A>(&self, a: A) -> Self::P<A>;

  fn slice<A>(&self, p: Self::P<A>) -> Self::P<String>;

  fn many1<A, PF>(&self, pf: PF) -> Self::P<Vec<A>>
  where
    PF: Fn() -> Self::P<A>, {
    self.map2(
      pf(),
      || self.many(pf),
      |a, b: Vec<A>| {
        let mut m = vec![];
        m.push(a);
        m.extend(b);
        m
      },
    )
  }

  fn list_of_n<A, PF>(&self, n: i32, pf: PF) -> Self::P<Vec<A>>
  where
    PF: Fn() -> Self::P<A>, {
    if n <= 0 {
      self.succeed(Vec::new())
    } else {
      self.map2(
        pf(),
        || self.list_of_n(n - 1, pf),
        |a, b: Vec<A>| {
          let mut m = vec![];
          m.push(a);
          m.extend(b);
          m
        },
      )
    }
  }

  fn many<A, PF>(&self, pf: PF) -> Self::P<Vec<A>>
  where
    PF: Fn() -> Self::P<A>, {
    let map2 = self.map2(
      pf(),
      || self.many(pf),
      |a, b: Vec<A>| {
        let mut m = vec![];
        m.push(a);
        m.extend(b);
        m
      },
    );
    self.or(map2, self.succeed(Vec::new()))
  }

  fn or<A>(&self, p1: Self::P<A>, p2: Self::P<A>) -> Self::P<A>;

  fn flat_map<A, B, PF>(&self, p: Self::P<A>, f: PF) -> Self::P<B>
  where
    PF: FnOnce(A) -> Self::P<B>;

  fn regex(&self, r: Regex) -> Self::P<String>;

  fn product<A, B, PF>(&self, p1: Self::P<A>, p2f: PF) -> Self::P<(A, B)>
  where
    PF: FnOnce() -> Self::P<B>, {
    self.flat_map(p1, |a| self.map(p2f(), move |b| (a, b)))
  }

  fn map2<A, B, C, PF, F>(&self, pa: Self::P<A>, pbf: PF, f: F) -> Self::P<C>
  where
    PF: FnOnce() -> Self::P<B>,
    F: Fn(A, B) -> C, {
    self.flat_map(pa, |a| self.map(pbf(), |b| f(a, b)))
  }

  fn map<A, B, F>(&self, p: Self::P<A>, f: F) -> Self::P<B>
  where
    F: FnOnce(A) -> B, {
    self.flat_map(p, move |e| self.succeed(f(e)))
  }

  fn label<A>(&self, msg: String, p: Self::P<A>) -> Self::P<A>;

  fn scope<A>(&self, msg: String, p: Self::P<A>) -> Self::P<A>;

  fn attempt<A>(&self, p: Self::P<A>) -> Self::P<A>;

  fn skip_l<A, B, PF>(&self, p1: Self::P<A>, p2f: PF) -> Self::P<B>
  where
    PF: FnOnce() -> Self::P<B>, {
    self.map2(self.slice(p1), p2f, |_, b| b)
  }

  fn skip_r<A, B, PF>(&self, p1: Self::P<A>, p2f: PF) -> Self::P<A>
  where
    PF: FnOnce() -> Self::P<B>, {
    self.map2(p1, || self.slice(p2f()), |a, _| a)
  }

  fn opt<A>(&self, p: Self::P<A>) -> Self::P<Option<A>> {
    self.or(self.map(p, Some), self.succeed(None))
  }

  fn whitespace(&self) -> Self::P<String> {
    self.regex(Regex::new("\\s*").unwrap())
  }

  fn digits(&self) -> Self::P<String> {
    self.regex(Regex::new("\\d+").unwrap())
  }

  fn double_string(&self) -> Self::P<String> {
    self.token(self.regex(Regex::new("[-+]?([0-9]*\\.)?[0-9]+([eE][-+]?[0-9]+)?").unwrap()))
  }

  fn double(&self) -> Self::P<f64> {
    let ds = self.double_string();
    let p = self.map(ds, |s: String| s.parse::<f64>().unwrap());
    self.label("double literal".to_string(), p)
  }

  fn token<A>(&self, p: Self::P<A>) -> Self::P<A> {
    self.skip_r(self.attempt(p), || self.whitespace())
  }

  // fn sep<X, A, PF>(&self, p1: Self::P<A>, p2f: PF) -> Self::P<Vec<A>>
  // where
  //   PF: FnOnce() -> Self::P<X>;

  // fn sep1<X, A, PF1, PF2>(&self, p1f: PF1, p2f: PF2) -> Self::P<Vec<A>>
  // where
  //   PF1: Fn() -> Self::P<A>,
  //   PF2: Fn() -> Self::P<X>,
  // {
  //   let a = self.many(|| self.skip_l(p2f(), p1f));
  //   self.map2(
  //     p1f(),
  //     || a,
  //     |a, b: Vec<A>| {
  //       let mut m = vec![];
  //       m.push(a);
  //       m.extend(b);
  //       m
  //     },
  //   )
  // }

  fn surround<X, Y, A>(&self, start: Self::P<X>, stop: Self::P<Y>, p: Self::P<A>) -> Self::P<A> {
    self.skip_l(start, || self.skip_r(p,  || stop))
  }

  fn eof(&self) -> Self::P<String> {
    self.label(
      "unexpected trailing characters".to_string(),
      self.regex(Regex::new("\\z").unwrap()),
    )
  }

  fn root<A>(&self, p: Self::P<A>) -> Self::P<A> {
    self.skip_r(p, || self.eof())
  }
}

fn main() {
  println!("Hello, world!");
}
