mod environment;
mod expr;
mod interpreter;
mod labelled_parameter;
mod operator;
mod parsers;

use oni_comb_parser_rs::prelude::*;

use crate::interpreter::Interpreter;

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
