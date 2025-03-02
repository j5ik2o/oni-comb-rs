use crate::core::{CommittedStatus, Element, ParseError, ParseResult, StaticParser, StaticParsers};
use crate::extension::parsers::{
  StaticElementsParsers, StaticLoggingParsers, StaticPrimitiveParsers, StaticRepeatParsers,
};
use regex::Regex;
use std::char;
use std::fmt::{Debug, Display};
use std::str;

pub mod elements_parsers_impl;
pub mod logging_parsers_impl;
pub mod primitive_parsers_impl;
pub mod repeat_parsers_impl;

/// StaticParserの実装を提供する構造体
pub struct StaticParsersImpl;

impl StaticParsers for StaticParsersImpl {
  type P<'p, I, A>
    = StaticParser<'p, I, A>
  where
    I: 'p,
    A: 'p + 'static;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a + 'static,
    'b: 'a, {
    use crate::prelude::ParserRunner;
    parser
      .parse(input)
      .success()
      .ok_or_else(|| ParseError::of_custom(0, None, "Parse failed".to_string()))
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a + 'static, {
    StaticParser::new(move |_| ParseResult::successful(value.clone(), 0))
  }

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| ParseResult::successful(value(), 0))
  }

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| ParseResult::failed(value.clone(), committed))
  }

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| {
      let (error, committed) = f();
      ParseResult::failed(error, committed)
    })
  }

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + 'static + Clone, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| {
      if f(&a) {
        Self::successful(a)
      } else {
        Self::failed(
          ParseError::of_custom(0, None, "Filter predicate failed".to_string()),
          CommittedStatus::Uncommitted,
        )
      }
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a + Clone,
    A: 'a + 'static,
    B: 'a + 'static + Clone, {
    parser.flat_map(f)
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: 'a + 'static,
    B: Clone + 'a + 'static, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| Self::successful(f(a)))
  }
}

impl StaticParsersImpl {
  // Helper functions for lazy_static tests
  pub fn lazy_static_str(s: &str) -> String {
    s.to_string()
  }

  pub fn lazy_static_parser<'a>() -> StaticParser<'a, char, &'a str> {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();

      if offset + 3 <= input.len() {
        if input[offset] == 'a' && input[offset + 1] == 'b' && input[offset + 2] == 'c' {
          ParseResult::successful("abc", 3)
        } else {
          ParseResult::failed_with_uncommitted(ParseError::of_mismatch(input, offset, 0, "expected 'abc'".to_string()))
        }
      } else {
        ParseResult::failed_with_uncommitted(ParseError::of_in_complete())
      }
    })
  }

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
      let input: &[I] = parse_state.input();
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
  pub fn elm_any_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: crate::core::Element + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        ParseResult::successful(&input[offset..offset + 1], 1)
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 任意の要素を解析するStaticParserを返します。
  pub fn elm_any<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: crate::core::Element + Clone + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        let mut result = Vec::new();
        result.push(input[offset].clone());
        ParseResult::successful(result, 1)
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
    I: crate::core::Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && input[offset] == element {
        ParseResult::successful(&input[offset], 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("expected: {:?}, but got: {:?}", element, input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 指定した要素を解析するStaticParserを返します。
  pub fn elm<'a, I>(element: I) -> StaticParser<'a, I, I>
  where
    I: crate::core::Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && input[offset] == element {
        ParseResult::successful(input[offset].clone(), 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("expected: {:?}, but got: {:?}", element, input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致する要素を解析するStaticParserを返します。(参照版)
  pub fn elm_pred_ref<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(f)
  }

  /// 条件に一致する要素を解析するStaticParserを返します。
  pub fn elm_pred<'a, I, F>(f: F) -> StaticParser<'a, I, Vec<I>>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(f).map(|slice| slice.to_vec())
  }

  /// 16進数を解析するStaticParserを返します。(参照版)
  pub fn elm_hex_digit_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_hex_digit())
  }

  /// 16進数を解析するStaticParserを返します。
  pub fn elm_hex_digit<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::elm_pred(|e: &I| e.is_ascii_hex_digit())
  }

  /// 8進数を解析するStaticParserを返します。(参照版)
  pub fn elm_oct_digit_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_oct_digit())
  }

  /// 8進数を解析するStaticParserを返します。
  pub fn elm_oct_digit<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::elm_pred(|e: &I| e.is_ascii_oct_digit())
  }

  /// 指定した要素のいずれかを解析するStaticParserを返します。(参照版)
  pub fn elm_ref_of<'a, I>(set: &'a [I]) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(move |e: &I| set.contains(e))
  }

  /// 指定した要素のいずれかを解析するStaticParserを返します。
  pub fn elm_of<'a, I>(set: &'a [I]) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::elm_ref_of(set).map(|slice| slice.to_vec())
  }

  /// 指定した範囲の要素を解析するStaticParserを返します。(参照版)
  pub fn elm_ref_in<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + PartialOrd + 'a, {
    Self::take_while1(move |e: &I| *e >= start && *e <= end)
  }

  /// 指定した範囲の要素を解析するStaticParserを返します。
  pub fn elm_in<'a, I>(start: I, end: I) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + PartialOrd + 'a, {
    Self::elm_ref_in(start, end).map(|slice| slice.to_vec())
  }

  /// 指定した範囲の要素を解析するStaticParserを返します。(参照版)
  pub fn elm_ref_from_until<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + PartialOrd + 'a, {
    Self::take_while1(move |e: &I| *e >= start && *e < end)
  }

  /// 指定した範囲の要素を解析するStaticParserを返します。
  pub fn elm_from_until<'a, I>(start: I, end: I) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + PartialOrd + 'a, {
    Self::elm_ref_from_until(start, end).map(|slice| slice.to_vec())
  }

  /// 指定した要素のいずれでもない要素を解析するStaticParserを返します。(参照版)
  pub fn none_ref_of<'a, I>(set: &'a [I]) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(move |e: &I| !set.contains(e))
  }

  /// 指定した要素のいずれでもない要素を解析するStaticParserを返します。
  pub fn none_of<'a, I>(set: &'a [I]) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::none_ref_of(set).map(|slice| slice.to_vec())
  }

  /// 空白文字を解析するStaticParserを返します。(参照版)
  pub fn elm_space_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_space())
  }

  /// 空白文字を解析するStaticParserを返します。
  pub fn elm_space<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_space()).map(|slice| slice.to_vec())
  }

  /// 複数の空白文字を解析するStaticParserを返します。(参照版)
  pub fn elm_multi_space_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_multi_space())
  }

  /// 複数の空白文字を解析するStaticParserを返します。
  pub fn elm_multi_space<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_multi_space()).map(|slice| slice.to_vec())
  }

  /// アルファベットを解析するStaticParserを返します。(参照版)
  pub fn elm_alpha_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_alpha())
  }

  /// アルファベットを解析するStaticParserを返します。
  pub fn elm_alpha<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_alpha()).map(|slice| slice.to_vec())
  }

  /// アルファベットと数字を解析するStaticParserを返します。(参照版)
  pub fn elm_alpha_digit_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_alpha_digit())
  }

  /// アルファベットと数字を解析するStaticParserを返します。
  pub fn elm_alpha_digit<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_alpha_digit()).map(|slice| slice.to_vec())
  }

  /// 数字を解析するStaticParserを返します。(参照版)
  pub fn elm_digit_ref<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_digit())
  }

  /// 数字を解析するStaticParserを返します。
  pub fn elm_digit<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::take_while1(|e: &I| e.is_ascii_digit()).map(|slice| slice.to_vec())
  }

  /// 指定した数の要素を取得するStaticParserを返します。
  pub fn take<'a, I>(n: usize) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();

      if offset + n <= input.len() {
        ParseResult::successful(&input[offset..offset + n], n)
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致する限り要素を取得するStaticParserを返します。(0個以上)
  pub fn take_while0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;

      while i < input.len() && f(&input[i]) {
        i += 1;
      }

      ParseResult::successful(&input[offset..i], i - offset)
    })
  }

  /// 条件に一致する限り要素を取得するStaticParserを返します。(1個以上)
  pub fn take_while1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();

      if offset >= input.len() {
        let pe = ParseError::of_in_complete();
        return ParseResult::failed_with_uncommitted(pe);
      }

      let mut i = offset;

      while i < input.len() && f(&input[i]) {
        i += 1;
      }

      if i > offset {
        ParseResult::successful(&input[offset..i], i - offset)
      } else {
        let pe = ParseError::of_in_complete();
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致する限り要素を取得するStaticParserを返します。(n個以上m個以下)
  pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();

      if offset >= input.len() {
        let pe = ParseError::of_in_complete();
        return ParseResult::failed_with_uncommitted(pe);
      }

      let mut i = offset;

      while i < input.len() && (i - offset) < m && f(&input[i]) {
        i += 1;
      }

      if (i - offset) >= n {
        ParseResult::successful(&input[offset..i], i - offset)
      } else {
        let pe = ParseError::of_in_complete();
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致するまで要素を取得するStaticParserを返します。(0個以上)
  pub fn take_till0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = 0;

      while offset + i < input.len() {
        if f(&input[offset + i]) {
          // 条件に一致する要素を含めて返す
          i += 1;
          break;
        }
        i += 1;
      }

      if i == 0 {
        ParseResult::successful(&input[offset..offset], 0)
      } else {
        ParseResult::successful(&input[offset..offset + i], i)
      }
    })
  }

  /// 条件に一致するまで要素を取得するStaticParserを返します。(1個以上)
  pub fn take_till1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = 0;
      let mut found_match = false;

      while offset + i < input.len() {
        if f(&input[offset + i]) {
          // 条件に一致する要素は含めない
          found_match = true;
          break;
        }
        i += 1;
      }

      if i == 0 || !found_match {
        let pe = ParseError::of_in_complete();
        ParseResult::failed_with_uncommitted(pe)
      } else {
        ParseResult::successful(&input[offset..offset + i], i)
      }
    })
  }

  /// 指定したシーケンスを解析するStaticParserを返します。
  pub fn seq<'a, I>(elements: &'a [I]) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + Debug + Clone + 'a + 'static, {
    let elements_vec = elements.to_vec();
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let elements_len = elements_vec.len();

      if offset + elements_len > input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        return ParseResult::failed_with_uncommitted(pe);
      }

      for i in 0..elements_len {
        if input[offset + i] != elements_vec[i] {
          let msg = format!(
            "expected: {:?}, but got: {:?}",
            elements_vec,
            &input[offset..offset + elements_len]
          );
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }

      ParseResult::successful(elements_vec.clone(), elements_len)
    })
  }

  /// 指定したタグを解析するStaticParserを返します。
  pub fn tag<'a, I, S>(tag: S) -> StaticParser<'a, I, String>
  where
    I: Element + Clone + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let tag_str = tag.as_ref().to_string();
    let tag_chars: Vec<char> = tag_str.chars().collect();

    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let tag_len = tag_chars.len();

      if offset + tag_len > input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        return ParseResult::failed_with_uncommitted(pe);
      }

      for i in 0..tag_len {
        let c = input[offset + i].clone().to_char();
        if c != tag_chars[i] {
          let msg = format!("expected: {}, but got: {:?}", tag_str, &input[offset..offset + tag_len]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }

      ParseResult::successful(tag_str.clone(), tag_len)
    })
  }

  /// 大文字小文字を区別せずに指定したタグを解析するStaticParserを返します。
  pub fn tag_no_case<'a, I, S>(tag: S) -> StaticParser<'a, I, String>
  where
    I: Element + Clone + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let tag_str = tag.as_ref().to_string();
    let tag_chars: Vec<char> = tag_str.chars().collect();

    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let tag_len = tag_chars.len();

      if offset + tag_len > input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        return ParseResult::failed_with_uncommitted(pe);
      }

      for i in 0..tag_len {
        let c = input[offset + i].clone().to_char();
        let c_lower = c.to_lowercase().next().unwrap_or(c);
        let tag_lower = tag_chars[i].to_lowercase().next().unwrap_or(tag_chars[i]);
        if c_lower != tag_lower {
          let msg = format!(
            "expected: {} (case insensitive), but got: {:?}",
            tag_str,
            &input[offset..offset + tag_len]
          );
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }

      // Return the matched string in lowercase to match the expected behavior
      let matched_str = input[offset..offset + tag_len]
        .iter()
        .map(|e| e.clone().to_char().to_lowercase().next().unwrap_or(e.clone().to_char()))
        .collect::<String>();

      ParseResult::successful(matched_str, tag_len)
    })
  }

  /// 正規表現にマッチする文字列を解析するStaticParserを返します。
  pub fn regex<'a, I, S>(pattern: S) -> StaticParser<'a, I, String>
  where
    I: Element + Clone + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let pattern_str = pattern.as_ref().to_string();
    let re = Regex::new(&pattern_str).unwrap();

    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();

      // Convert input to string for regex matching
      let input_str: String = input[offset..].iter().map(|e| e.clone().to_char()).collect();

      if let Some(m) = re.find(&input_str) {
        if m.start() == 0 {
          let matched_str = input_str[m.start()..m.end()].to_string();
          ParseResult::successful(matched_str, m.end())
        } else {
          let msg = format!("regex pattern did not match at the beginning");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("regex pattern did not match");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 遅延評価するStaticParserを返します。
  pub fn lazy<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> StaticParser<'a, I, A> + 'a,
    A: Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let parser = f();
      let method = parser.method;
      (method)(parse_state)
    })
  }

  /// 指定したStaticParserを囲むStaticParserを返します。
  pub fn surround<'a, I, A, B, C>(
    lp: StaticParser<'a, I, A>,
    parser: StaticParser<'a, I, B>,
    rp: StaticParser<'a, I, C>,
  ) -> StaticParser<'a, I, &'a [I]>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input = parse_state.input();
      let offset = parse_state.next_offset();

      // 左側のパーサーを実行
      match lp.parse(&input[offset..]) {
        ParseResult::Success { length: n1, .. } => {
          // 左側のパーサーが成功した場合、中央のパーサーを実行
          let middle_offset = offset + n1;
          if middle_offset >= input.len() {
            return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
          }

          match parser.parse(&input[middle_offset..]) {
            ParseResult::Success { length: n2, .. } => {
              // 中央のパーサーが成功した場合、右側のパーサーを実行
              let right_offset = middle_offset + n2;
              if right_offset >= input.len() {
                return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
              }

              match rp.parse(&input[right_offset..]) {
                ParseResult::Success { length: n3, .. } => {
                  // すべてのパーサーが成功した場合、中央部分（bodyの部分）を返す
                  let body_slice = &input[middle_offset..middle_offset + n2];
                  ParseResult::successful(body_slice, n1 + n2 + n3)
                }
                ParseResult::Failure {
                  error,
                  committed_status,
                } => {
                  // 右側のパーサーが失敗した場合、エラーを伝播
                  ParseResult::failed(error, committed_status)
                }
              }
            }
            ParseResult::Failure {
              error,
              committed_status,
            } => {
              // 中央のパーサーが失敗した場合、エラーを伝播
              ParseResult::failed(error, committed_status)
            }
          }
        }
        ParseResult::Failure {
          error,
          committed_status,
        } => {
          // 左側のパーサーが失敗した場合、エラーを伝播
          ParseResult::failed(error, committed_status)
        }
      }
    })
  }

  /// 指定した数の要素をスキップするStaticParserを返します。
  pub fn skip<'a, I>(n: usize) -> StaticParser<'a, I, ()> {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset + n <= input.len() {
        ParseResult::successful((), n)
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
}
