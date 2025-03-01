use crate::core::{CommittedStatus, Element, ParseError, ParseResult, ParseState, StaticParser};
use std::fmt::{Debug, Display};
use regex::Regex;
use std::str;
use std::char;

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
  pub fn elm_any_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: crate::core::Element + PartialEq + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
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
      let input: &[I] = parse_state.input();
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
      let input: &[I] = parse_state.input();
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
      let input: &[I] = parse_state.input();
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
  
  /// 条件に一致する要素までの連続を返すStaticParserを返します。
  /// 解析結果の長さは1要素以上必要です。
  pub fn take_till1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;
      let mut found = false;
      while i < input.len() {
        if f(&input[i]) {
          found = true;
          break;
        }
        i += 1;
      }
      if found {
        if i > offset {
          ParseResult::successful(&input[offset..i], i - offset)
        } else {
          let msg = format!("expected at least one element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        ParseResult::failed_with_uncommitted(ParseError::of_in_complete())
      }
    })
  }
  
  /// 指定された数の要素をスキップするStaticParserを返します。
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
  
  /// openとcloseに囲まれたbodyを解析するStaticParserを返します。
  pub fn surround<'a, I, A, B, C>(
    lp: StaticParser<'a, I, A>,
    parser: StaticParser<'a, I, B>,
    rp: StaticParser<'a, I, C>,
  ) -> StaticParser<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let lp_method = lp.method.clone();
      let parser_method = parser.method.clone();
      let rp_method = rp.method.clone();
      
      match (lp_method)(parse_state) {
        ParseResult::Success { length: n1, .. } => {
          let next_state = parse_state.next(n1);
          match (parser_method)(next_state) {
            ParseResult::Success { value, length: n2 } => {
              let next_state = next_state.next(n2);
              match (rp_method)(next_state) {
                ParseResult::Success { length: n3, .. } => {
                  ParseResult::successful(value, n1 + n2 + n3)
                }
                ParseResult::Failure { error, committed_status } => {
                  ParseResult::failed(error, committed_status)
                }
              }
            }
            ParseResult::Failure { error, committed_status } => {
              ParseResult::failed(error, committed_status)
            }
          }
        }
        ParseResult::Failure { error, committed_status } => {
          ParseResult::failed(error, committed_status)
        }
      }
    })
  }
  
  /// 指定したStaticParserを遅延評価するStaticParserを返します。
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
  
  /// 指定したシーケンスを解析するStaticParserを返します。
  pub fn seq<'a, I>(elements: &'a [I]) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let elements_len = elements.len();
      
      if offset + elements_len > input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        return ParseResult::failed_with_uncommitted(pe);
      }
      
      for i in 0..elements_len {
        if input[offset + i] != elements[i] {
          let msg = format!("expected: {:?}, but got: {:?}", elements, &input[offset..offset + elements_len]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }
      
      ParseResult::successful(elements, elements_len)
    })
  }
  
  /// 指定したタグを解析するStaticParserを返します。
  pub fn tag<'a, I, S>(tag: S) -> StaticParser<'a, I, &'a str>
  where
    I: Element + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let tag_str = tag.as_ref();
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
        if let Some(c) = input[offset + i].to_char() {
          if c != tag_chars[i] {
            let msg = format!("expected: {}, but got: {:?}", tag_str, &input[offset..offset + tag_len]);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            return ParseResult::failed_with_uncommitted(pe);
          }
        } else {
          let msg = format!("expected: {}, but got: {:?}", tag_str, &input[offset..offset + tag_len]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }
      
      ParseResult::successful(tag_str, tag_len)
    })
  }
  
  /// 大文字小文字を区別せずに指定したタグを解析するStaticParserを返します。
  pub fn tag_no_case<'a, I, S>(tag: S) -> StaticParser<'a, I, &'a str>
  where
    I: Element + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let tag_str = tag.as_ref();
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
        if let Some(c) = input[offset + i].to_char() {
          if c.to_lowercase().next().unwrap_or(c) != tag_chars[i].to_lowercase().next().unwrap_or(tag_chars[i]) {
            let msg = format!("expected: {} (case insensitive), but got: {:?}", tag_str, &input[offset..offset + tag_len]);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            return ParseResult::failed_with_uncommitted(pe);
          }
        } else {
          let msg = format!("expected: {} (case insensitive), but got: {:?}", tag_str, &input[offset..offset + tag_len]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          return ParseResult::failed_with_uncommitted(pe);
        }
      }
      
      ParseResult::successful(tag_str, tag_len)
    })
  }
  
  /// 正規表現にマッチする文字列を解析するStaticParserを返します。
  pub fn regex<'a, I, S>(pattern: S) -> StaticParser<'a, I, &'a str>
  where
    I: Element + PartialEq + Debug + 'a,
    S: AsRef<str> + 'a, {
    let pattern_str = pattern.as_ref();
    let re = Regex::new(pattern_str).unwrap();
    
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      
      if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        return ParseResult::failed_with_uncommitted(pe);
      }
      
      let input_str: String = input[offset..].iter().filter_map(|e| e.to_char()).collect();
      
      if let Some(m) = re.find(&input_str) {
        if m.start() == 0 {
          let matched_str = &input_str[m.start()..m.end()];
          ParseResult::successful(matched_str, m.end())
        } else {
          let msg = format!("expected: pattern {}, but got: {}", pattern_str, input_str);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("expected: pattern {}, but got: {}", pattern_str, input_str);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
  
  /// 指定した数の要素を取得するStaticParserを返します。
  pub fn take<'a, I>(n: usize) -> StaticParser<'a, I, &'a [I]>
  where
    I: Debug + 'a, {
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
  
  /// 条件に一致する要素の連続を返すStaticParserを返します。
  pub fn take_while0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Debug + 'a, {
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
  
  /// 条件に一致する要素の連続を返すStaticParserを返します。
  /// 解析結果の長さは1要素以上必要です。
  pub fn take_while1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;
      
      while i < input.len() && f(&input[i]) {
        i += 1;
      }
      
      if i > offset {
        ParseResult::successful(&input[offset..i], i - offset)
      } else {
        let msg = format!("expected at least one element");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
  
  /// 条件に一致する要素の連続を返すStaticParserを返します。
  /// 解析結果の長さはmin以上max以下である必要があります。
  pub fn take_while_n_m<'a, I, F>(min: usize, max: usize, f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;
      let mut count = 0;
      
      while i < input.len() && f(&input[i]) && count < max {
        i += 1;
        count += 1;
      }
      
      if count >= min {
        ParseResult::successful(&input[offset..i], i - offset)
      } else {
        let msg = format!("expected at least {} elements", min);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
  
  /// 条件に一致する要素までの連続を返すStaticParserを返します。
  pub fn take_till0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;
      
      while i < input.len() && !f(&input[i]) {
        i += 1;
      }
      
      if i < input.len() {
        ParseResult::successful(&input[offset..i], i - offset)
      } else {
        ParseResult::failed_with_uncommitted(ParseError::of_in_complete())
      }
    })
  }
  /// 英数字を解析するStaticParserを返します。(参照版)
  pub fn elm_alpha_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_alphanumeric() {
            ParseResult::successful(&input[offset], 1)
          } else {
            let msg = format!("expected: alphanumeric, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: alphanumeric, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 英数字を解析するStaticParserを返します。
  pub fn elm_alpha_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_alphanumeric() {
            ParseResult::successful(input[offset].clone(), 1)
          } else {
            let msg = format!("expected: alphanumeric, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: alphanumeric, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 数字を解析するStaticParserを返します。(参照版)
  pub fn elm_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(10) {
            ParseResult::successful(&input[offset], 1)
          } else {
            let msg = format!("expected: digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 数字を解析するStaticParserを返します。
  pub fn elm_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(10) {
            ParseResult::successful(input[offset].clone(), 1)
          } else {
            let msg = format!("expected: digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 16進数を解析するStaticParserを返します。(参照版)
  pub fn elm_hex_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(16) {
            ParseResult::successful(&input[offset], 1)
          } else {
            let msg = format!("expected: hex digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: hex digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 16進数を解析するStaticParserを返します。
  pub fn elm_hex_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(16) {
            ParseResult::successful(input[offset].clone(), 1)
          } else {
            let msg = format!("expected: hex digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: hex digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 8進数を解析するStaticParserを返します。(参照版)
  pub fn elm_oct_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(8) {
            ParseResult::successful(&input[offset], 1)
          } else {
            let msg = format!("expected: octal digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: octal digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 8進数を解析するStaticParserを返します。
  pub fn elm_oct_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if let Some(c) = input[offset].to_char() {
          if c.is_digit(8) {
            ParseResult::successful(input[offset].clone(), 1)
          } else {
            let msg = format!("expected: octal digit, but got: {:?}", c);
            let pe = ParseError::of_mismatch(input, offset, 0, msg);
            ParseResult::failed_with_uncommitted(pe)
          }
        } else {
          let msg = format!("expected: octal digit, but got: non-char element");
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致する要素を解析するStaticParserを返します。(参照版)
  pub fn elm_pred_ref<'a, I, F>(f: F) -> StaticParser<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if f(&input[offset]) {
          ParseResult::successful(&input[offset], 1)
        } else {
          let msg = format!("predicate failed for: {:?}", input[offset]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// 条件に一致する要素を解析するStaticParserを返します。
  pub fn elm_pred<'a, I, F>(f: F) -> StaticParser<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() {
        if f(&input[offset]) {
          ParseResult::successful(input[offset].clone(), 1)
        } else {
          let msg = format!("predicate failed for: {:?}", input[offset]);
          let pe = ParseError::of_mismatch(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      } else {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }
}
