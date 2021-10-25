use oni_comb_rs::core::{Parser, ParserFunctor};
use oni_comb_rs::extension::parser::{DiscardParser, RepeatParser};
use oni_comb_rs::prelude::*;
use regex::Regex;
use std::rc::Rc;

#[derive(Debug)]
enum Expr {
  Symbol(String),
}

#[derive(Debug)]
struct LabelledParameter {
  name: String,
  parameter: Rc<Expr>,
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  todo!()
}

fn labelled_call<'a>() -> Parser<'a, char, LabelledParameter> {
  todo!()
  // ident().flat_map(|name| {
  //   ident().flat_map(move |label| {
  //     elm('=')
  //       * expression().map(|parameter| LabelledParameter {
  //         name: name.to_string(),
  //         parameter,
  //       })
  //   })
  // })
}

fn ident<'a>() -> Parser<'a, char, String> {
  regex(Regex::new(r#"[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap())
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::Symbol).map(Rc::new)
}

fn main() {}
