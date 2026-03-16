use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

/// 汎用エスケープ文字列パーサー。
///
/// - `open` / `close`: 囲みの開始・終了文字
/// - `escape`: エスケープ文字（通常 `\`）
/// - `escape_handler`: エスケープ文字の次の 1 文字を受け取り、変換後の文字を返す。
///   `None` を返すと不正なエスケープとして `Fail::Cut` になる。
///
/// ```ignore
/// // シングルクォート文字列、\' と \\ のみ対応
/// let sq = escaped('\'', '\'', '\\', |c| match c {
///     '\'' => Some('\''),
///     '\\' => Some('\\'),
///     _ => None,
/// });
/// ```
pub struct Escaped<F> {
    open: char,
    close: char,
    escape: char,
    handler: F,
}

pub fn escaped<F>(open: char, close: char, escape: char, handler: F) -> Escaped<F>
where
    F: FnMut(char) -> Option<char>,
{
    Escaped {
        open,
        close,
        escape,
        handler,
    }
}

impl<'a, F> Parser<StrInput<'a>> for Escaped<F>
where
    F: FnMut(char) -> Option<char>,
{
    type Output = String;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<String, String> {
        let remaining = input.as_str();
        let mut chars = remaining.chars();

        // opening delimiter
        match chars.next() {
            Some(c) if c == self.open => {}
            _ => {
                return Err(Fail::Backtrack(format!(
                    "escaped: expected '{}'",
                    self.open
                )));
            }
        }

        let mut result = String::new();
        let mut consumed = self.open.len_utf8();

        loop {
            match chars.next() {
                Some(c) if c == self.close => {
                    consumed += c.len_utf8();
                    input.advance(consumed);
                    return Ok(result);
                }
                Some(c) if c == self.escape => {
                    consumed += c.len_utf8();
                    match chars.next() {
                        Some(next) => {
                            consumed += next.len_utf8();
                            match (self.handler)(next) {
                                Some(replacement) => result.push(replacement),
                                None => {
                                    return Err(Fail::Cut(format!(
                                        "escaped: invalid escape sequence '{}{}' ",
                                        self.escape, next
                                    )));
                                }
                            }
                        }
                        None => {
                            return Err(Fail::Cut(format!(
                                "escaped: unexpected EOF after '{}'",
                                self.escape
                            )));
                        }
                    }
                }
                Some(c) => {
                    consumed += c.len_utf8();
                    result.push(c);
                }
                None => {
                    return Err(Fail::Cut(format!(
                        "escaped: unterminated, expected '{}'",
                        self.close
                    )));
                }
            }
        }
    }
}
