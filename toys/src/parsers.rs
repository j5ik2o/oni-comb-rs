use crate::expr::Expr;
use crate::labelled_parameter::LabelledParameter;
use oni_comb_parser_rs::prelude::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::iter::FromIterator;
use std::rc::Rc;

fn ident<'a>() -> Parser<'a, char, String> {
  space() * regex(r"[a-zA-Z_][a-zA-Z0-9_]*") - space()
}

fn add<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('+') - space()
}

fn subtract<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('-') - space()
}

fn r#mod<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('%') - space()
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

pub fn program<'a>() -> Parser<'a, char, Rc<Expr>> {
  space() * top_level_definition().of_many0().map(Expr::Program).map(Rc::new)
}

fn top_level_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  (global_variable_definition() | function_definition()).name("top level definition")
}

fn function_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let define = space() * tag("fn") * space() * ident() - space();
  let args = ident().of_many0_sep(comma()).surround(lparen(), rparen());
  let p =
    (define + args + block()).map(|((name, args), body)| Expr::of_function_definition(name.to_string(), args, body));
  (space() * p - space()).name("function definition").cache()
}

fn global_variable_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let global = space() * tag("global") - space();
  let global_indent = global * ident();
  let eq = space() * tag("=") - space();
  let p =
    (global_indent - eq + expression() - semi_colon()).map(|(name, e)| Expr::of_global_variable_definition(name, e));
  (space() * p - space()).name("global variable definition").cache()
}

fn lines<'a>() -> Parser<'a, char, Vec<Rc<Expr>>> {
  line().of_many1() - space() - end()
}

fn line<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = println() | lazy(r#while) | lazy(r#if) | lazy(r#for) | assignment() | expression_line() | block();
  (space() * p - space()).name("line").cache()
}

fn r#while<'a>() -> Parser<'a, char, Rc<Expr>> {
  let r#while = space() * tag("while") - space();
  let condition = r#while * lazy(expression).surround(lparen(), rparen());
  let p = (condition + lazy(line)).map(|(c, body)| Expr::of_while(c, body));
  (space() * p - space()).attempt().name("while").cache()
}

fn r#for<'a>() -> Parser<'a, char, Rc<Expr>> {
  let r#for = tag("for") - space();
  let r#in = space() + tag("in") + space();
  let to = space() * tag("to") - space();
  let params = lparen() * ident() - r#in + expression() - to + expression() - space() - rparen();
  let p0 = r#for * params.debug("params") + lazy(line);
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
  (space() * p - space()).attempt().name("for").cache()
}

fn r#if<'a>() -> Parser<'a, char, Rc<Expr>> {
  let r#if = tag("if") - space();
  let condition = r#if * lparen() * expression() - rparen();
  let r#else = space() * tag("else") - space();
  let p = (condition + line() + (r#else * line()).opt()).map(|((p1, p2), p3)| Expr::of_if(p1, p2, p3));
  (space() * p - space()).attempt().name("if").cache()
}

fn block<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(line).of_many0().surround(lbrace(), rbrace()).map(Expr::of_block);
  (space() * p - space()).name("block").cache()
}

fn assignment<'a>() -> Parser<'a, char, Rc<Expr>> {
  let eq = space() * tag("=") - space();
  let p = (ident() - eq + expression() - semi_colon()).map(|(name, expr)| Expr::of_assignment(name, expr));
  (space() * p - space()).attempt().name("assignment").cache()
}

fn expression_line<'a>() -> Parser<'a, char, Rc<Expr>> {
  (expression() - semi_colon()).attempt().name("expression_line").cache()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  comparative().name("expression").cache()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  let r#println = tag("println");
  let p = (r#println * lazy(expression).surround(lparen(), rparen()) - semi_colon()).map(Expr::of_println);
  (space() * p - space()).attempt().name("println").cache()
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = regex(r#"^-?\d+"#)
    .map_res(|s| s.parse::<i64>())
    .map(Expr::of_integer_literal);
  (space() * p - space()).name("integer").cache()
}

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  primary()
    .chain_left1((mul() | div()).debug("operator").map(|e| match e {
      '*' => Expr::of_multiply,
      '/' => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }))
    .name("multitive")
    .cache()
}

fn moditive<'a>() -> Parser<'a, char, Rc<Expr>> {
  multitive()
    .chain_left1(r#mod().map(|e| match e {
      '%' => Expr::of_mod,
      _ => panic!("unexpected operator"),
    }))
    .name("moditive")
    .cache()
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  moditive()
    .chain_left1((add() | subtract()).map(|e| match e {
      '+' => Expr::of_add,
      '-' => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }))
    .name("additive")
    .cache()
}

fn comparative<'a>() -> Parser<'a, char, Rc<Expr>> {
  let lt = tag("<");
  let lte = tag("<=");
  let gt = tag(">");
  let gte = tag(">=");
  let eqeq = tag("==");
  let neq = tag("!=");
  let and = tag("&&");
  let or = tag("||");

  additive()
    .chain_left1(
      (space()
        * (and.attempt()
          | or.attempt()
          | lte.attempt()
          | gte.attempt()
          | lt.attempt()
          | gt.attempt()
          | neq.attempt()
          | eqeq)
        - space())
      .map(|e| match e {
        "&&" => Expr::of_and,
        "||" => Expr::of_or,
        "<=" => Expr::of_less_or_equal,
        ">=" => Expr::of_greater_or_equal,
        "<" => Expr::of_less_than,
        ">" => Expr::of_greater_than,
        "==" => Expr::of_equal_equal,
        "!=" => Expr::of_not_equal,
        _ => panic!("unexpected operator"),
      }),
    )
    .name("comparative")
    .cache()
}

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (ident() + lazy(expression).of_many0_sep(comma()).surround(lparen(), rparen()))
    .map(|(name, params)| Expr::of_function_call(name.to_string(), params));
  (space() * p - space()).attempt().name("function_call").cache()
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + lazy(expression)).map(|(label, param)| LabelledParameter::new(label, param));
  let p = (ident() + param.of_many1_sep(comma()).surround(lbracket(), rbracket()))
    .map(|(name, params)| Expr::of_labelled_call(name.to_string(), params));
  (space() * p - space()).attempt().name("labelled_call").cache()
}

fn array_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression)
    .of_many0_sep(comma())
    .surround(lbracket(), rbracket())
    .map(Expr::of_array_literal);
  (space() * p - space()).name("array_literal").cache()
}

fn bool_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (tag("true").attempt() | tag("false")).map(|e| match e {
    "true" => Expr::of_bool_literal(true),
    "false" => Expr::of_bool_literal(false),
    _ => panic!("unexpected token"),
  });
  (space() * p - space()).name("bool_literal").cache()
}

fn string_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let special_char = elm_ref('\\')
    | elm_ref('/')
    | elm_ref('"')
    | elm_ref('b').map(|_| &'\x08')
    | elm_ref('f').map(|_| &'\x0C')
    | elm_ref('n').map(|_| &'\n')
    | elm_ref('r').map(|_| &'\r')
    | elm_ref('t').map(|_| &'\t');
  let escape_sequence = elm_ref('\\') * special_char;
  let char_string = (none_ref_of("\\\"") | escape_sequence)
    .map(Clone::clone)
    .of_many1()
    .map(String::from_iter);
  let utf16_char: Parser<char, u16> = tag("\\u")
    * elm_pred(|c: &char| c.is_digit(16))
      .of_count(4)
      .map(String::from_iter)
      .map_res(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string = (char_string | utf16_string)
    .of_many0()
    .surround(elm_ref('"'), elm_ref('"'));
  string
    .map(|strings| Expr::of_string_literal(strings.concat()))
    .attempt()
    .name("string_literal")
    .cache()
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::of_symbol).name("identifier").cache()
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let expr = (lparen() * lazy(expression) - rparen()).map(|e| Rc::new(Expr::Parenthesized(e)));
  (expr
    | integer()
    | string_literal()
    | function_call()
    | labelled_call()
    | array_literal()
    | bool_literal()
    | identifier())
  .cache()
  .name("primary")
  .cache()
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::interpreter::Interpreter;
  use crate::labelled_parameter::LabelledParameter;
  use crate::operator::Operator;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_example() {
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
      Expr::of_while(
        Expr::of_binary(
          Operator::EqualEqual,
          Expr::of_integer_literal(1),
          Expr::of_integer_literal(2)
        ),
        Expr::of_block(vec![Expr::of_integer_literal(1)])
      ),
      result,
    );
  }

  #[test]
  fn test_for() {
    let source = r"for(i in 1 to 10) a=1;";
    let input = source.chars().collect::<Vec<_>>();
    let result = r#for().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::of_block(vec![
        Expr::of_assignment("i".to_string(), Expr::of_integer_literal(1)),
        Expr::of_while(
          Expr::of_binary(
            Operator::LessThan,
            Expr::of_symbol("i".to_string()),
            Expr::of_integer_literal(10),
          ),
          Expr::of_block(vec![
            Expr::of_assignment("a".to_string(), Expr::of_integer_literal(1)),
            Expr::of_assignment(
              "i".to_string(),
              Expr::of_binary(
                Operator::Add,
                Expr::of_symbol("i".to_string()),
                Expr::of_integer_literal(1)
              )
            )
          ]),
        ),
      ]),
      result,
    );
  }

  #[test]
  fn test_if() {
    let source = r"if(1==2){1;}";
    let input = source.chars().collect::<Vec<_>>();
    let result = r#if().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_if(
        Expr::of_binary(
          Operator::EqualEqual,
          Expr::of_integer_literal(1),
          Expr::of_integer_literal(2)
        ),
        Expr::of_block(vec![Expr::of_integer_literal(1)]),
        None
      ),
      result
    );
  }

  #[test]
  fn test_assignment() {
    let source = r"i=1;";
    let input = source.chars().collect::<Vec<_>>();
    let result = line().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_assignment("i".to_string(), Expr::of_integer_literal(1)),
      result
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
      Expr::of_labelled_call(
        "abc".to_string(),
        vec![LabelledParameter::new("n".to_string(), Expr::of_integer_literal(5))]
      ),
      result
    );
  }

  #[test]
  fn test_primary_function_call_args_0() {
    let source = r#"
    abc();
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = function_call().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_function_call("abc".to_string(), vec![]), result);
  }

  #[test]
  fn test_primary_function_call_args_1() {
    let source = r#"
    abc(1);
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = function_call().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::FunctionCall("abc".to_string(), vec![Expr::of_integer_literal(1)]),
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
      Expr::of_function_call(
        "abc".to_string(),
        vec![Expr::of_integer_literal(1), Expr::of_integer_literal(2)]
      ),
      result
    );
  }

  #[test]
  fn test_primary_bool_true() {
    let source = r"true";
    let input = source.chars().collect::<Vec<_>>();
    let result = bool_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_bool_literal(true), result);
  }

  #[test]
  fn test_primary_bool_false() {
    let source = r"false";
    let input = source.chars().collect::<Vec<_>>();
    let result = bool_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_bool_literal(false), result);
  }

  #[test]
  fn test_primary_bool_array_0() {
    let source = r"[]";
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_array_literal(vec![]), result);
  }

  #[test]
  fn test_primary_bool_array_1() {
    let source = r"[1]";
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_array_literal(vec![Expr::of_integer_literal(1)]), result);
  }

  #[test]
  fn test_primary_bool_array_2() {
    let source = r#"
    [1,2]
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = array_literal().parse_as_result(&input).unwrap();
    assert_eq!(
      Expr::of_array_literal(vec![Expr::of_integer_literal(1), Expr::of_integer_literal(2)]),
      result
    );
  }

  #[test]
  fn test_primary_integer() {
    let source = r#"
    10
    "#;
    let input = source.chars().collect::<Vec<_>>();
    let result = integer().parse_as_result(&input).unwrap();
    assert_eq!(Expr::of_integer_literal(10), result);
  }

  #[test]
  fn test_primary_identifier() {
    let source = r"abc";
    let input = source.chars().collect::<Vec<_>>();
    let result = identifier().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(Expr::of_symbol("abc".to_string()), result);
  }

  #[test]
  fn test_multitive() {
    let source = r"1/2";
    let input = source.chars().collect::<Vec<_>>();
    println!("start");

    let result = expression().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_binary(
        Operator::Divide,
        Expr::of_integer_literal(1),
        Expr::of_integer_literal(2)
      ),
      result
    );
  }

  #[test]
  fn test_moditive() {
    let source = r"2%2";
    let input = source.chars().collect::<Vec<_>>();
    let result = moditive().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_binary(Operator::Mod, Expr::of_integer_literal(2), Expr::of_integer_literal(2)),
      result
    );
  }

  #[test]
  fn test_additive() {
    let source = r"1+2";
    let input = source.chars().collect::<Vec<_>>();
    let result = additive().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_binary(Operator::Add, Expr::of_integer_literal(1), Expr::of_integer_literal(2)),
      result
    );
  }

  #[test]
  fn test_comparative() {
    let source = r"1>2";
    let input = source.chars().collect::<Vec<_>>();
    let result = expression().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_binary(
        Operator::GreaterThan,
        Expr::of_integer_literal(1),
        Expr::of_integer_literal(2)
      ),
      result
    );
  }

  #[test]
  fn test_comparative_symbol_number() {
    let source = r"a>2";
    let input = source.chars().collect::<Vec<_>>();
    let result = comparative().parse_as_result(&input).unwrap();
    println!("{:?}", result);
    assert_eq!(
      Expr::of_binary(
        Operator::GreaterThan,
        Expr::of_symbol("a".to_string()),
        Expr::of_integer_literal(2)
      ),
      result
    );
  }
}
