#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]
mod core;
mod extension;
mod internal;
mod utils;

pub mod prelude {
  pub use crate::core::*;
  pub use crate::extension::parser::*;
  pub use crate::extension::parsers::*;
  use crate::internal::*;
  pub use crate::utils::*;
  use std::fmt::{Debug, Display};

  /// Returns a [Parser] that does nothing.<br/>
  /// 何もしない[Parser]を返します。
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// # use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "a";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, ()> = unit();
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn unit<'a, I>() -> Parser<'a, I, ()> {
    ParsersImpl::unit()
  }

  /// Returns a [Parser] that does nothing. It is an alias for `unit()`.<br/>
  /// 何もしない[Parser]を返します。`unit()`のエイリアスです。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "a";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, ()> = empty();
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn empty<'a, I>() -> Parser<'a, I, ()> {
    ParsersImpl::empty()
  }

  /// Returns a [Parser] representing the termination.<br/>
  /// 終端を表す[Parser]を返します。
  ///
  /// Returns `Ok(())` if the termination is parsed successfully, `Err(Mismatch)` if the parsing fails.
  ///
  /// 終端の解析に成功したら`Ok(())`を返し、解析に失敗したら`Err(Mismatch)`を返します。
  ///
  /// # Example(例)
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "a";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, ()> = end();
  ///
  /// let result: Result<(), ParseError<char>> = parser.parse(&input).to_result();
  ///
  /// assert!(result.is_err());
  /// ```
  ///
  /// ## Failure case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, ()> = end();
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn end<'a, I>() -> Parser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    ParsersImpl::end()
  }

  /// Returns a [Parser] representing the successful parsing result.<br/>
  /// 成功した解析結果を表す[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = successful('a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), 'a');
  /// ```
  pub fn successful<'a, I, A>(value: A) -> Parser<'a, I, A>
  where
    I: 'a,
    A: Clone + 'a, {
    ParsersImpl::successful(value)
  }

  /// Returns a [Parser] representing the successful parsing result.<br/>
  /// 成功した解析結果を表す[Parser]を返します。
  ///
  /// - f: a closure that returns the parsed result value.
  /// - f: 解析結果の値を返すクロージャ
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = successful_lazy(|| 'a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), 'a');
  /// ```
  pub fn successful_lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    I: 'a,
    F: Fn() -> A + 'a,
    A: 'a, {
    ParsersImpl::successful_lazy(f)
  }

  /// Returns a [Parser] that represents the result of the failed parsing.<br/>
  /// 失敗した解析結果を表す[Parser]を返します。
  ///
  /// - value: [ParseError]
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parse_error: ParseError<char> = ParseError::of_in_complete();
  ///
  /// let parser: Parser<char, ()> = failed(parse_error.clone(), CommittedStatus::Committed);
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed<'a, I, A>(value: ParseError<'a, I>, commit: CommittedStatus) -> Parser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    ParsersImpl::failed(value, commit)
  }

  /// Returns a [Parser] that returns and commits the failed parsing result.<br/>
  /// 失敗した解析結果を返しコミットする[Parser]を返します。
  ///
  /// - value: [ParseError]
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parse_error: ParseError<char> = ParseError::of_in_complete();
  ///
  /// let parser: Parser<char, ()> = failed_with_commit(parse_error.clone());
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Committed);
  ///
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_with_commit<'a, I, A>(value: ParseError<'a, I>) -> Parser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    ParsersImpl::failed(value, CommittedStatus::Committed)
  }

  /// Returns a [Parser] that returns failed parsing results and does not commit.<br/>
  /// 失敗した解析結果を返しコミットしない[Parser]を返します。
  ///
  /// - value: [ParseError]
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parse_error: ParseError<char> = ParseError::of_in_complete();
  ///
  /// let parser: Parser<char, ()> = failed_with_uncommit(parse_error.clone());
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Uncommitted);
  ///
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_with_uncommit<'a, I, A>(value: ParseError<'a, I>) -> Parser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    ParsersImpl::failed(value, CommittedStatus::Uncommitted)
  }

  /// Returns a Parser that represents the result of the failed parsing.<br/>
  /// 失敗した解析結果を表すParserを返します。
  ///
  /// - f: 失敗した解析結果を返すクロージャ
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parse_error: ParseError<char> = ParseError::of_in_complete();
  ///
  /// let parser: Parser<char, ()> = failed_lazy(|| (parse_error.clone(), CommittedStatus::Committed));
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    ParsersImpl::failed_lazy(f)
  }

  // --- Element Parsers ---
  /// Returns a [Parser] that parses an any element.(for reference)<br/>
  /// 任意の要素を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &char> = elm_any_ref();
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  pub fn elm_any_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_any_ref()
  }

  /// Returns a [Parser] that parses an any element.<br/>
  /// 任意の要素を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = elm_any();
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  pub fn elm_any<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_any()
  }

  /// Returns a [Parser] that parses the specified element.(for reference)<br/>
  /// 指定した要素を解析する[Parser]を返します。(参照版)
  ///
  /// - element: element
  /// - element: 要素
  ///
  /// # Example(例)
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &char> = elm_ref('x');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  ///
  /// ## Failure case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &char> = elm_ref('a');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert!(result.failure().unwrap().is_mismatch());
  /// ```
  pub fn elm_ref<'a, I>(element: I) -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_ref(element)
  }

  /// Returns a [Parser] that parses the specified element.<br/>
  /// 指定した要素を解析する[Parser]を返します。
  ///
  /// - element: an element
  /// - element: 要素
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = elm('x');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  ///
  /// ## Failure case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = elm('a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert!(result.failure().unwrap().is_mismatch());
  /// ```
  pub fn elm<'a, I>(element: I) -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm(element)
  }

  /// Returns a parser that parses the elements that satisfy the specified closure conditions.(for reference)<br/>
  /// 指定されたクロージャの条件を満たす要素を解析する[Parser]を返します。(参照版)
  ///
  /// - f: Closure(クロージャ)
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &char> = elm_pred_ref(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  ///
  /// ## Failure case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &char> = elm_pred_ref(|c| *c == 'a');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert!(result.failure().unwrap().is_mismatch());
  /// ```
  pub fn elm_pred_ref<'a, I, F>(f: F) -> Parser<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_pred_ref(f)
  }

  /// Returns a parser that parses the elements that satisfy the specified closure conditions.<br/>
  /// 指定されたクロージャの条件を満たす要素を解析するパーサーを返します。
  ///
  /// - f: closure
  /// - f: クロージャ
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = elm_pred(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  ///
  /// ## Failure case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, char> = elm_pred(|c| *c == 'a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert!(result.failure().unwrap().is_mismatch());
  /// ```
  pub fn elm_pred<'a, I, F>(f: F) -> Parser<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_pred(f)
  }

  /// Returns a [Parser] that parses the elements in the specified set. (for reference)<br/>
  /// 指定した集合の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_ref_of("xyz").of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_ref_of<'a, I, S>(set: &'a S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::elm_ref_of(set)
  }

  /// Returns a [Parser] that parses the elements in the specified set.<br/>
  /// 指定した集合の要素を解析する[Parser]を返します。
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  pub fn elm_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::elm_of(set)
  }

  /// Returns a [Parser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  pub fn elm_in_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    ParsersImpl::elm_ref_in(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  pub fn elm_in<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    ParsersImpl::elm_in(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  pub fn elm_from_until_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    ParsersImpl::elm_ref_from_until(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  pub fn elm_from_until<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    ParsersImpl::elm_from_until(start, end)
  }

  /// Returns a [Parser] that parses elements that do not contain elements of the specified set.(for reference)
  /// 指定した集合の要素を含まない要素を解析する[Parser]を返します。(参照版)
  ///
  /// - set: a element of sets
  /// - set: 要素の集合
  pub fn none_ref_of<'a, I, S>(set: &'a S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::none_ref_of(set)
  }

  /// Returns a [Parser] that parses elements that do not contain elements of the specified set.
  /// 指定した集合の要素を含まない要素を解析する[Parser]を返します。
  ///
  /// - set: an element of sets
  /// - set: 要素の集合
  pub fn none_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::none_of(set)
  }

  /// Returns a [Parser] that parses the space (' ', '\t'). (for reference)<br/>
  /// スペース(' ', '\t')を解析する[Parser]を返します。(参照版)
  pub fn elm_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_space_ref()
  }

  /// Returns a [Parser] that parses the space (' ', '\t').<br/>
  /// スペース(' ', '\t')を解析する[Parser]を返します。
  pub fn elm_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_space()
  }

  /// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r'). (for reference)<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。(参照版)
  pub fn elm_multi_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_multi_space_ref()
  }

  /// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r').<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。
  pub fn elm_multi_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_multi_space()
  }

  /// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').(for reference)<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
  pub fn elm_alpha_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_alpha_ref()
  }

  /// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。
  pub fn elm_alpha<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_alpha()
  }

  /// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').(for reference)<br/>
  /// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
  pub fn elm_alpha_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_alpha_digit_ref()
  }

  /// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').<br/>
  /// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。
  pub fn elm_alpha_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_alpha_digit()
  }

  /// Returns a [Parser] that parses digits ('0'..='9').(for reference)<br/>
  /// 数字('0'..='9')を解析する[Parser]を返します。(参照版)
  pub fn elm_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_digit_ref()
  }

  /// Returns a [Parser] that parses digits ('0'..='9').<br/>
  /// 数字('0'..='9')を解析する[Parser]を返します。
  pub fn elm_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_digit()
  }

  /// Returns a [Parser] that parses digits ('1'..='9').(for reference)<br/>
  /// 数字('1'..='9')を解析する[Parser]を返します。(参照版)
  pub fn elm_digit_1_9_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    elm_digit_ref().with_filter_not(|c: &&I| c.is_ascii_digit_zero())
  }

  /// Returns a [Parser] that parses digits ('1'..='9').<br/>
  /// 数字('1'..='9')を解析する[Parser]を返します。
  pub fn elm_digit_1_9<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    elm_digit_1_9_ref().map(Clone::clone)
  }

  /// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').(for reference)<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。(参照版)
  pub fn elm_hex_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_hex_digit_ref()
  }

  /// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。
  pub fn elm_hex_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_hex_digit()
  }

  /// Returns a [Parser] that parses oct digits ('0'..='8').(for reference)<br/>
  /// 8進の数字('0'..='8')を解析する[Parser]を返します。(参照版)
  pub fn elm_oct_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_oct_digit_ref()
  }

  /// Returns a [Parser] that parses oct digits ('0'..='8').<br/>
  /// 8進の数字('0'..='8')を解析する[Parser]を返します。
  pub fn elm_oct_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + PartialEq + Clone + 'a, {
    ParsersImpl::elm_oct_digit()
  }

  // --- Elements Parsers ---

  /// Returns a [Parser] that parses a sequence of elements.
  /// 要素の列を解析する[Parser]を返す。
  pub fn seq<'a, 'b, I>(seq: &'b [I]) -> Parser<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a, {
    ParsersImpl::seq(seq)
  }

  /// Returns a [Parser] that parses a string.
  /// 文字列を解析する[Parser]を返す。
  ///
  /// - tag: a string
  /// - tag: 文字列
  ///
  /// # Example
  ///
  /// ## Success case
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &str> = tag("abc");
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ## Failure case
  ///
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "defabc";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, &str> = tag("abc");
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert!(result.failure().unwrap().is_mismatch());
  /// ```
  pub fn tag<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
  where
    'b: 'a, {
    ParsersImpl::tag(tag)
  }

  pub fn tag_no_case<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
  where
    'b: 'a, {
    ParsersImpl::tag_no_case(tag)
  }

  pub fn regex<'a>(pattern: &str) -> Parser<'a, char, String> {
    ParsersImpl::regex(pattern)
  }

  pub fn take<'a, I>(n: usize) -> Parser<'a, I, &'a [I]> {
    ParsersImpl::take(n)
  }

  pub fn take_while0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while0(f)
  }

  pub fn take_while1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while1(f)
  }

  pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while_n_m(n, m, f)
  }

  pub fn take_till0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till0(f)
  }

  pub fn take_till1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till1(f)
  }

  // --- Offset Control Parsers ---

  pub fn skip<'a, I>(n: usize) -> Parser<'a, I, ()> {
    ParsersImpl::skip(n)
  }

  // --- Enhanced Parsers ---

  pub fn surround<'a, I, A, B, C>(
    lp: Parser<'a, I, A>,
    parser: Parser<'a, I, B>,
    rp: Parser<'a, I, C>,
  ) -> Parser<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    ParsersImpl::surround(lp, parser, rp)
  }

  pub fn lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> Parser<'a, I, A> + 'a,
    A: Debug + 'a, {
    ParsersImpl::lazy(f)
  }
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::iter::FromIterator;

  use crate::core::{ParserFunctor, ParserMonad, ParserRunner};

  use crate::extension::parser::{
    CollectParser, ConversionParser, DiscardParser, LoggingParser, OffsetParser, OperatorParser, RepeatParser,
  };

  use super::prelude::*;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_attempt() {
    init();
    {
      let input1 = b"b";
      let p: Parser<u8, &u8> = failed_with_commit(ParseError::of_in_complete())
        .attempt()
        .or(elm_ref(b'b'));

      let r = p.parse_as_result(input1);
      assert!(r.is_ok());
    }

    {
      let input1 = "abra cadabra!".chars().collect::<Vec<char>>();
      let p = (tag("abra") + elm_space() + tag("abra")).attempt() | (tag("abra") + elm_space() + tag("cadabra!"));
      let r = p.parse_as_result(&input1);
      println!("result = {:?}", r);
      assert!(r.is_ok());
    }
  }

  #[test]
  fn test_successful_in_closure() {
    init();
    let input = b"a";
    let p = successful_lazy(|| 'a');

    let r = p.parse_as_result(input).unwrap();
    assert_eq!(r, 'a');
  }

  #[test]
  fn test_elem() {
    init();
    let p = elm(b'a');

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, b'a');
  }

  #[test]
  fn test_regex() {
    init();
    let input1 = "abc".chars().collect::<Vec<char>>();
    let input2 = "xbc".chars().collect::<Vec<char>>();
    let p = regex(r"a.*c$").name("regex_1");

    let r = p.parse_as_result(&input1);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "abc");

    let r = p.parse_as_result(&input2);
    assert!(r.is_err());
    println!("{}", r.err().unwrap());

    {
      let input3 = "12345 to".chars().collect::<Vec<_>>();
      let p = regex(r"\d+");
      let r = p.parse_as_result(&input3);
      println!("{:?}", r);
      assert!(r.is_ok());
      // assert_eq!(r.unwrap(), "abc");
    }
  }

  #[test]
  fn test_elm_of() {
    init();
    let patterns = b'a'..=b'f';
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = elm_of(&patterns);

    for index in 0..b.len() {
      let r = p.parse_as_result(&b[index..]);
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), b[index]);
    }

    let r = p.parse_as_result(b"g");
    assert!(r.is_err());
  }

  #[test]
  fn test_none_of() {
    init();
    let patterns = b'a'..=b'f';
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = none_of(&patterns);

    for index in 0..b.len() {
      let r = p.parse_as_result(&b[index..]);
      assert!(r.is_err());
    }

    let r = p.parse_as_result(b"g");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), b'g');
  }

  #[test]
  fn test_peek() {
    init();

    let input = "aname".chars().collect::<Vec<char>>();
    let p = elm('a').peek() + tag("aname");

    let result = p.parse_as_result(&input).unwrap();

    log::debug!("result = {:?}", result);
    assert_eq!(result.0, 'a');
    assert_eq!(result.1, "aname");
  }

  #[test]
  fn test_repeat() {
    init();
    let p = elm_ref(b'a').repeat(..=3).collect();

    let r = p.parse_as_result(b"");
    assert!(r.is_ok());

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);

    let r = p.parse_as_result(b"aaa").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_many_0() {
    init();
    let p = elm_ref(b'a').of_many0().collect();

    // let r = p.parse(b"").unwrap();
    // assert_eq!(r, vec![]);

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_1() {
    init();
    let p = elm_ref(b'a').of_many1().collect();

    let r = p.parse_as_result(b"");
    assert!(r.is_err());

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_n_m() {
    init();
    let p = elm_ref(b'a').of_many_n_m(1, 2).collect() + end();

    let r = p.parse_as_result(b"");
    assert!(r.is_err());

    let (a, _) = p.parse_as_result(b"a").unwrap();
    assert_eq!(a, vec![b'a']);

    let (a, _) = p.parse_as_result(b"aa").unwrap();
    assert_eq!(a, vec![b'a', b'a']);

    let r = p.parse_as_result(b"aaa");
    assert!(r.is_err());
  }

  #[test]
  fn test_count_sep() {
    init();
    let p1 = elm_ref(b'a');
    let p2 = elm_ref(b',');
    let p = p1.map(|e| *e).of_count_sep(3, p2);

    let r = p.parse_as_result(b"a,a,a").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_seq() {
    init();
    let p = seq(b"abc");

    let r = p.parse_as_result(b"abc").unwrap();
    assert_eq!(r, b"abc");
  }

  #[test]
  fn test_tag() {
    init();
    let input = "abe".chars().collect::<Vec<char>>();
    let p = tag("abc").attempt() | tag("abe");

    let r = p.parse(&input);
    println!("{:?}", r);
    assert_eq!(r.to_result().unwrap(), "abe");
  }

  #[test]
  fn test_tag_no_case() {
    init();
    let input = "AbC".chars().collect::<Vec<char>>();
    let p = tag_no_case("abc");

    let r = p.parse_as_result(&input).unwrap();
    assert_eq!(r, "abc");
  }

  #[test]
  fn test_opt() {
    init();
    let p = seq(b"abc").opt();

    if let Some(b) = p.parse_as_result(b"abc").unwrap() {
      assert_eq!(b, b"abc");
    } else {
      panic!()
    }
  }

  #[test]
  fn test_not() {
    init();
    let p = seq(b"abc").not();

    let _ = p.parse_as_result(b"def").unwrap();
  }

  #[test]
  fn test_take() {
    init();
    let str = "abc";
    let str_len = str.len();
    let mut input = vec![str_len as u8];
    input.extend_from_slice(str.as_bytes());

    // input: [u8; N]  = [ data size as u8 | data bytes ----- ]

    let bytes_parser: Parser<u8, &[u8]> = take(1).flat_map(|size: &[u8]| take(size[0] as usize));

    let ss = bytes_parser
      .parse(&input)
      .to_result()
      .map(|r| std::str::from_utf8(r).unwrap())
      .unwrap();

    println!("{}", ss);
    assert_eq!(ss, str);
  }

  #[test]
  fn test_take_2() {
    init();
    let input1 = "abcd".chars().collect::<Vec<char>>();
    let p = ((elm_ref('a') + elm_ref('b')).flat_map(|e| skip(1).map(move |_| e)) + elm_any_ref() + end())
      .collect()
      .map(|chars| String::from_iter(chars));

    let result = p.parse_as_result(&input1).unwrap();
    log::debug!("result = {:?}", result);
  }

  #[test]
  fn test_take_while0() {
    init();
    let p = take_while0(|c: &u8| c.is_ascii_digit()).map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"def");
    assert!(result.is_ok());
  }

  #[test]
  fn test_take_while1() {
    init();
    let p = take_while1(|c: &u8| c.is_ascii_digit()).map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_take_while_n_m() {
    init();
    let p = take_while_n_m(1, 3, |c: &u8| c.is_ascii_digit()).map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"a1b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1");

    let result = p.parse_as_result(b"a12b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "12");

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"a1234b");
    assert!(result.is_err());

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_take_till0() {
    init();
    let p = take_till0(|c| *c == b'c').map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"abcd");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc");

    let result = p.parse_as_result(b"def");
    assert!(result.is_ok());
  }

  #[test]
  fn test_take_till1() {
    init();
    let p = take_till1(|c| *c == b'c').map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"abcd");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc");

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_discard() {
    init();
    let p = seq(b"abc").discard();

    let result = p.parse_as_result(b"abc");
    assert!(result.is_ok());

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_and_then() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) + elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    let (a, b) = result;
    assert_eq!(*a, pv1);
    assert_eq!(*b, pv2);
  }

  #[test]
  fn test_last_offset() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm_ref(pv1);
    let p2 = elm_ref(pv2);
    let p = (p1 + p2).last_offset();

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    //  let (a, b) = result;
    //  assert_eq!(a, pv1);
    //  assert_eq!(b, pv2);
  }

  #[test]
  fn test_or() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) | elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv1);

    let result = p.parse_as_result(b"ba").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv2);
  }

  #[test]
  fn test_skip_left() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) * elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv2);
  }

  #[test]
  fn test_skip_right() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm_ref(pv1);
    let p2 = elm_ref(pv2);
    let p = p1 - p2;

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv1);
  }

  #[test]
  fn test_example1() {
    init();
    let input1 = "abc".chars().collect::<Vec<char>>();
    let input2 = "abd".chars().collect::<Vec<char>>();

    let pa = elm_ref('a');
    let pb = elm_ref('b');
    let pc = elm_ref('c');
    let pd = elm_ref('d');
    let p = (pa + pb + (pc | pd)).collect().map(String::from_iter);

    let result = p.parse_as_result(&input1).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abc");

    let result = p.parse_as_result(&input2).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abd");
  }

  #[test]
  fn test_example2() {
    init();

    let input = "aname".chars().collect::<Vec<char>>();
    let p = (elm_ref('a') + tag("name")).collect().map(String::from_iter);

    let result = p.parse_as_result(&input).unwrap();
    // let s: String = result.iter().collect();
    log::debug!("result = {:?}", result);
    // assert_eq!(s, "aname");
  }

  #[test]
  fn test_filter() {
    init();
    {
      let input: Vec<char> = "abc def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_err());
    }
    {
      let input: Vec<char> = "abc  def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_ok());
    }
  }

  #[test]
  fn test_filter_not() {
    init();
    {
      let input: Vec<char> = "abc def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter_not(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_ok());
    }
    {
      let input: Vec<char> = "abc  def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter_not(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_err());
    }
  }
}
