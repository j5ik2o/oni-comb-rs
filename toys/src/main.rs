mod operator;
mod expr;
mod labelled_parameter;
mod environment;
mod interpreter;
mod parsers;
use operator::*;

use oni_comb_parser_rs::core::{Parser, ParserFunctor, ParserRunner};
use oni_comb_parser_rs::extension::parser::{
  ConversionParser, DiscardParser, LoggingParser, OperatorParser, RepeatParser, SkipParser,
};
use oni_comb_parser_rs::prelude::*;

use std::collections::HashMap;

use std::rc::Rc;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::labelled_parameter::LabelledParameter;
use crate::parsers::program;


fn main() {
  let source = r#"
    define sub(i) {
      if (i > 3) {
        println(i);
      }
    }
    define main() {
      for (i in 1 to 10) {
        sub(i);
      }
    }
    "#;
  let input = source.chars().collect::<Vec<_>>();
  let result = program().parse(&input).to_result().unwrap();
  println!("{:?}", result);
  Interpreter::new().call_main(result);
}

