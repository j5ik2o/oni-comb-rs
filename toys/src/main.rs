mod environment;
mod expr;
mod interpreter;
mod labelled_parameter;
mod operator;
mod parsers;
mod values;

use oni_comb_parser_rs::prelude::*;

use crate::interpreter::Interpreter;

use crate::parsers::program;

fn main() {
  let source = r#"
    fn fizz_buzz(i) {
      if ((i % 3 == 0) && (i % 5 == 0)) {
        println("FizzBuzz");
      } else if (i % 3 == 0) {
        println("Fizz");
      } else if (i % 5 == 0) {
        println("Buzz");
      } else {
        println(i);
      }
    }
    fn main() {
      println("----");
      for (i in 1 to 100) {
        fizz_buzz(i);
      }
      println("----");
    }
    "#;
  let input = source.chars().collect::<Vec<_>>();
  let result = program().parse(&input).to_result().unwrap();
  println!("{:?}", result);
  Interpreter::new().call_main(result);
}
