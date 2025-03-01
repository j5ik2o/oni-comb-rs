use crate::core::{CommittedStatus, ParseError, ParseResult, ParseState, StaticParser};
use crate::internal::ParsersImpl;
use std::fmt::{Debug, Display};

/// StaticParserの実装を提供する構造体
pub struct StaticParsersImpl;

impl StaticParsersImpl {
  /// 何もしないStaticParserを返します。
  pub fn unit<'a, I>() -> StaticParser<'a, I, ()> {
    StaticParser::new(move |_| ParseResult::successful((), 0))
  }

  /// 何もしないStaticParserを返します。unit()のエイリアスです。
  pub fn empty<'a, I>() -> StaticParser<'a, I, ()> {
    Self::unit()
  }

  /// 終端を表すStaticParserを返します。
  pub fn end<'a, I>() -> StaticParser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();
      if offset >= input.len() {
        ParseResult::successful((), 0)
      } else {
        let msg = format!("expected end of input, but got: {}", input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 成功した解析結果を表すStaticParserを返します。
  pub fn successful<'a, I, A>(value: A) -> StaticParser<'a, I, A>
  where
    A: Clone + 'a, {
    StaticParser::new(move |_| ParseResult::successful(value.clone(), 0))
  }

  /// 成功した解析結果を表すStaticParserを返します。
  pub fn successful_lazy<'a, I, A, F>(value: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a, {
    StaticParser::new(move |_| ParseResult::successful(value(), 0))
  }

  /// 失敗した解析結果を表すStaticParserを返します。
  pub fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> StaticParser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    StaticParser::new(move |_| ParseResult::failed(value.clone(), committed.clone()))
  }

  /// 失敗した解析結果を表すStaticParserを返します。
  pub fn failed_lazy<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    StaticParser::new(move |_| {
      let (pe, committed) = f();
      ParseResult::failed(pe, committed)
    })
  }

  /// 任意の要素を解析するStaticParserを返します。(参照版)
  pub fn elm_any_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: crate::core::Element + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        ParseResult::successful(&input[offset], 1)
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 任意の要素を解析するStaticParserを返します。
  pub fn elm_any<'a, I>() -> StaticParser<'a, I, I>
  where
    I: crate::core::Element + Clone + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        ParseResult::successful(input[offset].clone(), 1)
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 指定した要素を解析するStaticParserを返します。(参照版)
  pub fn elm_ref<'a, I>(element: I) -> StaticParser<'a, I, &'a I>
  where
    I: crate::core::Element + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && input[offset] == element {
        ParseResult::successful(&input[offset], 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("expected: {}, but got: {}", element, input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 指定した要素を解析するStaticParserを返します。
  pub fn elm<'a, I>(element: I) -> StaticParser<'a, I, I>
  where
    I: crate::core::Element + Clone + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && input[offset] == element {
        ParseResult::successful(input[offset].clone(), 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("expected: {}, but got: {}", element, input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
}
