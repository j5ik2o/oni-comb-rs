use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

/// 関数ポインタを `Parser` としてラップする。
///
/// `recursive()` の `Box<dyn Parser>` + vtable を回避し、
/// 通常の関数呼び出しで再帰パーサーを構築できる。
///
/// ```ignore
/// fn my_value<'a>(input: &mut StrInput<'a>) -> PResult<Json, ParseError> {
///     // 先頭バイトで分岐 → 各コンビネータを呼ぶ
///     // 再帰は fn_parser(my_value) で自分自身を参照
/// }
/// let parser = fn_parser(my_value);
/// ```
pub struct FnParser<F> {
    f: F,
}

pub fn fn_parser<I, O, E, F>(f: F) -> FnParser<F>
where
    I: Input,
    F: FnMut(&mut I) -> PResult<O, E>,
{
    FnParser { f }
}

impl<I, O, E, F> Parser<I> for FnParser<F>
where
    I: Input,
    F: FnMut(&mut I) -> PResult<O, E>,
{
    type Output = O;
    type Error = E;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<O, E> {
        (self.f)(input)
    }
}
