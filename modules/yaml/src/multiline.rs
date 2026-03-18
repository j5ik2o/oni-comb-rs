use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::prelude::*;

use crate::value::YamlValue;

/// Parse a block scalar (literal `|` or folded `>`).
pub(crate) fn block_scalar<'a>(input: &mut StrInput<'a>) -> PResult<YamlValue, ParseError> {
  let style = input.next_token().ok_or_else(|| {
    Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("| or >"),
    ))
  })?;

  // Parse optional chomping indicator
  let chomp = match input.peek_byte() {
    Some(b'-') => {
      input.next_token();
      Chomp::Strip
    }
    Some(b'+') => {
      input.next_token();
      Chomp::Keep
    }
    _ => Chomp::Clip,
  };

  // Skip rest of indicator line
  while input.peek_byte().is_some() && input.peek_byte() != Some(b'\n') {
    input.next_token();
  }
  // Consume the newline
  if input.peek_byte() == Some(b'\n') {
    input.next_token();
  }

  // Detect indent from first content line
  let mut content_indent = 0;
  let remaining = input.remaining();
  for c in remaining.chars() {
    if c == ' ' {
      content_indent += 1;
    } else {
      break;
    }
  }

  if content_indent == 0 {
    // Empty block scalar
    return Ok(YamlValue::String(String::new()));
  }

  // Collect lines at the detected indent level
  let mut result = String::new();
  let mut trailing_newlines = 0;

  loop {
    if input.is_eof() {
      break;
    }

    let remaining = input.remaining();

    // Check if this line has enough indent
    let line_indent = remaining.chars().take_while(|&c| c == ' ').count();

    // Empty line
    if remaining.starts_with('\n') {
      input.next_token();
      trailing_newlines += 1;
      continue;
    }

    if line_indent < content_indent {
      break;
    }

    // Flush trailing newlines
    for _ in 0..trailing_newlines {
      result.push('\n');
    }
    trailing_newlines = 0;

    // Skip indent
    for _ in 0..content_indent {
      input.next_token();
    }

    // Read line content
    let _line_start = result.len();
    while input.peek_byte().is_some() && input.peek_byte() != Some(b'\n') {
      if let Some(c) = input.next_token() {
        result.push(c);
      }
    }

    // Consume newline
    if input.peek_byte() == Some(b'\n') {
      input.next_token();
    }

    match style {
      '|' => result.push('\n'),
      '>' => {
        // Folded: newline becomes space, but blank lines become newlines
        result.push('\n');
      }
      _ => {}
    }
  }

  // Apply folding for '>' style
  if style == '>' {
    let mut folded = String::with_capacity(result.len());
    let lines: Vec<&str> = result.lines().collect();
    for (i, line) in lines.iter().enumerate() {
      if line.is_empty() {
        folded.push('\n');
      } else {
        if i > 0 && !lines[i - 1].is_empty() {
          folded.push(' ');
        }
        folded.push_str(line);
      }
    }
    result = folded;
    result.push('\n');
  }

  // Apply chomping
  match chomp {
    Chomp::Strip => {
      while result.ends_with('\n') {
        result.pop();
      }
    }
    Chomp::Clip => {
      while result.ends_with("\n\n") {
        result.pop();
      }
    }
    Chomp::Keep => {
      // Keep all trailing newlines
      for _ in 0..trailing_newlines {
        result.push('\n');
      }
    }
  }

  Ok(YamlValue::String(result))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Chomp {
  Strip, // '-': no trailing newline
  Clip,  // default: single trailing newline
  Keep,  // '+': keep all trailing newlines
}
