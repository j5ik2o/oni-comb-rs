use chumsky::prelude::*;

pub fn parse_identifier(s: &str) -> Option<String> {
  let head = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphabetic() || *c == '_');
  let tail = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated();
  let ident = head.then(tail).map(|(h, t): (char, Vec<char>)| {
    let mut result = String::with_capacity(1 + t.len());
    result.push(h);
    for c in t {
      result.push(c);
    }
    result
  });
  ident.parse(s).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
  let digits = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
    .repeated()
    .at_least(1)
    .collect::<String>();
  let parser = digits.map(|d| d.parse::<u64>().unwrap());
  parser.parse(s).ok()
}

/// flat_map 同一型分岐: digit → tag (chumsky は then_with を使用)
pub fn parse_flat_map_same_type(s: &str) -> Option<String> {
  let digit = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit());
  let parser = digit.then_with(|c: char| {
    let expected: &str = match c {
      '1' => "one",
      '2' => "two",
      '3' => "three",
      _ => "",
    };
    just::<_, _, Simple<char>>(expected.chars().collect::<Vec<_>>()).map(|cs| cs.into_iter().collect::<String>())
  });
  parser.parse(s).ok()
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択
/// chumsky は元々動的ディスパッチなので、.boxed() で型を統一
pub fn parse_flat_map_boxed(s: &str) -> Option<(char, String)> {
  let head = filter::<_, _, Simple<char>>(|c: &char| *c == 'c' || *c == 'i');
  let parser = head.then_with(|t: char| {
    let colon = just::<_, _, Simple<char>>(':');
    let value = match t {
      'c' => filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphabetic())
        .repeated()
        .at_least(1)
        .boxed(),
      _ => filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .boxed(),
    };
    colon.then(value).map(|(c, v)| (c, v.into_iter().collect::<String>()))
  });
  parser.parse(s).ok()
}
