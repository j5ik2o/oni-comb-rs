use pom::parser::*;

pub fn parse_identifier(s: &str) -> Option<String> {
  let input: Vec<char> = s.chars().collect();
  let head = is_a(|c: char| c.is_ascii_alphabetic() || c == '_');
  let tail = is_a(|c: char| c.is_ascii_alphanumeric() || c == '_').repeat(0..);
  let ident = (head + tail).map(|(h, t)| {
    let mut result = String::with_capacity(1 + t.len());
    result.push(h);
    for c in t {
      result.push(c);
    }
    result
  });
  ident.parse(&input).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
  let input: Vec<char> = s.chars().collect();
  let digits = is_a(|c: char| c.is_ascii_digit()).repeat(1..);
  let parser = digits.map(|d| d.iter().collect::<String>().parse::<u64>().unwrap());
  parser.parse(&input).ok()
}

/// pom 用のタグマッチヘルパー: 指定文字列と一致するか検証
fn pom_tag<'a>(expected: &'static str) -> pom::parser::Parser<'a, char, String> {
  let len = expected.len();
  pom::parser::take(len).convert(move |chars: &[char]| {
    let s: String = chars.iter().collect();
    if s == expected {
      Ok(s)
    } else {
      Err(format!("expected '{}', got '{}'", expected, s))
    }
  })
}

/// flat_map 同一型分岐: digit → tag (pom は bind を使用)
pub fn parse_flat_map_same_type(s: &str) -> Option<String> {
  let input: Vec<char> = s.chars().collect();
  let digit = is_a(|c: char| c.is_ascii_digit());
  let parser = digit
    >> (|c: char| match c {
      '1' => pom_tag("one"),
      '2' => pom_tag("two"),
      '3' => pom_tag("three"),
      _ => pom_tag(""),
    });
  parser.parse(&input).ok()
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択
/// pom は元々動的ディスパッチ
pub fn parse_flat_map_boxed(s: &str) -> Option<(char, String)> {
  let input: Vec<char> = s.chars().collect();
  let head = is_a(|c: char| c == 'c' || c == 'i');
  let parser = head
    >> (|t: char| {
      let colon = sym(':');
      let value = match t {
        'c' => is_a(|c: char| c.is_ascii_alphabetic()).repeat(1..),
        _ => is_a(|c: char| c.is_ascii_digit()).repeat(1..),
      };
      (colon + value).map(|(c, v)| (c, v.into_iter().collect::<String>()))
    });
  parser.parse(&input).ok()
}
