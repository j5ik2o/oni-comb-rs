use crate::expr::Expr;
use crate::labelled_parameter::LabelledParameter;
use oni_comb_parser_rs::core::{Parser, ParserFunctor};
use oni_comb_parser_rs::extension::parser::{
  ConversionParser, DiscardParser, LoggingParser, OperatorParser, RepeatParser, SkipParser,
};
use oni_comb_parser_rs::prelude::*;
use std::rc::Rc;

pub fn program<'a>() -> Parser<'a, char, Rc<Expr>> {
  space() * top_level_definition().of_many0().map(Expr::Program).map(Rc::new)
}

fn top_level_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  global_variable_definition() | function_definition()
}

fn function_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let define_p = space() * tag("define") * space() * ident() - space();
  let def_args_p = ident().of_many0_sep(comma()).surround(lparen(), rparen());
  let p = (define_p + def_args_p + block_expr())
    .map(|((name, args), body)| Expr::of_function_definition(name.to_string(), args, body));
  space() * p - space()
}

fn global_variable_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let global_p = space() * tag("global") - space();
  let global_indent_p = global_p * ident();
  let eq = space() * tag("=") - space();
  let p =
    (global_indent_p - eq + expression() - semi_colon()).map(|(name, e)| Expr::of_global_variable_definition(name, e));
  space() * p - space()
}

fn lines<'a>() -> Parser<'a, char, Vec<Rc<Expr>>> {
  line().of_many1() - space() - end()
}

fn line<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p =
    println() | lazy(while_expr) | lazy(if_expr) | lazy(for_in_expr) | assignment() | expression_line() | block_expr();
  space() * p - space()
}

fn while_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let while_p = space() * tag("while") - space();
  let condition = while_p * lazy(expression).surround(lparen(), rparen());
  let p = (condition + lazy(line)).map(|(c, body)| Expr::of_while(c, body));
  (space() * p - space()).attempt()
}

fn for_in_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let params_p = lparen() * ident() - (space() + tag("in") + space()) + expression() - (space() * tag("to") - space())
    + expression()
    - space()
    - rparen();
  let p0 = (tag("for") - space()) * params_p.log("params") + lazy(line);
  let p = p0.map(|(((name, from), to), body)| {
    Expr::of_block(vec![
      Expr::of_assignment(name.to_string(), from),
      Expr::of_while(
        Expr::of_less_than(Expr::of_symbol(name.to_string()), to),
        Expr::of_block(vec![
          body,
          Expr::of_assignment(
            name.to_string(),
            Expr::of_add(Expr::of_symbol(name.to_string()), Expr::of_integer_literal(1)),
          ),
        ]),
      ),
    ])
  });
  (space() * p - space()).attempt()
}

fn if_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let condition = (tag("if") - space()) * lparen() * expression() - rparen();
  let else_p = space() * tag("else") - space();
  let p = (condition + line() + (else_p * line()).opt()).map(|((p1, p2), p3)| Expr::of_if(p1, p2, p3));
  (space() * p - space()).attempt()
}

fn block_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(line).of_many0().surround(lbrace(), rbrace()).map(Expr::of_block);
  space() * p - space()
}

fn assignment<'a>() -> Parser<'a, char, Rc<Expr>> {
  let eq = space() * tag("=") - space();
  let p = (ident() - eq + expression() - semi_colon()).map(|(name, expr)| Expr::of_assignment(name, expr));
  (space() * p - space()).attempt()
}

fn expression_line<'a>() -> Parser<'a, char, Rc<Expr>> {
  (expression() - semi_colon()).attempt()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  comparative()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  let println_p = tag("println");
  let p = (println_p * lazy(expression).surround(lparen(), rparen()) - semi_colon()).map(Expr::of_println);
  (space() * p - space()).attempt()
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = regex(r#"^-?\d+"#)
    .map_res(|s| s.parse::<i64>())
    .map(Expr::of_integer_literal);
  space() * p - space()
}

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  chain_left1(
    primary(),
    (mul() | div()).log("operator").map(|e| match e {
      '*' => Expr::of_multiply,
      '/' => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  chain_left1(
    multitive(),
    (add() | subtract()).map(|e| match e {
      '+' => Expr::of_add,
      '-' => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn comparative<'a>() -> Parser<'a, char, Rc<Expr>> {
  let lt = tag("<");
  let lte = tag("<=");
  let gt = tag(">");
  let gte = tag(">=");
  let eqeq = tag("==");
  let neq = tag("!=");

  let p = chain_left1(
    additive(),
    (space() * (lte.attempt() | gte.attempt() | lt.attempt() | gt.attempt() | neq.attempt() | eqeq) - space()).map(
      |e| match e {
        "<=" => Expr::of_less_or_equal,
        ">=" => Expr::of_greater_or_equal,
        "<" => Expr::of_less_than,
        ">" => Expr::of_greater_than,
        "==" => Expr::of_equal_equal,
        "!=" => Expr::of_not_equal,
        _ => panic!("unexpected operator"),
      },
    ),
  );
  p
}

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (ident() + lazy(expression).of_many0_sep(comma()).surround(lparen(), rparen()))
    .map(|(name, params)| Expr::of_function_call(name.to_string(), params));
  (space() * p - space()).attempt()
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + lazy(expression)).map(|(label, param)| LabelledParameter::new(label, param));
  let p = (ident() + param.of_many1_sep(comma()).surround(lbracket(), rbracket()))
    .map(|(name, params)| Expr::of_labelled_call(name.to_string(), params));
  (space() * p - space()).attempt()
}

fn array_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression)
    .of_many0_sep(comma())
    .surround(lbracket(), rbracket())
    .map(Expr::of_array_literal);
  p
}

fn bool_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (tag("true").attempt() | tag("false")).map(|e| match e {
    "true" => Expr::of_bool_literal(true),
    "false" => Expr::of_bool_literal(false),
    _ => panic!("unexpected token"),
  });
  space() * p - space()
}

fn ident<'a>() -> Parser<'a, char, String> {
  space() * regex(r"[a-zA-Z_][a-zA-Z0-9_]*") - space()
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::of_symbol)
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let expr = (lparen() * lazy(expression) - rparen()).map(|e| Rc::new(Expr::Parenthesized(e)));
  expr | integer() | function_call() | labelled_call() | array_literal() | bool_literal() | identifier()
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::interpreter::Interpreter;
  use crate::labelled_parameter::LabelledParameter;
  use crate::operator::Operator;
  use oni_comb_parser_rs::core::ParserRunner;
  use std::env;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_example() {
    init();
    let source = r#"
    {
      a = 1;
      b = 2;
      c = a + b;
      println(c);
    }
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = line().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    Interpreter::new().interpret(result);
  }

  #[test]
  fn test_while() {
    let source = r"while(1==2){1;}";
    let input = source.chars().collect::<Vec<_>>();
    let result = line().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::While(
        Rc::new(Expr::Binary(
          Operator::EqualEqual,
          Rc::new(Expr::IntegerLiteral(1)),
          Rc::new(Expr::IntegerLiteral(2))
        )),
        Rc::new(Expr::Block(vec![Rc::new(Expr::IntegerLiteral(1))]))
      ),
      *result,
    );
  }

  #[test]
  fn test_for() {
    init();
    let source = r"for(i in 1 to 10) a=1;";
    let input = source.chars().collect::<Vec<_>>();
    let result = for_in_expr().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::Block(vec![
        Rc::new(Expr::Assignment("i".to_string(), Rc::new(Expr::IntegerLiteral(1)))),
        Rc::new(Expr::While(
          Rc::new(Expr::Binary(
            Operator::LessThan,
            Rc::new(Expr::Symbol("i".to_string())),
            Rc::new(Expr::IntegerLiteral(10)),
          )),
          Rc::new(Expr::Block(vec![
            Rc::new(Expr::Assignment("a".to_string(), Rc::new(Expr::IntegerLiteral(1)),)),
            Rc::new(Expr::Assignment(
              "i".to_string(),
              Rc::new(Expr::Binary(
                Operator::Add,
                Rc::new(Expr::Symbol("i".to_string())),
                Rc::new(Expr::IntegerLiteral(1))
              ))
            ))
          ])),
        )),
      ]),
      *result,
    );
  }

  #[test]
  fn test_if() {
    let source = r"if(1==2){1;}";
    let input = source.chars().collect::<Vec<_>>();
    let result = if_expr().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::If(
        Rc::new(Expr::Binary(
          Operator::EqualEqual,
          Rc::new(Expr::IntegerLiteral(1)),
          Rc::new(Expr::IntegerLiteral(2))
        )),
        Rc::new(Expr::Block(vec![Rc::new(Expr::IntegerLiteral(1))])),
        None
      ),
      *result
    );
  }

  #[test]
  fn test_assignment() {
    let source = r"i=1;";
    let input = source.chars().collect::<Vec<_>>();
    let result = line().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::Assignment("i".to_string(), Rc::new(Expr::IntegerLiteral(1))),
      *result
    );
  }

  #[test]
  fn test_println() {
    let source = r#"println(1+2*3);"#;
    let input = source.chars().collect::<Vec<_>>();
    let result = line().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    // assert_eq!(Expr::Println(Rc::new(Expr::IntegerLiteral(10))), *result);
    Interpreter::new().interpret(result);
  }

  #[test]
  fn test_primary_labelled_call_args_1() {
    let source = r#"
    abc[n=5]
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = labelled_call().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::LabelledCall(
        "abc".to_string(),
        vec![LabelledParameter::new(
          "n".to_string(),
          Rc::new(Expr::IntegerLiteral(5))
        )]
      ),
      *result
    );
  }

  #[test]
  fn test_primary_function_call_args_0() {
    let source = r#"
    abc();
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = function_call().parse_as_result(&input).unwrap();
    assert_eq!(Expr::FunctionCall("abc".to_string(), vec![]), *result);
  }

  #[test]
  fn test_primary_function_call_args_1() {
    let source = r#"
    abc(1);
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = function_call().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::FunctionCall("abc".to_string(), vec![Rc::new(Expr::IntegerLiteral(1))]),
      *result
    );
  }

  #[test]
  fn test_primary_function_call_args_2() {
    let source = r#"
    abc(1,2);
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = function_call().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::FunctionCall(
        "abc".to_string(),
        vec![Rc::new(Expr::IntegerLiteral(1)), Rc::new(Expr::IntegerLiteral(2))]
      ),
      *result
    );
  }

  #[test]
  fn test_primary_bool_true() {
    let source = r"true";
    let input = source.chars().collect::<Vec<_>>();
    let result = bool_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::BoolLiteral(true), *result);
  }

  #[test]
  fn test_primary_bool_false() {
    let source = r"false";
    let input = source.chars().collect::<Vec<_>>();
    let result = bool_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::BoolLiteral(false), *result);
  }

  #[test]
  fn test_primary_bool_array_0() {
    let source = r"[]";
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::ArrayLiteral(vec![]), *result);
  }

  #[test]
  fn test_primary_bool_array_1() {
    let source = r"[1]";
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::ArrayLiteral(vec![Rc::new(Expr::IntegerLiteral(1))]), *result);
  }

  #[test]
  fn test_primary_bool_array_2() {
    let source = r#"
    [1,2]
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::ArrayLiteral(vec![Rc::new(Expr::IntegerLiteral(1)), Rc::new(Expr::IntegerLiteral(2))]),
      *result
    );
  }

  #[test]
  fn test_primary_integer() {
    let source = r#"
    10
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = integer().parse_as_result(&input).unwrap();
    assert_eq!(Expr::IntegerLiteral(10), *result);
  }

  #[test]
  fn test_primary_identifier() {
    let source = r"abc";
    let input = source.chars().collect::<Vec<_>>();
    let result = identifier().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(Expr::Symbol("abc".to_string()), *result);
  }

  #[test]
  fn test_multitive() {
    init();
    let source = r"1/2";
    let input = source.chars().collect::<Vec<_>>();
    println!("start");

    let result = expression().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::Binary(
        Operator::Divide,
        Rc::new(Expr::IntegerLiteral(1)),
        Rc::new(Expr::IntegerLiteral(2))
      ),
      *result
    );
  }

  #[test]
  fn test_additive() {
    let source = r"1+2";
    let input = source.chars().collect::<Vec<_>>();
    let result = additive().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::Binary(
        Operator::Add,
        Rc::new(Expr::IntegerLiteral(1)),
        Rc::new(Expr::IntegerLiteral(2))
      ),
      *result
    );
  }

  #[test]
  fn test_comparative() {
    let source = r"1>2";
    let input = source.chars().collect::<Vec<_>>();
    let result = expression().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::Binary(
        Operator::GreaterThan,
        Rc::new(Expr::IntegerLiteral(1)),
        Rc::new(Expr::IntegerLiteral(2))
      ),
      *result
    );
  }

  #[test]
  fn test_comparative_symbol_number() {
    let source = r"a>2";
    let input = source.chars().collect::<Vec<_>>();
    let result = comparative().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::Binary(
        Operator::GreaterThan,
        Rc::new(Expr::Symbol("a".to_string())),
        Rc::new(Expr::IntegerLiteral(2))
      ),
      *result
    );
  }
}
fn add<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('+') - space()
}

fn subtract<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('-') - space()
}

fn mul<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('*') - space()
}

fn div<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('/') - space()
}
fn lbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("[") - space()
}

fn rbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("]") - space()
}

fn lbrace<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("{") - space()
}

fn rbrace<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("}") - space()
}

fn lparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("(") - space()
}

fn rparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(")") - space()
}

fn comma<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(",") - space()
}

fn semi_colon<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(";") - space()
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}
