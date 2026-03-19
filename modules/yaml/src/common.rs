use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::PResult;
use oni_comb_parser::input::Input;
use oni_comb_parser::prelude::*;

/// Skip inline whitespace (spaces and tabs, not newlines).
#[inline]
pub(crate) fn skip_inline_ws<'a>(input: &mut StrInput<'a>) -> PResult<(), ParseError> {
  take_while0(|c: char| c == ' ' || c == '\t')
    .parse_next(input)
    .map(|_| ())
}

/// Skip a comment: '#' followed by everything until end of line.
#[inline]
pub(crate) fn skip_comment<'a>(input: &mut StrInput<'a>) -> PResult<(), ParseError> {
  if input.peek_byte() == Some(b'#') {
    take_while0(|c: char| c != '\n').parse_next(input).map(|_| ())?;
  }
  Ok(())
}

/// Skip inline whitespace, optional comment, and optional newline.
pub(crate) fn skip_ws_and_comments<'a>(input: &mut StrInput<'a>) -> PResult<(), ParseError> {
  loop {
    skip_inline_ws(input)?;
    skip_comment(input)?;
    if input.peek_byte() == Some(b'\n') {
      input.next_token();
    } else if input.peek_byte() == Some(b'\r') {
      input.next_token();
      if input.peek_byte() == Some(b'\n') {
        input.next_token();
      }
    } else {
      break;
    }
  }
  skip_inline_ws(input)?;
  Ok(())
}

/// Get the current indentation level (column - 1 for 0-based).
///
/// **Important:** This function only returns a meaningful indentation value
/// when the input position is at the start of a line's content (i.e., after
/// leading whitespace). Callers must ensure this precondition — typically by
/// calling `skip_ws_and_comments` before invoking this function. Mid-line
/// calls will return the column offset, not the line's indentation level.
#[inline]
pub(crate) fn current_indent<'a>(input: &StrInput<'a>) -> usize {
  input.column() - 1
}
