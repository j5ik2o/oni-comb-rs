#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::str_input::StrInput;
use oni_comb_parser::text::satisfy::Satisfy;
use oni_comb_parser::text::take_while::{TakeWhile0, TakeWhile1};

/// コンビネータ合成のみを測定するため、String 構築を行わない identifier パーサー。
/// Satisfy + TakeWhile0 の .then() で (char, &str) を返す。
fn parse_identifier_no_alloc(s: &str) -> Option<(char, &str)> {
    let head = Satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = TakeWhile0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut parser = head.then(tail);
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// TakeWhile1 は &str を返すのでアロケーション不要。
fn parse_integer_no_alloc(s: &str) -> Option<&str> {
    let mut parser = TakeWhile1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    let id_inputs = ["x", "foo", "foo_bar_123", "_private"];
    let int_inputs = ["0", "42", "9999999"];

    for input in id_inputs {
        let _ = parse_identifier_no_alloc(input);
    }
    for input in int_inputs {
        let _ = parse_integer_no_alloc(input);
    }
}
