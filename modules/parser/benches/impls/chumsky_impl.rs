use chumsky::prelude::*;

pub fn parse_identifier(s: &str) -> Option<String> {
  let head = any::<_, extra::Default>().filter(|c: &char| c.is_ascii_alphabetic() || *c == '_');
  let tail = any::<_, extra::Default>()
    .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
    .repeated()
    .collect::<Vec<_>>();
  let ident = head.then(tail).map(|(h, t): (char, Vec<char>)| {
    let mut result = String::with_capacity(1 + t.len());
    result.push(h);
    for c in t {
      result.push(c);
    }
    result
  });
  // chumsky 0.12 の parse() は内部で end() を追加するため、残余入力を明示的に消費する
  ident.then_ignore(any().repeated()).parse(s).into_output()
}

pub fn parse_integer(s: &str) -> Option<u64> {
  let digits = any::<_, extra::Default>()
    .filter(|c: &char| c.is_ascii_digit())
    .repeated()
    .at_least(1)
    .collect::<String>();
  let parser = digits.map(|d| d.parse::<u64>().unwrap());
  // chumsky 0.12 の parse() は内部で end() を追加するため、残余入力を明示的に消費する
  parser.then_ignore(any().repeated()).parse(s).into_output()
}

/// flat_map 同一型分岐: digit → tag (chumsky 0.12 は then_with が廃止されたため choice を使用)
pub fn parse_flat_map_same_type(s: &str) -> Option<String> {
  let p1 = just::<_, &str, extra::Default>('1').ignore_then(just("one")).to("one".to_string());
  let p2 = just::<_, &str, extra::Default>('2').ignore_then(just("two")).to("two".to_string());
  let p3 = just::<_, &str, extra::Default>('3').ignore_then(just("three")).to("three".to_string());
  choice((p1, p2, p3)).parse(s).into_output()
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択
/// chumsky 0.12 は then_with が廃止されたため choice で実装
pub fn parse_flat_map_boxed(s: &str) -> Option<(char, String)> {
  let c_parser = just('c').then_ignore(just(':')).then(
    any::<_, extra::Default>()
      .filter(|c: &char| c.is_ascii_alphabetic())
      .repeated()
      .at_least(1)
      .collect::<String>(),
  );
  let i_parser = just('i').then_ignore(just(':')).then(
    any::<_, extra::Default>()
      .filter(|c: &char| c.is_ascii_digit())
      .repeated()
      .at_least(1)
      .collect::<String>(),
  );
  choice((c_parser, i_parser)).parse(s).into_output()
}
