use oni_comb_parser::error::{ExpectError, Expected, MinimalError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input_position::InputPosition;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TestCheckpoint {
  offset: usize,
  state: u8,
}

#[derive(Clone, Debug)]
struct TestInput<'a> {
  src: &'a [u8],
  offset: usize,
  state: u8,
}

impl<'a> TestInput<'a> {
  fn new(src: &'a [u8]) -> Self {
    Self {
      src,
      offset: 0,
      state: 0,
    }
  }
}

impl<'a> InputStream for TestInput<'a> {
  type Checkpoint = TestCheckpoint;
  type Error = MinimalError;
  type Slice = &'a [u8];
  type Token = u8;

  fn next_token(&mut self) -> Option<Self::Token> {
    let token = self.peek_token()?;
    self.offset += 1;
    Some(token)
  }

  fn peek_token(&self) -> Option<Self::Token> {
    self.src.get(self.offset).copied()
  }

  fn slice_since(&self, cp: Self::Checkpoint) -> Self::Slice {
    &self.src[cp.offset..self.offset]
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    TestCheckpoint {
      offset: self.offset,
      state: self.state,
    }
  }

  fn reset(&mut self, checkpoint: Self::Checkpoint) {
    self.offset = checkpoint.offset;
    self.state = checkpoint.state;
  }

  fn offset(&self) -> usize {
    self.offset
  }

  fn remaining(&self) -> Self::Slice {
    &self.src[self.offset..]
  }

  fn is_eof(&self) -> bool {
    self.offset >= self.src.len()
  }

  fn line(&self) -> usize {
    1
  }

  fn column(&self) -> usize {
    self.offset + 1
  }

  fn line_start(&self) -> usize {
    0
  }

  fn position_after(&self, consumed: usize) -> InputPosition {
    InputPosition::new(self.offset + consumed, 1, self.offset + consumed + 1, 0)
  }
}

struct MutateThenBacktrack {
  state: u8,
}

impl Parser<TestInput<'_>> for MutateThenBacktrack {
  type Error = MinimalError;
  type Output = u8;

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<u8, Self::Error> {
    input.state = self.state;
    Err(Fail::Backtrack(MinimalError::from_position(
      input.position(),
      Expected::Description("forced backtrack"),
    )))
  }
}

struct MutateThenCut {
  state: u8,
}

impl Parser<TestInput<'_>> for MutateThenCut {
  type Error = MinimalError;
  type Output = ();

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<Self::Output, Self::Error> {
    input.state = self.state;
    Err(Fail::Cut(MinimalError::from_position(
      input.position(),
      Expected::Description("forced cut"),
    )))
  }
}

struct AssertStateAndConsume {
  expected_state: u8,
}

impl Parser<TestInput<'_>> for AssertStateAndConsume {
  type Error = MinimalError;
  type Output = u8;

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<Self::Output, Self::Error> {
    assert_eq!(input.state, self.expected_state);
    input
      .next_token()
      .ok_or_else(|| Fail::Backtrack(MinimalError::from_position(input.position(), Expected::Eof)))
  }
}

struct StatefulManyA;

impl Parser<TestInput<'_>> for StatefulManyA {
  type Error = MinimalError;
  type Output = u8;

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<Self::Output, Self::Error> {
    input.state = input.state.saturating_add(1);
    match input.peek_token() {
      Some(b'a') => {
        input.next_token();
        Ok(b'a')
      }
      _ => Err(Fail::Backtrack(MinimalError::from_position(
        input.position(),
        Expected::Byte(b'a'),
      ))),
    }
  }
}

struct StatefulItem;

impl Parser<TestInput<'_>> for StatefulItem {
  type Error = MinimalError;
  type Output = u8;

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<Self::Output, Self::Error> {
    input.state = input.state.saturating_add(1);
    match input.peek_token() {
      Some(b'a') => {
        input.next_token();
        Ok(b'a')
      }
      _ => Err(Fail::Backtrack(MinimalError::from_position(
        input.position(),
        Expected::Byte(b'a'),
      ))),
    }
  }
}

struct StatefulComma;

impl Parser<TestInput<'_>> for StatefulComma {
  type Error = MinimalError;
  type Output = u8;

  fn parse_next(&mut self, input: &mut TestInput<'_>) -> PResult<Self::Output, Self::Error> {
    input.state = input.state.saturating_add(10);
    match input.peek_token() {
      Some(b',') => {
        input.next_token();
        Ok(b',')
      }
      _ => Err(Fail::Backtrack(MinimalError::from_position(
        input.position(),
        Expected::Byte(b','),
      ))),
    }
  }
}

#[test]
fn or_rewinds_checkpointed_state_before_right_branch() {
  let mut input = TestInput::new(b"x");
  let mut parser = MutateThenBacktrack { state: 7 }.or(AssertStateAndConsume { expected_state: 0 });

  assert_eq!(parser.parse_next(&mut input).unwrap(), b'x');
  assert_eq!(input.state, 0);
}

#[test]
fn attempt_rewinds_checkpointed_state() {
  let mut input = TestInput::new(b"");
  let mut parser = MutateThenCut { state: 9 }.attempt();

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(_)) => {}
    other => panic!("expected backtrack, got {:?}", other),
  }

  assert_eq!(input.state, 0);
}

#[test]
fn optional_rewinds_checkpointed_state() {
  let mut input = TestInput::new(b"");
  let mut parser = MutateThenBacktrack { state: 5 }.optional();

  assert_eq!(parser.parse_next(&mut input).unwrap(), None);
  assert_eq!(input.state, 0);
}

#[test]
fn many0_rewinds_state_after_terminal_backtrack() {
  let mut input = TestInput::new(b"aa!");
  let mut parser = StatefulManyA.many0();

  assert_eq!(parser.parse_next(&mut input).unwrap(), vec![b'a', b'a']);
  assert_eq!(input.state, 2);
  assert_eq!(input.remaining(), b"!");
}

#[test]
fn many1_rewinds_state_after_terminal_backtrack() {
  let mut input = TestInput::new(b"aa!");
  let mut parser = StatefulManyA.many1();

  assert_eq!(parser.parse_next(&mut input).unwrap(), vec![b'a', b'a']);
  assert_eq!(input.state, 2);
  assert_eq!(input.remaining(), b"!");
}

#[test]
fn sep_by0_rewinds_state_after_failed_separator() {
  let mut input = TestInput::new(b"a,a!");
  let mut parser = StatefulItem.sep_by0(StatefulComma);

  assert_eq!(parser.parse_next(&mut input).unwrap(), vec![b'a', b'a']);
  assert_eq!(input.state, 12);
  assert_eq!(input.remaining(), b"!");
}

#[test]
fn sep_by1_rewinds_state_after_failed_separator() {
  let mut input = TestInput::new(b"a,a!");
  let mut parser = StatefulItem.sep_by1(StatefulComma);

  assert_eq!(parser.parse_next(&mut input).unwrap(), vec![b'a', b'a']);
  assert_eq!(input.state, 12);
  assert_eq!(input.remaining(), b"!");
}
