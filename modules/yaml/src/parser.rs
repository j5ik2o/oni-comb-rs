use std::borrow::Cow;
use std::boxed::Box;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::value::YamlValue;

type BoxParser<'a, O> = Box<dyn Parser<StrInputStream<'a>, Output = O, Error = ParseError> + 'a>;

fn boxed<'a, O, P>(parser: P) -> BoxParser<'a, O>
where
  P: Parser<StrInputStream<'a>, Output = O, Error = ParseError> + 'a, {
  Box::new(parser)
}

fn classify_plain_scalar<'a>(text: &'a str) -> Result<YamlValue<'a>, ()> {
  let trimmed = strip_trailing_comment(text).trim_end();
  if trimmed.is_empty() {
    return Err(());
  }

  Ok(match trimmed {
    "null" => YamlValue::Null,
    "true" => YamlValue::Bool(true),
    "false" => YamlValue::Bool(false),
    _ => match trimmed.parse::<i64>() {
      Ok(value) if trimmed.chars().all(|c| c == '-' || c.is_ascii_digit()) => YamlValue::Integer(value),
      _ => YamlValue::String(Cow::Borrowed(trimmed)),
    },
  })
}

fn strip_trailing_comment(text: &str) -> &str {
  let mut previous = None;
  for (index, ch) in text.char_indices() {
    if ch == '#' && previous.is_some_and(|prev: char| prev == ' ') {
      return &text[..index];
    }
    previous = Some(ch);
  }
  text
}

fn line_start<'a>() -> impl Parser<StrInputStream<'a>, Output = (), Error = ParseError> {
  guard(|input: &StrInputStream<'_>| input.offset() == input.line_start())
}

fn exact_indent<'a>(indent: usize) -> BoxParser<'a, ()> {
  boxed(
    line_start()
      .zip_right(take_while0(|c: char| c == ' '))
      .flat_map(move |spaces: &'a str| boxed(guard(move |_input: &StrInputStream<'_>| spaces.len() == indent)))
      .discard()
      .context("expected indentation"),
  )
}

fn spaces0<'a>() -> impl Parser<StrInputStream<'a>, Output = &'a str, Error = ParseError> {
  take_while0(|c: char| c == ' ')
}

fn spaces1<'a>() -> impl Parser<StrInputStream<'a>, Output = &'a str, Error = ParseError> {
  take_while1(|c: char| c == ' ')
}

fn line_comment<'a>() -> impl Parser<StrInputStream<'a>, Output = (), Error = ParseError> {
  char('#').zip_right(take_till0(|c: char| c == '\n')).discard()
}

fn blank_line<'a>() -> BoxParser<'a, ()> {
  boxed(
    spaces0()
      .zip_right(line_comment().optional())
      .zip_left(char('\n'))
      .discard(),
  )
}

fn line_end_or_eof<'a>() -> BoxParser<'a, ()> {
  boxed(
    spaces0()
      .zip_right(line_comment().optional())
      .zip_right(char('\n').discard().or(eof()))
      .discard(),
  )
}

fn document_end<'a>() -> BoxParser<'a, ()> {
  boxed(
    blank_line().many0().zip_right(
      spaces0()
        .flat_map(move |spaces: &'a str| {
          let tail = line_comment().optional().zip_left(eof()).discard();
          if spaces.is_empty() {
            boxed(tail)
          } else {
            boxed(tail.context("expected indentation"))
          }
        })
        .discard(),
    ),
  )
}

fn leading_blank_lines<'a>() -> BoxParser<'a, ()> {
  boxed(blank_line().many0().discard())
}

fn single_quoted_scalar<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    between(char('\''), take_till0(|c: char| c == '\''), char('\''))
      .map(|value: &'a str| YamlValue::String(Cow::Borrowed(value))),
  )
}

fn double_quoted_scalar<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(quoted_string().map(YamlValue::String))
}

fn plain_key<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    take_till1(|c: char| matches!(c, ':' | '\n'))
      .map_res(classify_plain_scalar, "YAML mapping key")
      .context("YAML key"),
  )
}

fn not_flow_start<'a>() -> BoxParser<'a, ()> {
  boxed(seq("- ").not().zip_right(char('{').or(char('[')).not()))
}

fn block_plain_scalar<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    take_till1(|c: char| matches!(c, '\n'))
      .map_res(classify_plain_scalar, "YAML plain scalar")
      .context("YAML plain scalar"),
  )
}

fn flow_plain_scalar<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    take_till1(|c: char| matches!(c, ',' | ']' | '}' | '\n'))
      .map_res(classify_plain_scalar, "YAML flow plain scalar")
      .context("YAML flow plain scalar"),
  )
}

fn quoted_scalar<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(single_quoted_scalar().or(double_quoted_scalar()))
}

fn flow_key<'a>() -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    quoted_scalar()
      .or(
        take_till1(|c: char| matches!(c, ':' | ',' | ']' | '}' | '\n'))
          .map_res(classify_plain_scalar, "YAML flow key"),
      )
      .context("YAML key"),
  )
}

fn flow_separator<'a>() -> BoxParser<'a, ()> {
  boxed(spaces0().zip_right(char(',')).zip_right(spaces0()).discard())
}

fn flow_sequence_with<'a, P>(value: P) -> BoxParser<'a, YamlValue<'a>>
where
  P: Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> + Clone + 'a, {
  boxed(
    char('[')
      .zip_right(spaces0())
      .zip_right(value.sep_by0(flow_separator()))
      .zip_left(spaces0().zip_right(char(']').cut()))
      .map(YamlValue::Sequence)
      .context("YAML flow sequence"),
  )
}

fn flow_mapping_entry_with<'a, P>(value: P) -> BoxParser<'a, (YamlValue<'a>, YamlValue<'a>)>
where
  P: Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> + Clone + 'a, {
  boxed(
    flow_key()
      .zip_left(spaces0().zip_right(char(':')).zip_right(spaces0()).cut())
      .zip(value),
  )
}

fn flow_mapping_with<'a, P>(value: P) -> BoxParser<'a, YamlValue<'a>>
where
  P: Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> + Clone + 'a, {
  boxed(
    char('{')
      .zip_right(spaces0())
      .zip_right(flow_mapping_entry_with(value).sep_by0(flow_separator()))
      .zip_left(spaces0().zip_right(char('}').cut()))
      .map(YamlValue::Mapping)
      .context("YAML flow mapping"),
  )
}

fn flow_value<'a>() -> impl Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> + Clone {
  recursive(|value| {
    flow_mapping_with(value.clone())
      .or(flow_sequence_with(value))
      .or(quoted_scalar())
      .or(flow_plain_scalar())
      .context("YAML flow value")
  })
}

fn inline_value_with_context<'a>(context: &'static str) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    flow_mapping_with(flow_value())
      .or(flow_sequence_with(flow_value()))
      .or(quoted_scalar())
      .or(block_plain_scalar())
      .context(context),
  )
}

fn block_inline_value<'a>() -> BoxParser<'a, YamlValue<'a>> {
  inline_value_with_context("YAML inline value")
}

fn block_scalar_line_at<'a>(indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    exact_indent(indent)
      .zip_right(block_inline_value())
      .zip_left(line_end_or_eof()),
  )
}

fn block_sequence_item_at<'a>(indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    exact_indent(indent)
      .zip_right(seq("- "))
      .zip_right(block_inline_value().cut())
      .zip_left(line_end_or_eof())
      .context("YAML block sequence item"),
  )
}

fn block_sequence_at<'a>(indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    block_sequence_item_at(indent)
      .many1()
      .map(YamlValue::Sequence)
      .context("YAML block sequence"),
  )
}

fn nested_block_value_at<'a>(parent_indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    char('\n')
      .zip_right(take_while1(|c: char| c == ' ').peek())
      .flat_map(move |spaces: &'a str| {
        let child_indent = spaces.len();
        boxed(
          guard(move |_input: &StrInputStream<'_>| child_indent > parent_indent)
            .context("expected deeper indentation")
            .zip_right(block_value_at(child_indent)),
        )
      })
      .context("YAML nested block value"),
  )
}

fn block_mapping_entry_at<'a>(indent: usize) -> BoxParser<'a, (YamlValue<'a>, YamlValue<'a>)> {
  boxed(
    exact_indent(indent)
      .zip_right(not_flow_start())
      .zip_right(plain_key())
      .zip_left(spaces0().zip_right(char(':')))
      .flat_map(move |key| {
        let nested = spaces0().zip_left(char('\n').peek()).zip_right(nested_block_value_at(indent).cut());
        let inline = spaces1().zip_right(block_inline_value()).zip_left(line_end_or_eof());

        boxed(
          nested
            .or(inline)
            .cut()
            .map(move |value| (key.clone(), value))
            .context("YAML block mapping entry"),
        )
      }),
  )
}

fn block_mapping_at<'a>(indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    block_mapping_entry_at(indent)
      .many1()
      .map(YamlValue::Mapping)
      .context("YAML block mapping"),
  )
}

fn block_value_at<'a>(indent: usize) -> BoxParser<'a, YamlValue<'a>> {
  boxed(
    block_mapping_at(indent)
      .or(block_sequence_at(indent))
      .or(block_scalar_line_at(indent))
      .context("YAML value"),
  )
}

fn top_inline_value<'a>() -> BoxParser<'a, YamlValue<'a>> {
  inline_value_with_context("YAML value")
}

/// YAML value parser (does not require EOF).
pub fn yaml_value<'a>() -> impl Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> {
  leading_blank_lines().zip_right(block_mapping_at(0).or(block_sequence_at(0)).or(top_inline_value()))
}

/// Complete YAML parser (value + trailing comments/newlines + EOF).
pub fn yaml<'a>() -> impl Parser<StrInputStream<'a>, Output = YamlValue<'a>, Error = ParseError> {
  yaml_value().zip_left(document_end())
}

fn fail_to_error(error: oni_comb_parser::fail::Fail<ParseError>) -> ParseError {
  match error {
    oni_comb_parser::fail::Fail::Backtrack(err) | oni_comb_parser::fail::Fail::Cut(err) => err,
    oni_comb_parser::fail::Fail::Incomplete => ParseError::from_expected(0, Expected::Description("incomplete input")),
    oni_comb_parser::fail::Fail::ZeroProgress => ParseError::from_expected(0, Expected::Description("zero progress")),
  }
}

/// Parse a YAML document, returning the parsed value or an error.
pub fn parse(src: &str) -> Result<YamlValue<'_>, ParseError> {
  let mut input = StrInputStream::new(src);
  yaml()
    .parse_next(&mut input)
    .map_err(|error| fail_to_error(error).fill_location_from_src(src))
}

/// Parse a YAML value without requiring EOF.
pub fn parse_value(src: &str) -> Result<YamlValue<'_>, ParseError> {
  let mut input = StrInputStream::new(src);
  yaml_value()
    .parse_next(&mut input)
    .map_err(|error| fail_to_error(error).fill_location_from_src(src))
}
