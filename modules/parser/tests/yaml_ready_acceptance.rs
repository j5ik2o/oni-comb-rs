//! YAML-ready acceptance criteria for downstream grammars.
//!
//! This file fixes the phase-1 contract before the parser core redesign starts.
//! The top-level acceptance grammars in this file must stay declarative:
//! - do not call `parse_next` directly from top-level grammar definitions
//! - do not call `checkpoint` / `reset` directly from top-level grammar definitions
//! - do not discard parser results and branch on input state by hand in top-level grammar definitions
//! - do not fall back to imperative escape hatches such as `fn_parser`
//!
//! Small helper parsers and `InputStream` wrappers are allowed to encapsulate
//! downstream-owned stateful adaptation on top of the existing public contract.
//!
//! The ignored tests below are the executable contract for tasks 2.*-4.*.
//! They stay in `modules/parser/tests` so the readiness criteria live next to
//! other downstream grammar examples.

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;
use oni_comb_parser::primitive::seq::InputSeq;
use oni_comb_parser::str_input_stream::{StrCheckpoint, StrInputStream};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedOutcome {
  Accept,
  Reject {
    line: usize,
    column: usize,
    reason: &'static str,
  },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LitmusGrammarCase {
  id: &'static str,
  input: &'static str,
  outcome: ExpectedOutcome,
}

const LITMUS_GRAMMAR_CASES: [LitmusGrammarCase; 10] = [
  LitmusGrammarCase {
    id: "block list",
    input: "- milk\n- eggs\n- bread\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "indent nesting",
    input: "parent:\n  child:\n    grandchild: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "flow/block switching",
    input: "items: [one, two]\nmapping:\n  nested: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "multiline block",
    input: "note: |\n  line one\n  line two\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "block scalar header",
    input: "literal: |-\n  line one\n  line two\nfolded: >2\n    line three\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "document boundary",
    input: "---\nkey: value\n...\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "simple-key gating",
    input: "plain: value\n? explicit key\n: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "simple-key backtrack",
    input: "plain: value\n? explicit key\n: value\nflow: [one, two]\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "flow plain scalar boundary",
    input: "{key: plain, next: [one, two]}\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "indent error",
    input: "root:\n  child: ok\n next: wrong\n",
    outcome: ExpectedOutcome::Reject {
      line: 3,
      column: 2,
      reason: "indentation must match an active block context",
    },
  },
];

fn litmus_case(id: &'static str) -> LitmusGrammarCase {
  *LITMUS_GRAMMAR_CASES
    .iter()
    .find(|case| case.id == id)
    .unwrap_or_else(|| panic!("missing YAML-ready litmus grammar case: {id}"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct YamlState {
  indent_stack: [usize; 8],
  indent_len: u8,
  flow_level: u8,
  simple_key_allowed: bool,
}

impl Default for YamlState {
  fn default() -> Self {
    Self {
      indent_stack: [0; 8],
      indent_len: 0,
      flow_level: 0,
      simple_key_allowed: true,
    }
  }
}

impl YamlState {
  fn expected_indent(&self) -> usize {
    if self.indent_len == 0 {
      0
    } else {
      self.indent_stack[self.indent_len as usize - 1]
    }
  }

  fn push_indent(&mut self, indent: usize) {
    let index = self.indent_len as usize;
    self.indent_stack[index] = indent;
    self.indent_len += 1;
  }
}

#[derive(Clone, Copy, Debug)]
struct YamlCheckpoint {
  inner: StrCheckpoint,
  state: YamlState,
}

impl PartialEq for YamlCheckpoint {
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl Eq for YamlCheckpoint {}

impl PartialOrd for YamlCheckpoint {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for YamlCheckpoint {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.inner.cmp(&other.inner)
  }
}

struct YamlReadyInput<'a> {
  inner: StrInputStream<'a>,
  state: YamlState,
}

impl<'a> YamlReadyInput<'a> {
  fn new(src: &'a str) -> Self {
    Self {
      inner: StrInputStream::new(src),
      state: YamlState::default(),
    }
  }
}

impl<'a> InputStream for YamlReadyInput<'a> {
  type Token = char;
  type Slice = &'a str;
  type Checkpoint = YamlCheckpoint;
  type Error = ParseError;

  fn next_token(&mut self) -> Option<Self::Token> {
    self.inner.next_token()
  }

  fn peek_token(&self) -> Option<Self::Token> {
    self.inner.peek_token()
  }

  fn slice_since(&self, cp: Self::Checkpoint) -> Self::Slice {
    self.inner.slice_since(cp.inner)
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    YamlCheckpoint {
      inner: self.inner.checkpoint(),
      state: self.state,
    }
  }

  fn reset(&mut self, checkpoint: Self::Checkpoint) {
    self.inner.reset(checkpoint.inner);
    self.state = checkpoint.state;
  }

  fn offset(&self) -> usize {
    self.inner.offset()
  }

  fn remaining(&self) -> Self::Slice {
    self.inner.remaining()
  }

  fn is_eof(&self) -> bool {
    self.inner.is_eof()
  }

  fn line(&self) -> usize {
    self.inner.line()
  }

  fn column(&self) -> usize {
    self.inner.column()
  }

  fn line_start(&self) -> usize {
    self.inner.line_start()
  }

  fn position_after(&self, consumed: usize) -> oni_comb_parser::input_position::InputPosition {
    self.inner.position_after(consumed)
  }
}

impl<'a> InputSeq<'a, str> for YamlReadyInput<'a> {
  fn starts_with(&self, tag: &str) -> bool {
    self.remaining().starts_with(tag)
  }

  fn advance_by(&mut self, tag: &str) {
    self.inner.advance(tag.len());
  }

  fn tag_to_expected(tag: &'static str) -> Expected {
    Expected::Tag(tag)
  }
}

#[derive(Clone)]
struct WithExpectedIndent<P> {
  indent: usize,
  parser: P,
}

fn with_expected_indent<P>(indent: usize, parser: P) -> WithExpectedIndent<P> {
  WithExpectedIndent { indent, parser }
}

impl<'a, P> Parser<YamlReadyInput<'a>> for WithExpectedIndent<P>
where
  P: Parser<YamlReadyInput<'a>, Error = ParseError>,
{
  type Output = P::Output;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlReadyInput<'a>) -> Result<Self::Output, Fail<Self::Error>> {
    let saved_stack = input.state;
    input.state.push_indent(self.indent);
    let result = self.parser.parse_next(input);
    input.state.indent_stack = saved_stack.indent_stack;
    input.state.indent_len = saved_stack.indent_len;
    result
  }
}

#[derive(Clone)]
struct WithFlow<P> {
  parser: P,
}

fn with_flow<P>(parser: P) -> WithFlow<P> {
  WithFlow { parser }
}

impl<'a, P> Parser<YamlReadyInput<'a>> for WithFlow<P>
where
  P: Parser<YamlReadyInput<'a>, Error = ParseError>,
{
  type Output = P::Output;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlReadyInput<'a>) -> Result<Self::Output, Fail<Self::Error>> {
    let saved_flow = input.state.flow_level;
    input.state.flow_level += 1;
    let result = self.parser.parse_next(input);
    input.state.flow_level = saved_flow;
    result
  }
}

#[derive(Clone)]
struct WithSimpleKeyAllowed<P> {
  value: bool,
  parser: P,
}

fn with_simple_key_allowed<P>(value: bool, parser: P) -> WithSimpleKeyAllowed<P> {
  WithSimpleKeyAllowed { value, parser }
}

impl<'a, P> Parser<YamlReadyInput<'a>> for WithSimpleKeyAllowed<P>
where
  P: Parser<YamlReadyInput<'a>, Error = ParseError>,
{
  type Output = P::Output;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlReadyInput<'a>) -> Result<Self::Output, Fail<Self::Error>> {
    let saved = input.state.simple_key_allowed;
    input.state.simple_key_allowed = self.value;
    let result = self.parser.parse_next(input);
    input.state.simple_key_allowed = saved;
    result
  }
}

#[derive(Clone, Copy)]
struct ActiveIndent {
  reason: &'static str,
}

fn active_indent(reason: &'static str) -> ActiveIndent {
  ActiveIndent { reason }
}

impl<'a> Parser<YamlReadyInput<'a>> for ActiveIndent {
  type Output = ();
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlReadyInput<'a>) -> Result<(), Fail<ParseError>> {
    let cp = input.checkpoint();
    while matches!(input.peek_token(), Some(' ')) {
      input.next_token();
    }
    let actual = input.column() - 1;
    let expected = input.state.expected_indent();
    if actual == expected {
      Ok(())
    } else {
      let error = ParseError::from_position(input.position(), Expected::Description(self.reason));
      input.reset(cp);
      Err(Fail::Backtrack(error))
    }
  }
}

#[derive(Clone, Copy)]
struct FlowPlainScalar;

fn flow_plain_scalar() -> FlowPlainScalar {
  FlowPlainScalar
}

impl<'a> Parser<YamlReadyInput<'a>> for FlowPlainScalar {
  type Output = &'a str;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlReadyInput<'a>) -> Result<&'a str, Fail<ParseError>> {
    let cp = input.checkpoint();
    let start = input.offset();
    while let Some(ch) = input.peek_token() {
      let is_boundary = if input.state.flow_level > 0 {
        matches!(ch, ',' | ']' | '}' | ':' | '\n')
      } else {
        matches!(ch, '\n')
      };
      if is_boundary {
        break;
      }
      input.next_token();
    }

    if input.offset() == start {
      Err(Fail::Backtrack(ParseError::from_position(
        input.position(),
        Expected::Description("flow plain scalar"),
      )))
    } else {
      Ok(input.slice_since(cp).trim_end())
    }
  }
}

fn newline<'a>() -> impl Parser<YamlReadyInput<'a>, Output = char, Error = ParseError> {
  sym('\n')
}

fn line_start<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  guard(|input: &YamlReadyInput<'_>| input.offset() == input.line_start())
}

fn simple_key_allowed<'a>(value: bool) -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  guard(move |input: &YamlReadyInput<'_>| input.state.simple_key_allowed == value)
}

fn word<'a>() -> impl Parser<YamlReadyInput<'a>, Output = &'a str, Error = ParseError> {
  take_while1(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
}

fn rest_of_line<'a>() -> impl Parser<YamlReadyInput<'a>, Output = &'a str, Error = ParseError> {
  take_till1(|c: char| c == '\n')
}

fn plain_mapping_line<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  line_start()
    .zip_right(simple_key_allowed(true))
    .zip_right(word())
    .zip_left(seq(": "))
    .zip_right(rest_of_line())
    .zip_left(newline())
    .discard()
}

fn explicit_key_value<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  line_start()
    .zip_right(seq("? "))
    .zip_right(rest_of_line())
    .zip_left(newline())
    .zip_right(line_start())
    .zip_right(seq(": "))
    .zip_right(rest_of_line())
    .zip_left(newline())
    .discard()
}

fn list_item<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  line_start()
    .zip_right(seq("- "))
    .zip_right(rest_of_line())
    .zip_left(newline())
    .discard()
}

fn indented_mapping_line<'a>(reason: &'static str) -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  active_indent(reason)
    .zip_right(word())
    .zip_left(seq(": "))
    .zip_right(rest_of_line())
    .zip_left(newline())
    .discard()
}

fn indented_key_only_line<'a>(reason: &'static str) -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  active_indent(reason)
    .zip_right(word())
    .zip_left(seq(":\n"))
    .discard()
}

fn indented_block_line<'a>(reason: &'static str) -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  active_indent(reason)
    .zip_right(rest_of_line())
    .zip_left(newline())
    .discard()
}

fn flow_sequence<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  sym('[')
    .zip_right(with_flow(flow_plain_scalar().sep_by0(seq(", "))))
    .zip_left(sym(']'))
    .discard()
}

fn flow_value<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  flow_sequence().or(flow_plain_scalar().discard())
}

fn flow_entry<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  flow_plain_scalar()
    .zip_left(seq(": "))
    .zip(flow_value())
    .discard()
}

fn flow_mapping<'a>() -> impl Parser<YamlReadyInput<'a>, Output = (), Error = ParseError> {
  sym('{')
    .zip_right(with_flow(flow_entry().sep_by1(seq(", "))))
    .zip_left(sym('}'))
    .discard()
}

fn run_acceptance_case<'a, P>(case: LitmusGrammarCase, mut parser: P)
where
  P: Parser<YamlReadyInput<'a>, Output = (), Error = ParseError>,
{
  let mut input = YamlReadyInput::new(case.input);
  match (case.outcome, parser.parse_next(&mut input)) {
    (ExpectedOutcome::Accept, Ok(())) => {}
    (ExpectedOutcome::Accept, Err(err)) => panic!("expected accept for {}, got {:?}", case.id, err),
    (ExpectedOutcome::Reject { line, column, reason }, Err(Fail::Backtrack(err) | Fail::Cut(err))) => {
      assert_eq!(err.line, line, "unexpected line for {}", case.id);
      assert_eq!(err.column, column, "unexpected column for {}", case.id);
      assert!(
        err.expected.contains(&Expected::Description(reason)),
        "unexpected reason for {}: {:?}",
        case.id,
        err.expected
      );
    }
    (ExpectedOutcome::Reject { .. }, Ok(())) => panic!("expected reject for {}, got success", case.id),
    (_, Err(Fail::Incomplete)) => panic!("unexpected incomplete for {}", case.id),
    (_, Err(Fail::ZeroProgress)) => panic!("unexpected zero progress for {}", case.id),
  }
}

#[test]
fn yaml_ready_litmus_grammar_catalog_is_fixed() {
  let actual_ids: [&str; 10] = core::array::from_fn(|index| LITMUS_GRAMMAR_CASES[index].id);
  assert_eq!(
    actual_ids,
    [
      "block list",
      "indent nesting",
      "flow/block switching",
      "multiline block",
      "block scalar header",
      "document boundary",
      "simple-key gating",
      "simple-key backtrack",
      "flow plain scalar boundary",
      "indent error",
    ]
  );
}

#[test]
fn yaml_ready_litmus_grammar_examples_are_fixed() {
  let accept_count = LITMUS_GRAMMAR_CASES
    .iter()
    .filter(|case| matches!(case.outcome, ExpectedOutcome::Accept))
    .count();
  let reject_count = LITMUS_GRAMMAR_CASES.len() - accept_count;

  assert_eq!(accept_count, 9);
  assert_eq!(reject_count, 1);

  let indent_error = litmus_case("indent error");
  assert_eq!(
    indent_error.outcome,
    ExpectedOutcome::Reject {
      line: 3,
      column: 2,
      reason: "indentation must match an active block context",
    }
  );
}

#[test]
fn block_list_acceptance_contract() {
  let parser = list_item().many1().zip_left(eof()).discard();
  run_acceptance_case(litmus_case("block list"), parser);
}

#[test]
fn indent_nesting_acceptance_contract() {
  let reason = "indentation must match an active block context";
  let parser = line_start()
    .zip_right(word())
    .zip_left(seq(":\n"))
    .zip_right(with_expected_indent(
      2,
      indented_key_only_line(reason).zip_right(with_expected_indent(4, indented_mapping_line(reason))),
    ))
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("indent nesting"), parser);
}

#[test]
fn flow_block_switching_acceptance_contract() {
  let reason = "indentation must match an active block context";
  let parser = line_start()
    .zip_right(seq("items: "))
    .zip_right(flow_sequence())
    .zip_left(newline())
    .zip_right(
      line_start()
        .zip_right(seq("mapping:\n"))
        .zip_right(with_expected_indent(2, indented_mapping_line(reason))),
    )
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("flow/block switching"), parser);
}

#[test]
fn multiline_block_acceptance_contract() {
  let reason = "indentation must match an active block context";
  let parser = line_start()
    .zip_right(seq("note: |\n"))
    .zip_right(with_expected_indent(2, indented_block_line(reason).many1()))
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("multiline block"), parser);
}

#[test]
fn block_scalar_header_acceptance_contract() {
  let reason = "indentation must match an active block context";
  let literal = line_start()
    .zip_right(seq("literal: |-\n"))
    .zip_right(with_expected_indent(2, indented_block_line(reason).many1()))
    .discard();
  let folded = line_start()
    .zip_right(seq("folded: >2\n"))
    .zip_right(with_expected_indent(4, indented_block_line(reason)))
    .discard();
  let parser = literal.zip_right(folded).zip_left(eof()).discard();
  run_acceptance_case(litmus_case("block scalar header"), parser);
}

#[test]
fn document_boundary_acceptance_contract() {
  let parser = line_start()
    .zip_right(seq("---\n"))
    .zip_right(plain_mapping_line())
    .zip_right(line_start().zip_right(seq("...\n")).discard())
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("document boundary"), parser);
}

#[test]
fn simple_key_gating_acceptance_contract() {
  let parser = with_simple_key_allowed(true, plain_mapping_line())
    .zip_right(explicit_key_value())
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("simple-key gating"), parser);
}

#[test]
fn simple_key_backtrack_acceptance_contract() {
  let parser = with_simple_key_allowed(false, plain_mapping_line())
    .attempt()
    .or(plain_mapping_line())
    .zip_right(explicit_key_value())
    .zip_right(
      line_start()
        .zip_right(seq("flow: "))
        .zip_right(flow_sequence())
        .zip_left(newline())
        .discard(),
    )
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("simple-key backtrack"), parser);
}

#[test]
fn flow_plain_scalar_boundary_acceptance_contract() {
  let parser = line_start()
    .zip_right(flow_mapping())
    .zip_left(newline())
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("flow plain scalar boundary"), parser);
}

#[test]
fn indent_error_acceptance_contract() {
  let reason = "indentation must match an active block context";
  let parser = line_start()
    .zip_right(seq("root:\n"))
    .zip_right(with_expected_indent(
      2,
      indented_mapping_line(reason).zip_right(indented_mapping_line(reason)),
    ))
    .zip_left(eof())
    .discard();
  run_acceptance_case(litmus_case("indent error"), parser);
}
