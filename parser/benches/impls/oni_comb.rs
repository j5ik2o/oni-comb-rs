use oni_comb_parser::prelude::*;

pub fn parse_identifier(s: &str) -> Option<String> {
    let head = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut parser = head.zip(tail).map(|(h, t): (char, &str)| {
        let mut result = String::with_capacity(1 + t.len());
        result.push(h);
        result.push_str(t);
        result
    });
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
    let mut parser =
        take_while1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u64>().unwrap());
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// flat_map 同一型分岐: digit を読み、結果に応じた tag を返す (Box 不要)
pub fn parse_flat_map_same_type(s: &str) -> Option<&str> {
    let mut parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|c| match c {
        '1' => tag("one"),
        '2' => tag("two"),
        '3' => tag("three"),
        _ => tag(""),
    });
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択 (Box<dyn Parser> 必要)
pub fn parse_flat_map_boxed(s: &str) -> Option<(&str, &str)> {
    let mut parser = satisfy(|c: char| c == 'c' || c == 'i').flat_map(
        |t| -> Box<
            dyn Parser<
                oni_comb_parser::str_input::StrInput<'_>,
                Output = (&str, &str),
                Error = String,
            >,
        > {
            match t {
                'c' => Box::new(
                    tag(":").zip(take_while1(|c: char| c.is_ascii_alphabetic())),
                ),
                _ => Box::new(tag(":").zip(take_while1(|c: char| c.is_ascii_digit()))),
            }
        },
    );
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// zip vs flat_map 比較用: zip 版 (String 構築なし)
pub fn parse_identifier_zip(s: &str) -> Option<(char, &str)> {
    let mut parser = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
        .zip(take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'));
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// zip vs flat_map 比較用: flat_map 版 (String 構築なし)
pub fn parse_identifier_flat_map(s: &str) -> Option<&str> {
    let mut parser = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_')
        .flat_map(|_| take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_'));
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}
