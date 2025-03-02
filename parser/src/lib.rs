#![warn(dead_code)]
#![allow(incomplete_features)]
mod core;
mod extension;
mod internal;
mod utils;

// Re-export StaticParser for public use
pub use crate::core::static_parser::StaticParser;

pub mod prelude {
  pub use crate::core::*;
  pub use crate::extension::parser::*;
  pub use crate::extension::parsers::*;
  use crate::internal::*;
  pub use crate::utils::*;
  use std::fmt::{Debug, Display};

  // StaticParser re-export
  pub use crate::core::static_parser::StaticParser;

  /// Returns a [Parser] that does nothing.<br/>
  /// 何もしない[Parser]を返します。
  ///
  /// # Example
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

  /// Returns a [StaticParser] that does nothing.<br/>
  /// 何もしない[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// # use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "a";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, ()> = unit_static();
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn unit_static<'a, I>() -> StaticParser<'a, I, ()> {
    StaticParsersImpl::unit()
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

  /// Returns a [StaticParser] that does nothing. It is an alias for `unit_static()`.<br/>
  /// 何もしない[StaticParser]を返します。`unit_static()`のエイリアスです。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "a";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, ()> = empty_static();
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn empty_static<'a, I>() -> StaticParser<'a, I, ()> {
    StaticParsersImpl::empty()
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
  pub fn end<'a, I>() -> Parser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    ParsersImpl::end()
  }

  /// Returns a [StaticParser] representing the termination.<br/>
  /// 終端を表す[StaticParser]を返します。
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
  /// let parser: StaticParser<char, ()> = end_static();
  ///
  /// let result: Result<(), ParseError<char>> = parser.parse(&input).to_result();
  ///
  /// assert!(result.is_err());
  /// ```
  pub fn end_static<'a, I>() -> StaticParser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    StaticParsersImpl::end()
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

  /// Returns a [StaticParser] representing the successful parsing result.<br/>
  /// 成功した解析結果を表す[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, char> = successful_static('a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), 'a');
  /// ```
  pub fn successful_static<'a, I, A>(value: A) -> StaticParser<'a, I, A>
  where
    I: 'a,
    A: Clone + 'a, {
    StaticParsersImpl::successful(value)
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

  /// Returns a [StaticParser] representing the successful parsing result.<br/>
  /// 成功した解析結果を表す[StaticParser]を返します。
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
  /// let parser: StaticParser<char, char> = successful_lazy_static(|| 'a');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), 'a');
  /// ```
  pub fn successful_lazy_static<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    I: 'a,
    F: Fn() -> A + 'a,
    A: 'a, {
    StaticParsersImpl::successful_lazy(f)
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

  /// Returns a [StaticParser] that represents the result of the failed parsing.<br/>
  /// 失敗した解析結果を表す[StaticParser]を返します。
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
  /// let parser: StaticParser<char, ()> = failed_static(parse_error.clone(), CommittedStatus::Committed);
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_static<'a, I, A>(value: ParseError<'a, I>, commit: CommittedStatus) -> StaticParser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    StaticParsersImpl::failed(value, commit)
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

  /// Returns a [StaticParser] that returns and commits the failed parsing result.<br/>
  /// 失敗した解析結果を返しコミットする[StaticParser]を返します。
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
  /// let parser: StaticParser<char, ()> = failed_with_commit_static(parse_error.clone());
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Committed);
  ///
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_with_commit_static<'a, I, A>(value: ParseError<'a, I>) -> StaticParser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    StaticParsersImpl::failed(value, CommittedStatus::Committed)
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

  /// Returns a [StaticParser] that returns and uncommits the failed parsing result.<br/>
  /// 失敗した解析結果を返しアンコミットする[StaticParser]を返します。
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
  /// let parser: StaticParser<char, ()> = failed_with_uncommit_static(parse_error.clone());
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Uncommitted);
  ///
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_with_uncommit_static<'a, I, A>(value: ParseError<'a, I>) -> StaticParser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    StaticParsersImpl::failed(value, CommittedStatus::Uncommitted)
  }

  /// Returns a [Parser] that represents the result of the failed parsing.<br/>
  /// 失敗した解析結果を表す[Parser]を返します。
  ///
  /// - f: 失敗した解析結果を返すクロージャ
  /// - f: Closure that returns failed analysis results.
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

  /// Returns a [StaticParser] that represents the result of the failed parsing.<br/>
  /// 失敗した解析結果を表す[StaticParser]を返します。
  ///
  /// - f: 失敗した解析結果を返すクロージャ
  /// - f: Closure that returns failed analysis results.
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
  /// let parser: StaticParser<char, ()> = failed_lazy_static(|| (parse_error.clone(), CommittedStatus::Committed));
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), parse_error);
  /// ```
  pub fn failed_lazy_static<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    StaticParsersImpl::failed_lazy(f)
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
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_any_ref()
  }

  /// Returns a [StaticParser] that parses an any element.(for reference)<br/>
  /// 任意の要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, &[char]> = elm_any_ref_static();
  ///
  /// let result: ParseResult<char, &[char]> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0..1]);
  /// ```
  pub fn elm_any_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_any_ref()
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
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_any()
  }

  /// Returns a [StaticParser] that parses an any element.<br/>
  /// 任意の要素を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, Vec<char>> = elm_any_static();
  ///
  /// let result: ParseResult<char, Vec<char>> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// let chars = result.success().unwrap();
  /// assert_eq!(chars.len(), 1);
  /// assert_eq!(chars[0], input[0]);
  /// ```
  pub fn elm_any_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_any()
  }

  /// Returns a [Parser] that parses the specified element.(for reference)<br/>
  /// 指定した要素を解析する[Parser]を返します。(参照版)
  ///
  /// - element: element
  /// - element: 要素
  ///
  /// # Example(例)
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
  pub fn elm_ref<'a, I>(element: I) -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_ref(element)
  }

  /// Returns a [StaticParser] that parses the specified element.(for reference)<br/>
  /// 指定した要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - element: element
  /// - element: 要素
  ///
  /// # Example(例)
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, &[char]> = elm_ref_static('x');
  ///
  /// let result: ParseResult<char, &[char]> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0..1]);
  /// ```
  pub fn elm_ref_static<'a, I>(element: I) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_ref(element)
  }

  /// Returns a [Parser] that parses the specified element.<br/>
  /// 指定した要素を解析する[Parser]を返します。
  ///
  /// - element: an element
  /// - element: 要素
  ///
  /// # Example
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
  pub fn elm<'a, I>(element: I) -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm(element)
  }

  /// Returns a [StaticParser] that parses the specified element.<br/>
  /// 指定した要素を解析する[StaticParser]を返します。
  ///
  /// - element: an element
  /// - element: 要素
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, Vec<char>> = elm_static('x');
  ///
  /// let result: ParseResult<char, Vec<char>> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// let chars = result.success().unwrap();
  /// assert_eq!(chars.len(), 1);
  /// assert_eq!(chars[0], input[0]);
  /// ```
  pub fn elm_static<'a, I>(element: I) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm(element)
  }

  /// Returns a [Parser] that parses the elements that satisfy the specified closure conditions.(for reference)<br/>
  /// 指定されたクロージャの条件を満たす要素を解析する[Parser]を返します。(参照版)
  ///
  /// - f: Closure(クロージャ)
  ///
  /// # Example
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
  pub fn elm_pred_ref<'a, I, F>(f: F) -> Parser<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_pred_ref(f)
  }

  /// Returns a [StaticParser] that parses the elements that satisfy the specified closure conditions.(for reference)<br/>
  /// 指定されたクロージャの条件を満たす要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - f: Closure(クロージャ)
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "x";
  /// let input: Vec<char> = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, &[char]> = elm_pred_ref_static(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, &[char]> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0..1]);
  /// ```
  pub fn elm_pred_ref_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_pred_ref(f)
  }

  /// Returns a [Parser] that parses the elements that satisfy the specified closure conditions.<br/>
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
  pub fn elm_pred<'a, I, F>(f: F) -> Parser<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_pred(f)
  }

  /// Returns a [StaticParser] that parses the elements that satisfy the specified closure conditions.<br/>
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
  /// let parser: StaticParser<char, Vec<char>> = elm_pred_static(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, Vec<char>> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// let chars = result.success().unwrap();
  /// assert_eq!(chars.len(), 1);
  /// assert_eq!(chars[0], input[0]);
  /// ```
  pub fn elm_pred_static<'a, I, F>(f: F) -> StaticParser<'a, I, Vec<I>>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
    StaticParsersImpl::elm_pred(f)
  }

  /// Returns a [Parser] that parses the elements in the specified set. (for reference)<br/>
  /// 指定した集合の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_ref_of("xyz").of_many1().map(|chars| chars.into_iter().map(|c| *c).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_ref_of<'a, I, S>(set: &'static S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    ParsersImpl::elm_ref_of(set)
  }

  /// Returns a [StaticParser] that parses the elements in the specified set. (for reference)<br/>
  /// 指定した集合の要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_ref_of_static("xyz").of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_ref_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;

      while i < input.len() && set.contains(&input[i]) {
        i += 1;
      }

      if i > offset {
        ParseResult::successful(&input[offset..i], i - offset)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("element not in set: {:?}", input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// Returns a [Parser] that parses the elements in the specified set.<br/>
  /// 指定した集合の要素を解析する[Parser]を返します。
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_of("xyz").of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_of<'a, I, S>(set: &'static S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    ParsersImpl::elm_of(set)
  }

  /// Returns a [StaticParser] that parses the elements in the specified set.<br/>
  /// 指定した集合の要素を解析する[StaticParser]を返します。
  ///
  /// - set: element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_of_static("xyz").of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    elm_ref_of_static(set).map(|slice| slice.to_vec())
  }

  /// Returns a [Parser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_in_ref('x', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a + 'static, {
    ParsersImpl::elm_ref_in(start, end)
  }

  /// Returns a [StaticParser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_in_ref_static('x', 'z').of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in_ref_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    StaticParsersImpl::elm_ref_in(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_in('x', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a + 'static, {
    ParsersImpl::elm_in(start, end)
  }

  /// Returns a [StaticParser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[StaticParser]を返します。
  ///
  /// - start: start element
  /// - end: end element
  ///
  /// - start: 開始要素
  /// - end: 終了要素
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_in_static('x', 'z').of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    StaticParsersImpl::elm_in(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。(参照版)
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "wxy";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_from_until_ref('w', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a + 'static, {
    ParsersImpl::elm_ref_from_until(start, end)
  }

  /// Returns a [StaticParser] that parses the elements in the specified range. (for reference)<br/>
  /// 指定した範囲の要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "wxy";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_from_until_ref_static('w', 'z').of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until_ref_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    StaticParsersImpl::elm_ref_from_until(start, end)
  }

  /// Returns a [Parser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[Parser]を返します。
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "wxy";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_from_until('w', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a + 'static, {
    ParsersImpl::elm_from_until(start, end)
  }

  /// Returns a [StaticParser] that parses the elements in the specified range.<br/>
  /// 指定した範囲の要素を解析する[StaticParser]を返します。
  ///
  /// - start: a start element
  /// - end: an end element, process up to the element at end - 1
  ///
  /// - start: 開始要素
  /// - end: 終了要素, end - 1の要素まで処理
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "wxy";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_from_until_static('w', 'z').of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    StaticParsersImpl::elm_from_until(start, end)
  }

  /// Returns a [Parser] that parses elements that do not contain elements of the specified set.(for reference)<br/>
  /// 指定した集合の要素を含まない要素を解析する[Parser]を返します。(参照版)
  ///
  /// - set: a element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = none_ref_of("abc").of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_ref_of<'a, I, S>(set: &'static S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    ParsersImpl::none_ref_of(set)
  }

  /// Returns a [StaticParser] that parses elements that do not contain elements of the specified set.(for reference)<br/>
  /// 指定した集合の要素を含まない要素を解析する[StaticParser]を返します。(参照版)
  ///
  /// - set: a element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = none_ref_of_static("abc").of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_ref_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      let mut i = offset;

      while i < input.len() && !set.contains(&input[i]) {
        i += 1;
      }

      if i > offset {
        ParseResult::successful(&input[offset..i], i - offset)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("element in set: {:?}", input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
  }

  /// Returns a [Parser] that parses elements that do not contain elements of the specified set.<br/>
  /// 指定した集合の要素を含まない要素を解析する[Parser]を返します。
  ///
  /// - set: an element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = none_of("abc").of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_of<'a, I, S>(set: &'static S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    ParsersImpl::none_of(set)
  }

  /// Returns a [StaticParser] that parses elements that do not contain elements of the specified set.<br/>
  /// 指定した集合の要素を含まない要素を解析する[StaticParser]を返します。
  ///
  /// - set: an element of sets
  /// - set: 要素の集合
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = none_of_static("abc").of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    none_ref_of_static(set).map(|slice| slice.to_vec())
  }

  /// Returns a [Parser] that parses the space (' ', '\t'). (for reference)<br/>
  /// スペース(' ', '\t')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "   ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_space_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_space_ref()
  }

  /// Returns a [StaticParser] that parses the space (' ', '\t'). (for reference)<br/>
  /// スペース(' ', '\t')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "   ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_space_ref_static().of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Clone + Debug + 'a, {
    StaticParsersImpl::elm_space_ref()
  }

  /// Returns a [Parser] that parses the space (' ', '\t').<br/>
  /// スペース(' ', '\t')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "   ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_space().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_space()
  }

  /// Returns a [StaticParser] that parses the space (' ', '\t').<br/>
  /// スペース(' ', '\t')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "   ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_space_static().of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_space()
  }

  /// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r'). (for reference)<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = " \n ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_multi_space_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_multi_space_ref()
  }

  /// Returns a [StaticParser] that parses spaces containing newlines (' ', '\t', '\n', '\r'). (for reference)<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = " \n ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_multi_space_ref_static().of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Clone + Debug + 'a, {
    StaticParsersImpl::elm_pred_ref(|e: &I| e.is_ascii_multi_space())
  }

  /// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r').<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = " \n ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_multi_space().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_multi_space()
  }

  /// Returns a [StaticParser] that parses spaces containing newlines (' ', '\t', '\n', '\r').<br/>
  /// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = " \n ";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_multi_space_static().of_many1().map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_pred(|e: &I| e.is_ascii_multi_space())
  }

  /// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').(for reference)<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcxyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_alpha_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_alpha_ref()
  }

  /// Returns a [StaticParser] that parses alphabets ('A'..='Z', 'a'..='z').(for reference)<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcxyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_alpha_ref_static().of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Clone + Debug + 'a, {
    StaticParsersImpl::elm_alpha_ref()
  }

  /// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcxyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_alpha().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_alpha()
  }

  /// Returns a [StaticParser] that parses alphabets ('A'..='Z', 'a'..='z').<br/>
  /// 英字('A'..='Z', 'a'..='z')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcxyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_alpha_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_alpha()
  }

  /// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').(for reference)<br/>
  /// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc0123xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_alpha_digit_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_alpha_digit_ref()
  }

  /// Returns a [StaticParser] that parses alphabets and digits ('A'..='Z', 'a'..='z', '0'..='9').(for reference)<br/>
  /// 英数字('A'..='Z', 'a'..='z', '0'..='9')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc123";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_alpha_digit_ref_static().of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_alpha_digit_ref()
  }

  /// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').<br/>
  /// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc0123xyz";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_alpha_digit().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_alpha_digit()
  }

  /// Returns a [StaticParser] that parses alphabets and digits ('A'..='Z', 'a'..='z', '0'..='9').<br/>
  /// 英数字('A'..='Z', 'a'..='z', '0'..='9')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc123";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_alpha_digit_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_alpha_digit()
  }

  /// Returns a [Parser] that parses digits ('0'..='9').(for reference)<br/>
  /// 数字('0'..='9')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_digit_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_digit_ref()
  }

  /// Returns a [StaticParser] that parses digits ('0'..='9').(for reference)<br/>
  /// 数字('0'..='9')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_digit_ref_static().of_many1().map(|chars| {
  ///   chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_digit_ref()
  }

  /// Returns a [Parser] that parses digits ('0'..='9').<br/>
  /// 数字('0'..='9')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_digit().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_digit()
  }

  /// Returns a [StaticParser] that parses digits ('0'..='9').<br/>
  /// 数字('0'..='9')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_digit_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_digit()
  }

  /// Returns a [Parser] that parses digits ('1'..='9').(for reference)<br/>
  /// 数字('1'..='9')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_digit().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_digit_ref().with_filter_not(|c: &&I| c.is_ascii_digit_zero())
  }

  /// Returns a [StaticParser] that parses digits ('1'..='9').(for reference)<br/>
  /// 数字('1'..='9')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_digit_1_9_ref_static().of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c: &char| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9_ref_static<'a>() -> StaticParser<'a, char, &'a [char]> {
    StaticParsersImpl::elm_ref_in('1', '9')
  }

  /// Returns a [Parser] that parses digits ('1'..='9').<br/>
  /// 数字('1'..='9')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_digit_1_9().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_digit_1_9_ref().map(Clone::clone)
  }

  /// Returns a [StaticParser] that parses digits ('1'..='9').<br/>
  /// 数字('1'..='9')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "123456789";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_digit_1_9_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9_static<'a>() -> StaticParser<'a, char, Vec<char>> {
    StaticParsersImpl::elm_in('1', '9')
  }

  /// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').(for reference)<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789ABCDEFabcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_hex_digit_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_hex_digit_ref()
  }

  /// Returns a [StaticParser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').(for reference)<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789ABCDEFabcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_hex_digit_ref_static().of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_hex_digit_ref()
  }

  /// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789ABCDEFabcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_hex_digit().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    ParsersImpl::elm_hex_digit()
  }

  /// Returns a [StaticParser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').<br/>
  /// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "0123456789ABCDEFabcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_hex_digit_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_hex_digit()
  }

  /// Returns a [Parser] that parses oct digits ('0'..='8').(for reference)<br/>
  /// 8進の数字('0'..='8')を解析する[Parser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "012345678";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_oct_digit_ref().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    ParsersImpl::elm_oct_digit_ref()
  }

  /// Returns a [StaticParser] that parses oct digits ('0'..='8').(for reference)<br/>
  /// 8進の数字('0'..='8')を解析する[StaticParser]を返します。(参照版)
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "012345678";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_oct_digit_ref_static().of_many1().map(|chars| chars.iter().flat_map(|slice| slice.iter().map(|c| *c)).collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_oct_digit_ref()
  }

  /// Returns a [Parser] that parses oct digits ('0'..='8').<br/>
  /// 8進の数字('0'..='8')を解析する[Parser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "012345678";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = elm_oct_digit().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    ParsersImpl::elm_oct_digit()
  }

  /// Returns a [StaticParser] that parses oct digits ('0'..='8').<br/>
  /// 8進の数字('0'..='8')を解析する[StaticParser]を返します。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "012345678";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = elm_oct_digit_static().of_many1().map(|chars| {
  ///   chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>()
  /// });
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit_static<'a, I>() -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + Clone + Debug + 'a, {
    StaticParsersImpl::elm_oct_digit()
  }

  // --- Elements Parsers ---

  /// Returns a [Parser] that parses a sequence of elements.<br/>
  /// 要素の列を解析する[Parser]を返す。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc";
  /// let input = text.as_bytes();
  ///
  /// let parser: Parser<u8, &str> = seq(b"abc").collect().map_res(std::str::from_utf8);
  ///
  /// let result: ParseResult<u8, &str> = parser.parse(input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn seq<'a, 'b, I>(seq: &'b [I]) -> Parser<'a, I, Vec<I>>
  where
    I: PartialEq + Debug + Clone + 'a,
    'b: 'a, {
    ParsersImpl::seq(seq)
  }

  /// Returns a [StaticParser] that parses a sequence of elements.<br/>
  /// 要素の列を解析する[StaticParser]を返す。
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc";
  /// let input = text.as_bytes();
  ///
  /// let parser: StaticParser<u8, String> = seq_static(b"abc").collect().map(|v: Vec<Vec<u8>>| String::from_utf8_lossy(&v[0]).to_string());
  ///
  /// let result: ParseResult<u8, String> = parser.parse(input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text.to_string());
  /// ```
  pub fn seq_static<'a, 'b, I>(seq: &'b [I]) -> StaticParser<'a, I, Vec<I>>
  where
    I: Element + PartialEq + Debug + Clone + 'a + 'static,
    'b: 'a, {
    StaticParsersImpl::seq(seq)
  }

  /// Returns a [Parser] that parses a string.<br/>
  /// 文字列を解析する[Parser]を返す。
  ///
  /// - tag: a string
  /// - tag: 文字列
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = tag("abc");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag<'a, 'b>(tag: &'b str) -> Parser<'a, char, String>
  where
    'b: 'a, {
    ParsersImpl::tag(tag)
  }

  /// Returns a [StaticParser] that parses a string.<br/>
  /// 文字列を解析する[StaticParser]を返す。
  ///
  /// - tag: a string
  /// - tag: 文字列
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = tag_static("abc");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag_static<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
  where
    'b: 'a, {
    StaticParsersImpl::tag(tag)
  }

  /// Returns a [Parser] that parses a string. However, it is not case-sensitive.<br/>
  /// 文字列を解析する[Parser]を返す。ただし大文字小文字を区別しない。
  ///
  /// - tag: a string
  /// - tag: 文字列
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = tag("abc");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag_no_case<'a, 'b>(tag: &'b str) -> Parser<'a, char, String>
  where
    'b: 'a, {
    ParsersImpl::tag_no_case(tag)
  }

  /// Returns a [StaticParser] that parses a string. However, it is not case-sensitive.<br/>
  /// 文字列を解析する[StaticParser]を返す。ただし大文字小文字を区別しない。
  ///
  /// - tag: a string
  /// - tag: 文字列
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = tag_no_case_static("ABC");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag_no_case_static<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
  where
    'b: 'a, {
    StaticParsersImpl::tag_no_case(tag)
  }

  /// Helper function for lazy_static tests
  /// This is used to avoid lifetime issues with lazy_static
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// fn value<'a>() -> StaticParser<'a, char, String> {
  ///   lazy_static_parser()
  /// }
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser = value();
  /// let result = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn lazy_static_parser<'a>() -> StaticParser<'a, char, String> {
    StaticParsersImpl::lazy_static_parser().map(|s| s.to_string())
  }

  /// Helper function for lazy_static tests
  /// This is used to avoid lifetime issues with lazy_static
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = lazy_static_str("abc");
  /// let result = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc".to_string());
  /// ```
  pub fn lazy_static_str<'a>(s: &'a str) -> StaticParser<'a, char, String> {
    StaticParsersImpl::tag(s)
  }

  /// Returns a [Parser] that parses a string that match a regular expression.<br/>
  /// 正規表現に合致する文字列を解析する[Parser]を返す。
  ///
  /// - pattern: a regular expression
  /// - pattern: 正規表現
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = regex("[abc]+");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn regex<'a>(pattern: &str) -> Parser<'a, char, String> {
    ParsersImpl::regex(pattern)
  }

  /// Returns a [StaticParser] that parses a string using a regular expression.<br/>
  /// 正規表現を使用して文字列を解析する[StaticParser]を返す。
  ///
  /// - pattern: a regular expression pattern
  /// - pattern: 正規表現パターン
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc123";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = regex_static(r"[a-z]+");
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn regex_static<'a>(pattern: &'a str) -> StaticParser<'a, char, String> {
    StaticParsersImpl::regex(pattern)
  }

  /// Returns a [Parser] that returns an element of the specified length.<br/>
  /// 指定された長さの要素を返す[Parser]を返す。
  ///
  /// - n: Length of the reading element
  /// - n: 読む要素の長さ
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take(3).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn take<'a, I>(n: usize) -> Parser<'a, I, &'a [I]> {
    ParsersImpl::take(n)
  }

  /// Returns a [StaticParser] that returns an element of the specified length.<br/>
  /// 指定された長さの要素を返す[StaticParser]を返す。
  ///
  /// - n: Length of the reading element
  /// - n: 読む要素の長さ
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_static(3).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn take_static<'a, I>(n: usize) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take(n)
  }

  /// クロージャの結果が真である間は要素を返す[Parser]を返す。<br/>
  /// Returns a [Parser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さは必須ではありません。<br/>
  /// The length of the analysis result is not required.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while0(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while0(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "");
  /// ```
  pub fn take_while0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while0(f)
  }

  /// クロージャの結果が真である間は要素を返す[StaticParser]を返す。<br/>
  /// Returns a [StaticParser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さは必須ではありません。<br/>
  /// The length of the analysis result is not required.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while0_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while0_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "");
  /// ```
  pub fn take_while0_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take_while0(f)
  }

  /// クロージャの結果が真である間は要素を返す[Parser]を返す。<br/>
  /// Returns a [Parser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さは1要素以上必要です。<br/>
  /// The length of the analysis result must be at least one element.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while1(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while1(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
  /// ```
  pub fn take_while1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while1(f)
  }

  /// クロージャの結果が真である間は要素を返す[StaticParser]を返す。<br/>
  /// Returns a [StaticParser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さは1要素以上必要です。<br/>
  /// The length of the analysis result must be at least one element.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while1_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while1_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
  /// ```
  pub fn take_while1_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take_while1(f)
  }

  /// クロージャの結果が真である間は要素を返す[Parser]を返す。<br/>
  /// Returns a [Parser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さはn要素以上m要素以下である必要があります。<br/>
  /// The length of the analysis result should be between n and m elements.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while_n_m(1, 3, |e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_while_n_m(1, 3, |e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
  /// ```
  pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while_n_m(n, m, f)
  }

  /// クロージャの結果が真である間は要素を返す[StaticParser]を返す。<br/>
  /// Returns a [StaticParser] that returns elements, while the result of the closure is true.
  ///
  /// 解析結果の長さはn要素以上m要素以下である必要があります。<br/>
  /// The length of the analysis result should be between n and m elements.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while_n_m_static(1, 3, |e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_while_n_m_static(1, 3, |e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(|chars| chars.into_iter().collect::<String>());
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
  /// ```
  pub fn take_while_n_m_static<'a, I, F>(n: usize, m: usize, f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take_while_n_m(n, m, f)
  }

  /// Returns a [Parser] that returns a sequence up to either the end element or the element that matches the condition.<br/>
  /// 条件に一致する要素もしくは最後の要素までの連続を返す[Parser]を返す。
  ///
  /// 解析結果の長さは1要素以上必要です。<br/>
  /// The length of the analysis result must be at least one element.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_till0(|e| matches!(*e, 'c')).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_till0(|e| matches!(*e, 'c')).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "def");
  /// ```
  pub fn take_till0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till0(f)
  }

  /// Returns a [StaticParser] that returns a sequence up to and including the element that matches the condition.<br/>
  /// 条件に一致する要素を含む連続を返す[StaticParser]を返す。
  ///
  /// 解析結果の長さは0要素以上必要です。<br/>
  /// The length of the analysis result can be zero or more elements.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_till0_static(|e| matches!(*e, 'c')).map(|chars| String::from_iter(chars.iter().map(|c| *c)));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_till0_static(|e| matches!(*e, 'c')).map(|chars| String::from_iter(chars.iter().map(|c| *c)));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "def");
  /// ```
  pub fn take_till0_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take_till0(f)
  }

  /// Returns a [Parser] that returns a sequence up to either the end element or the element that matches the condition.<br/>
  /// 条件に一致する要素もしくは最後の要素までの連続を返す[Parser]を返す。
  ///
  /// 解析結果の長さは1要素以上必要です。<br/>
  /// The length of the analysis result must be at least one element.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_till1(|e| matches!(*e, 'c')).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = take_till1(|e| matches!(*e, 'c')).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
  /// ```
  pub fn take_till1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till1(f)
  }

  /// Returns a [StaticParser] that returns a sequence up to and including the element that matches the condition.<br/>
  /// 条件に一致する要素を含む連続を返す[StaticParser]を返す。
  ///
  /// 解析結果の長さは1要素以上必要です。<br/>
  /// The length of the analysis result must be at least one element.
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_till1_static(|e| matches!(*e, 'c')).map(|chars| String::from_iter(chars.iter().map(|c| *c)));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "ab");
  /// ```
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "def";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, String> = take_till1_static(|e| matches!(*e, 'c')).map(|chars| String::from_iter(chars.iter().map(|c| *c)));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// ```
  pub fn take_till1_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::take_till1(f)
  }

  // --- Offset Control Parsers ---

  /// Returns a [Parser] that skips the specified number of elements.<br/>
  /// 指定された数の要素をスキップする[Parser]を返す。
  ///
  /// - size: a size of elements
  /// - size: スキップする要素数
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = (skip(3) * tag("def"));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "def");
  /// ```
  pub fn skip<'a, I>(n: usize) -> Parser<'a, I, ()> {
    ParsersImpl::skip(n)
  }

  /// Returns a [StaticParser] that skips the specified number of elements.<br/>
  /// 指定された数の要素をスキップする[StaticParser]を返す。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abcdef";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, ()> = skip_static(3);
  ///
  /// let result: ParseResult<char, ()> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), ());
  /// ```
  pub fn skip_static<'a, I>(n: usize) -> StaticParser<'a, I, ()> {
    StaticParsersImpl::skip(n)
  }

  // --- Enhanced Parsers ---

  /// Return a [Parser] that skips the previous and following [Parser]s.<br/>
  /// 前後のパーサーをスキップするパーサーを返す。
  ///
  /// - lp: 左側のパーサー
  /// - parser: 中央のパーサー
  /// - rp: 右側のパーサー
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "(abc)";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: Parser<char, String> = surround(elm('('), tag("abc"), elm(')'));
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
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

  /// Returns a [StaticParser] that parses the body surrounded by open and close.<br/>
  /// openとcloseに囲まれたbodyを解析する[StaticParser]を返す。
  ///
  /// # Example
  ///
  /// ```rust
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "(abc)";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// let parser: StaticParser<char, &[char]> = surround_static(
  ///   elm_ref_static('('),
  ///   take_static(3),
  ///   elm_ref_static(')'),
  /// );
  ///
  /// let result: ParseResult<char, &[char]> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[1..4]);
  /// ```
  pub fn surround_static<'a, I, A, B, C>(
    lp: StaticParser<'a, I, A>,
    parser: StaticParser<'a, I, B>,
    rp: StaticParser<'a, I, C>,
  ) -> StaticParser<'a, I, &'a [I]>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a,
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::surround(lp, parser, rp)
  }

  /// Returns a [Parser] that lazily evaluates the specified [Parser].<br/>
  /// 指定した[Parser]を遅延評価する[Parser]を返す。
  ///
  /// - f: Function to generate parser
  /// - f: パーサーを生成する関数
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// fn value<'a>() -> Parser<'a, char, String> {
  ///   tag("abc")
  /// }
  /// let parser: Parser<char, String> = lazy(value);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> Parser<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a, {
    ParsersImpl::lazy(f)
  }

  /// Returns a [StaticParser] that lazily evaluates the specified [StaticParser].<br/>
  /// 指定した[StaticParser]を遅延評価する[StaticParser]を返す。
  ///
  /// - f: Function to generate parser
  /// - f: パーサーを生成する関数
  ///
  /// # Example
  ///
  /// ```rust
  /// use std::iter::FromIterator;
  /// use oni_comb_parser_rs::prelude::*;
  ///
  /// let text: &str = "abc";
  /// let input = text.chars().collect::<Vec<_>>();
  ///
  /// fn value<'a>() -> StaticParser<'a, char, String> {
  ///   tag_static("abc").map(|s| String::from_iter(s.chars()))
  /// }
  /// let parser: StaticParser<char, String> = lazy_static(value);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc".to_string());
  /// ```
  pub fn lazy_static<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> StaticParser<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    StaticParsersImpl::lazy(f)
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

  #[test]
  fn test_elm_alpha_digit_ref_static() {
    init();
    {
      let text = "abc123";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_alpha_digit_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-alphanumeric characters
      {
        let input2 = "abc123!@#".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "abc123");
      }
    }
  }

  #[test]
  fn test_elm_alpha_digit_static() {
    init();
    {
      let text = "abc123";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_alpha_digit_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-alphanumeric characters
      {
        let input2 = "abc123!@#".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "abc123");
      }
    }
  }

  #[test]
  fn test_elm_alpha_ref_static() {
    init();
    {
      let text = "abc";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_alpha_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-alphabetic characters
      {
        let input2 = "abc123".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "abc");
      }

      // Test with error case
      {
        let input3 = "123abc".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_alpha_static() {
    init();
    {
      let text = "abc";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_alpha_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-alphabetic characters
      {
        let input2 = "abc123".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "abc");
      }

      // Test with error case
      {
        let input3 = "123abc".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_any_ref_static() {
    init();
    {
      let input = "a".chars().collect::<Vec<_>>();
      let p = elm_any_ref_static();

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result[0], 'a');

      // Test with empty input
      let input2: Vec<char> = vec![];
      let result2 = p.parse_as_result(&input2);
      assert!(result2.is_err());
    }
  }

  #[test]
  fn test_elm_any_static() {
    init();
    {
      let input = "a".chars().collect::<Vec<_>>();
      let p = elm_any_static();

      let result = p.parse_as_result(&input).unwrap();
      let result_char = result[0];
      assert_eq!(result_char, 'a');

      // Test with empty input
      let input2: Vec<char> = vec![];
      let result2 = p.parse_as_result(&input2);
      assert!(result2.is_err());
    }
  }

  #[test]
  fn test_elm_digit_1_9_ref_static() {
    init();
    {
      let text = "123456789";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_digit_1_9_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with zero digit (should not be parsed)
      {
        let input2 = "0123456789".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with non-digit characters
      {
        let input3 = "123abc".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "123");
      }
    }
  }

  #[test]
  fn test_elm_digit_1_9_static() {
    init();
    {
      let text = "123456789";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_digit_1_9_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with zero digit (should not be parsed)
      {
        let input2 = "0123456789".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with non-digit characters
      {
        let input3 = "123abc".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "123");
      }
    }
  }

  #[test]
  fn test_elm_digit_ref_static() {
    init();
    {
      let text = "0123456789";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_digit_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-digit characters
      {
        let input2 = "123abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "123");
      }

      // Test with error case
      {
        let input3 = "abc123".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_digit_static() {
    init();
    {
      let text = "0123456789";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_digit_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-digit characters
      {
        let input2 = "123abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "123");
      }

      // Test with error case
      {
        let input3 = "abc123".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_from_until_ref_static() {
    init();
    {
      let input = "abcdefg".chars().collect::<Vec<_>>();
      let p = elm_from_until_ref_static('a', 'c').of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters outside the range
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_from_until_static() {
    init();
    {
      let input = "abcdefg".chars().collect::<Vec<_>>();
      let p = elm_from_until_static('a', 'c')
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters outside the range
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_hex_digit_ref_static() {
    init();
    {
      let text = "0123456789abcdefABCDEF";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_hex_digit_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-hex characters
      {
        let input2 = "0123456789abcdefABCDEFghijkl".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "0123456789abcdefABCDEF");
      }

      // Test with error case
      {
        let input3 = "ghijkl0123456789abcdefABCDEF".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_hex_digit_static() {
    init();
    {
      let text = "0123456789abcdefABCDEF";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_hex_digit_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, text);

      // Test with non-hex characters
      {
        let input2 = "0123456789abcdefABCDEFghijkl".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "0123456789abcdefABCDEF");
      }

      // Test with error case
      {
        let input3 = "ghijkl0123456789abcdefABCDEF".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_in_ref_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let p = elm_in_ref_static('a', 'c').of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not in the set
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_in_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let p = elm_in_static('a', 'c')
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not in the set
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_multi_space_ref_static() {
    init();
    {
      let input = "   abc".chars().collect::<Vec<_>>();
      let p = elm_multi_space_ref_static().map(|chars| chars.iter().map(|c| *c).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "   ");

      // Test with no spaces
      {
        let input2 = "abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "");
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "");
      }
    }
  }

  #[test]
  fn test_elm_multi_space_static() {
    init();
    {
      let input = "   abc".chars().collect::<Vec<_>>();
      let p = elm_multi_space_static().map(|chars| chars.into_iter().collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "   ");

      // Test with no spaces
      {
        let input2 = "abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result2, "");
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "");
      }
    }
  }

  #[test]
  fn test_elm_oct_digit_ref_static() {
    init();
    {
      let input = "01234567abc".chars().collect::<Vec<_>>();
      let p = elm_oct_digit_ref_static().of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "01234567");

      // Test with non-octal digits
      {
        let input2 = "89abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_oct_digit_static() {
    init();
    {
      let input = "01234567abc".chars().collect::<Vec<_>>();
      let p = elm_oct_digit_static()
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "01234567");

      // Test with non-octal digits
      {
        let input2 = "89abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_of_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let set = |c: &char| *c == 'a' || *c == 'b' || *c == 'c';
      let p = elm_of_static(&set)
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not matching the predicate
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_pred_ref_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let pred = |c: &char| *c == 'a' || *c == 'b' || *c == 'c';
      let p = elm_pred_ref_static(&pred).of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not matching the predicate
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_pred_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let pred = |c: &char| *c == 'a' || *c == 'b' || *c == 'c';
      let p = elm_pred_static(&pred)
        .of_many1()
        .map(|chars| chars.into_iter().flat_map(|v| v.into_iter()).collect::<String>());

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not matching the predicate
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_ref_of_static() {
    init();
    {
      let input = "abcdef".chars().collect::<Vec<_>>();
      let set = |c: &char| *c == 'a' || *c == 'b' || *c == 'c';
      let p = elm_ref_of_static(&set).of_many1().map(|chars| {
        chars
          .iter()
          .flat_map(|slice| slice.iter().map(|c| *c))
          .collect::<String>()
      });

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "abc");

      // Test with characters not matching the predicate
      {
        let input2 = "defg".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_ref_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();
      let p = elm_ref_static('a');

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result[0], 'a');

      // Test with non-matching character
      {
        let input2 = "def".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_space_ref_static() {
    init();
    {
      let input = " abc".chars().collect::<Vec<_>>();
      let p = elm_space_ref_static();

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(*result[0], ' ');

      // Test with non-space character
      {
        let input2 = "abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_space_static() {
    init();
    {
      let input = " abc".chars().collect::<Vec<_>>();
      let p = elm_space_static();

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, ' ');

      // Test with non-space character
      {
        let input2 = "abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_elm_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();
      let p = elm_static('a');

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, 'a');

      // Test with non-matching character
      {
        let input2 = "def".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with empty input
      {
        let input3: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }
    }
  }

  #[test]
  fn test_empty_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();
      let p = empty_static();

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, ());

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result2 = p.parse_as_result(&input2);
        assert_eq!(result2.unwrap(), ());
      }
    }
  }

  #[test]
  fn test_end_static() {
    init();
    {
      // Test with empty input - should succeed
      let input: Vec<char> = vec![];
      let p = end_static();

      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, ());

      // Test with non-empty input - should fail
      {
        let input2 = "abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }
    }
  }

  #[test]
  fn test_failed_lazy_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();

      {
        let counter = std::rc::Rc::new(std::cell::Cell::new(0));
        let counter_clone = counter.clone();
        let p = failed_lazy_static(move || {
          counter_clone.set(counter_clone.get() + 1);
          (
            ParseError::mismatch(format!("error message: {}", counter_clone.get())),
            CommittedStatus::Uncommitted,
          )
        });

        // The error message should be evaluated lazily
        assert_eq!(counter.get(), 0);

        let result = p.parse_as_result(&input);
        assert!(result.is_err());

        // The error message should be evaluated now
        assert_eq!(counter.get(), 1);

        // Check the error message
        match result {
          Err(ParseError::Mismatch { message, .. }) => {
            assert_eq!(message, "error message: 1");
          }
          _ => panic!("Expected Mismatch error"),
        }

        // Parsing again should evaluate the error message again
        let result2 = p.parse_as_result(&input);
        assert!(result2.is_err());
        assert_eq!(counter.get(), 2);
      }
    }
  }

  #[test]
  fn test_failed_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();
      let p = failed_static("error message");

      let result = p.parse_as_result(&input);
      assert!(result.is_err());

      // Check the error message
      match result {
        Err(ParseError::Mismatch { message, .. }) => {
          assert_eq!(message, "error message");
        }
        _ => panic!("Expected Mismatch error"),
      }

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }
    }
  }

  #[test]
  fn test_failed_with_commit_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();

      // Create a parser that fails with commit
      let p1 = failed_with_commit_static("error message");

      // Create a parser that succeeds
      let p2 = elm_static('a');

      // Combine them with or - since p1 commits, p2 should not be tried
      let p = p1.or(p2);

      let result = p.parse_as_result(&input);
      assert!(result.is_err());

      // Check the error message
      match result {
        Err(ParseError::Mismatch { message, .. }) => {
          assert_eq!(message, "error message");
        }
        _ => panic!("Expected Mismatch error"),
      }

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result2 = p1.parse_as_result(&input2);
        assert!(result2.is_err());
      }
    }
  }

  #[test]
  fn test_failed_with_uncommit_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();

      // Create a parser that fails with uncommit
      let p1 = failed_with_uncommit_static("error message");

      // Create a parser that succeeds
      let p2 = elm_static('a');

      // Combine them with or - since p1 uncommits, p2 should be tried and succeed
      let p = p1.or(p2);

      let result = p.parse_as_result(&input);
      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 'a');

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result2 = p1.parse_as_result(&input2);
        assert!(result2.is_err());
      }
    }
  }

  #[test]
  fn test_lazy_static() {
    init();

    {
      let input = "abc".chars().collect::<Vec<_>>();

      {
        // Create a counter to verify lazy evaluation
        let counter = std::rc::Rc::new(std::cell::Cell::new(0));
        let counter_clone = counter.clone();

        // Create a parser that will be lazily evaluated
        let p = lazy_static(move || {
          counter_clone.set(counter_clone.get() + 1);
          elm_static('a')
        });

        // The parser should not be evaluated yet
        assert_eq!(counter.get(), 0);

        // Parse the input
        let result = p.parse_as_result(&input).unwrap();
        assert_eq!(result, 'a');

        // The parser should be evaluated now
        assert_eq!(counter.get(), 1);

        // Parse again to verify the parser is evaluated each time
        let result2 = p.parse_as_result(&input).unwrap();
        assert_eq!(result2, 'a');
        assert_eq!(counter.get(), 2);

        // Test with non-matching input
        {
          let input2 = "def".chars().collect::<Vec<_>>();
          let result3 = p.parse_as_result(&input2);
          assert!(result3.is_err());
          assert_eq!(counter.get(), 3);
        }
      }
    }
  }

  #[test]
  fn test_none_of_static() {
    init();
    {
      let input = "abc".chars().collect::<Vec<_>>();

      // Create a parser that matches any character except 'b' or 'c'
      let p = none_of_static(&['b', 'c']);

      // Should match 'a'
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, 'a');

      // Should not match 'b'
      {
        let input2 = "bcd".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Should not match 'c'
      {
        let input3 = "cde".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }

      // Should match 'd'
      {
        let input4 = "def".chars().collect::<Vec<_>>();
        let result4 = p.parse_as_result(&input4).unwrap();
        assert_eq!(result4, 'd');
      }

      // Should fail with empty input
      {
        let input5: Vec<char> = vec![];
        let result5 = p.parse_as_result(&input5);
        assert!(result5.is_err());
      }
    }
  }

  #[test]
  fn test_none_ref_of_static() {
    init();
    let input = "abc".chars().collect::<Vec<_>>();

    // Create a parser that matches any character except 'b' or 'c' and returns a reference
    let p = none_ref_of_static(&['b', 'c']);

    // Should match 'a' and return a reference to it
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(*result, 'a');

    // Should not match 'b'
    {
      let input2 = "bcd".chars().collect::<Vec<_>>();
      let result2 = p.parse_as_result(&input2);
      assert!(result2.is_err());
    }

    // Should not match 'c'
    {
      let input3 = "cde".chars().collect::<Vec<_>>();
      let result3 = p.parse_as_result(&input3);
      assert!(result3.is_err());
    }

    // Should match 'd' and return a reference to it
    {
      let input4 = "def".chars().collect::<Vec<_>>();
      let result4 = p.parse_as_result(&input4).unwrap();
      assert_eq!(*result4, 'd');
    }

    // Should fail with empty input
    {
      let input5: Vec<char> = vec![];
      let result5 = p.parse_as_result(&input5);
      assert!(result5.is_err());
    }
  }

  #[test]
  fn test_regex_static() {
    init();
    let input = "abc123def".chars().collect::<Vec<_>>();

    // Create a parser that matches digits using regex
    let p = regex_static(r"[0-9]+");

    // Test with input that has digits in the middle
    let result = p.parse(&input[3..]).unwrap();
    assert_eq!(result, "123");

    // Test with input that starts with digits
    {
      let input2 = "123abc".chars().collect::<Vec<_>>();
      let result2 = p.parse(&input2).unwrap();
      assert_eq!(result2, "123");
    }

    // Test with input that doesn't have digits
    {
      let input3 = "abc".chars().collect::<Vec<_>>();
      let result3 = p.parse(&input3);
      assert!(result3.is_err());
    }

    // Test with empty input
    {
      let input4: Vec<char> = vec![];
      let result4 = p.parse(&input4);
      assert!(result4.is_err());
    }

    // Test with more complex regex
    {
      let p2 = regex_static(r"[a-z]+[0-9]+");
      let input5 = "abc123def".chars().collect::<Vec<_>>();
      let result5 = p2.parse(&input5).unwrap();
      assert_eq!(result5, "abc123");
    }
  }

  #[test]
  fn test_seq_static() {
    init();
    let input = "abc".chars().collect::<Vec<_>>();

    // Create parsers for individual characters
    let p1 = elm_static('a');
    let p2 = elm_static('b');
    let p3 = elm_static('c');

    // Sequence them together
    let p = seq_static(p1, p2, p3);

    // Test successful parsing
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, 'c'); // seq returns the result of the last parser

    // Test with partial match
    {
      let input2 = "ab".chars().collect::<Vec<_>>();
      let result2 = p.parse_as_result(&input2);
      assert!(result2.is_err());
    }

    // Test with non-matching input
    {
      let input3 = "def".chars().collect::<Vec<_>>();
      let result3 = p.parse_as_result(&input3);
      assert!(result3.is_err());
    }

    // Test with empty input
    {
      let input4: Vec<char> = vec![];
      let result4 = p.parse_as_result(&input4);
      assert!(result4.is_err());
    }

    // Test with map to transform the result
    {
      let p_mapped = seq_static(p1, p2, p3).map(|c| c.to_ascii_uppercase());
      let result5 = p_mapped.parse_as_result(&input).unwrap();
      assert_eq!(result5, 'C');
    }
  }

  #[test]
  fn test_skip_static() {
    init();
    let input = "abcdef".chars().collect::<Vec<_>>();

    // Create a parser that skips the first 3 characters and then matches 'd'
    let p = skip_static(3).and_then(elm_static('d'));

    // Test successful parsing
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, 'd');

    // Test with insufficient input length
    {
      let input2 = "ab".chars().collect::<Vec<_>>();
      let result2 = p.parse_as_result(&input2);
      assert!(result2.is_err());
    }

    // Test with empty input
    {
      let input3: Vec<char> = vec![];
      let result3 = p.parse_as_result(&input3);
      assert!(result3.is_err());
    }

    // Test with non-matching character after skip
    {
      let input4 = "abcxyz".chars().collect::<Vec<_>>();
      let result4 = p.parse_as_result(&input4);
      assert!(result4.is_err());
    }

    // Test with skip(0)
    {
      let p2 = skip_static(0).and_then(elm_static('a'));
      let result5 = p2.parse_as_result(&input).unwrap();
      assert_eq!(result5, 'a');
    }
  }

  #[test]
  fn test_successful_lazy_static() {
    init();

    {
      let input = "abc".chars().collect::<Vec<_>>();

      // Create a counter to verify lazy evaluation
      // Use Rc<Cell<i32>> to allow sharing between closure and test code
      let counter = std::rc::Rc::new(std::cell::Cell::new(0));
      let counter_clone = counter.clone();

      // Create a parser that lazily evaluates a successful value
      let p = successful_lazy_static(move || {
        counter_clone.set(counter_clone.get() + 1);
        "result"
      });

      // The parser should not be evaluated yet
      assert_eq!(counter.get(), 0);

      // Parse the input
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "result");

      // The parser should be evaluated now
      assert_eq!(counter.get(), 1);

      // Parse again to verify the parser is evaluated each time
      let result2 = p.parse_as_result(&input).unwrap();
      assert_eq!(result2, "result");
      assert_eq!(counter.get(), 2);

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result3, "result");
        assert_eq!(counter.get(), 3);
      }
    }

    // Test with map to transform the result
    {
      let input = "abc".chars().collect::<Vec<_>>();

      let counter = std::rc::Rc::new(std::cell::Cell::new(0));
      let counter_clone = counter.clone();
      let p_mapped = successful_lazy_static(move || {
        counter_clone.set(counter_clone.get() + 1);
        "result"
      })
      .map(|s| s.to_uppercase());

      let result4 = p_mapped.parse_as_result(&input).unwrap();
      assert_eq!(result4, "RESULT");
      assert_eq!(counter.get(), 1);
    }
  }

  #[test]
  fn test_successful_static() {
    init();

    {
      let input = "abc".chars().collect::<Vec<_>>();

      // Create a parser that always succeeds with a specified value
      let p = successful_static("result");

      // Parse the input
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "result");

      // Parse again to verify the result is the same
      let result2 = p.parse_as_result(&input).unwrap();
      assert_eq!(result2, "result");

      // Test with empty input
      {
        let input2: Vec<char> = vec![];
        let result3 = p.parse_as_result(&input2).unwrap();
        assert_eq!(result3, "result");
      }

      // Test with map to transform the result
      {
        let p_mapped = successful_static("result").map(|s| s.to_uppercase());
        let result4 = p_mapped.parse_as_result(&input).unwrap();
        assert_eq!(result4, "RESULT");
      }

      // Test with different input types
      {
        let p_int = successful_static(42);
        let result5 = p_int.parse_as_result(&input).unwrap();
        assert_eq!(result5, 42);
      }

      // Test with complex type
      {
        #[derive(Debug, PartialEq)]
        struct TestStruct {
          value: i32,
        }

        let test_struct = TestStruct { value: 42 };
        let p_struct = successful_static(test_struct);
        let result6 = p_struct.parse_as_result(&input).unwrap();
        assert_eq!(result6, TestStruct { value: 42 });
      }
    }
  }

  #[test]
  fn test_surround_static() {
    init();

    {
      let input = "(abc)".chars().collect::<Vec<_>>();

      // Create parsers for opening, content, and closing
      let open_parser = elm_static('(');
      let content_parser = elm_static('a').and_then(elm_static('b')).and_then(elm_static('c'));
      let close_parser = elm_static(')');

      // Create a parser that surrounds the content with opening and closing parsers
      let p = surround_static(open_parser, content_parser, close_parser);

      // Test successful parsing
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, 'c'); // surround returns the result of the content parser

      // Test with missing closing character
      {
        let input2 = "(abc".chars().collect::<Vec<_>>();
        let result2 = p.parse_as_result(&input2);
        assert!(result2.is_err());
      }

      // Test with missing opening character
      {
        let input3 = "abc)".chars().collect::<Vec<_>>();
        let result3 = p.parse_as_result(&input3);
        assert!(result3.is_err());
      }

      // Test with non-matching content
      {
        let input4 = "(xyz)".chars().collect::<Vec<_>>();
        let result4 = p.parse_as_result(&input4);
        assert!(result4.is_err());
      }

      // Test with empty input
      {
        let input5: Vec<char> = vec![];
        let result5 = p.parse_as_result(&input5);
        assert!(result5.is_err());
      }

      // Test with map to transform the result
      {
        let p_mapped = surround_static(open_parser, content_parser, close_parser).map(|c| c.to_ascii_uppercase());
        let result6 = p_mapped.parse_as_result(&input).unwrap();
        assert_eq!(result6, 'C');
      }

      // Test with different opening and closing characters
      {
        let input7 = "[abc]".chars().collect::<Vec<_>>();
        let open_parser2 = elm_static('[');
        let close_parser2 = elm_static(']');
        let p2 = surround_static(open_parser2, content_parser, close_parser2);
        let result7 = p2.parse_as_result(&input7).unwrap();
        assert_eq!(result7, 'c');
      }
    }
  }

  #[test]
  fn test_tag_no_case_static() {
    init();

    // Test with lowercase input matching lowercase tag
    {
      let input1 = "hello".chars().collect::<Vec<_>>();
      let p1 = tag_no_case_static("hello");
      let result1 = p1.parse_as_result(&input1).unwrap();
      assert_eq!(result1, "hello");
    }

    // Test with uppercase input matching lowercase tag
    {
      let input2 = "HELLO".chars().collect::<Vec<_>>();
      let p2 = tag_no_case_static("hello");
      let result2 = p2.parse_as_result(&input2).unwrap();
      assert_eq!(result2, "HELLO");
    }

    // Test with mixed case input matching lowercase tag
    {
      let input3 = "HeLLo".chars().collect::<Vec<_>>();
      let p3 = tag_no_case_static("hello");
      let result3 = p3.parse_as_result(&input3).unwrap();
      assert_eq!(result3, "HeLLo");
    }

    // Test with lowercase input matching uppercase tag
    {
      let input4 = "hello".chars().collect::<Vec<_>>();
      let p4 = tag_no_case_static("HELLO");
      let result4 = p4.parse_as_result(&input4).unwrap();
      assert_eq!(result4, "hello");
    }

    // Test with non-matching input
    {
      let input5 = "world".chars().collect::<Vec<_>>();
      let p5 = tag_no_case_static("hello");
      let result5 = p5.parse_as_result(&input5);
      assert!(result5.is_err());
    }

    // Test with partial matching input
    {
      let input6 = "hel".chars().collect::<Vec<_>>();
      let p6 = tag_no_case_static("hello");
      let result6 = p6.parse_as_result(&input6);
      assert!(result6.is_err());
    }

    // Test with empty input
    {
      let input7: Vec<char> = vec![];
      let p7 = tag_no_case_static("hello");
      let result7 = p7.parse_as_result(&input7);
      assert!(result7.is_err());
    }

    // Test with empty tag
    {
      let input8 = "hello".chars().collect::<Vec<_>>();
      let p8 = tag_no_case_static("");
      let result8 = p8.parse_as_result(&input8).unwrap();
      assert_eq!(result8, "");
    }

    // Test with map to transform the result
    {
      let input9 = "hello".chars().collect::<Vec<_>>();
      let p9 = tag_no_case_static("hello").map(|s| format!("{}!", s));
      let result9 = p9.parse_as_result(&input9).unwrap();
      assert_eq!(result9, "hello!");
    }
  }

  #[test]
  fn test_tag_static() {
    init();

    // Test with matching input
    {
      let input1 = "hello".chars().collect::<Vec<_>>();
      let p1 = tag_static("hello");
      let result1 = p1.parse_as_result(&input1).unwrap();
      assert_eq!(result1, "hello");
    }

    // Test with case-sensitive mismatch (unlike tag_no_case_static)
    {
      let input2 = "HELLO".chars().collect::<Vec<_>>();
      let p2 = tag_static("hello");
      let result2 = p2.parse_as_result(&input2);
      assert!(result2.is_err());
    }

    // Test with mixed case mismatch
    {
      let input3 = "HeLLo".chars().collect::<Vec<_>>();
      let p3 = tag_static("hello");
      let result3 = p3.parse_as_result(&input3);
      assert!(result3.is_err());
    }

    // Test with uppercase tag matching uppercase input
    {
      let input4 = "HELLO".chars().collect::<Vec<_>>();
      let p4 = tag_static("HELLO");
      let result4 = p4.parse_as_result(&input4).unwrap();
      assert_eq!(result4, "HELLO");
    }

    // Test with non-matching input
    {
      let input5 = "world".chars().collect::<Vec<_>>();
      let p5 = tag_static("hello");
      let result5 = p5.parse_as_result(&input5);
      assert!(result5.is_err());
    }

    // Test with partial matching input
    {
      let input6 = "hel".chars().collect::<Vec<_>>();
      let p6 = tag_static("hello");
      let result6 = p6.parse_as_result(&input6);
      assert!(result6.is_err());
    }

    // Test with empty input
    {
      let input7: Vec<char> = vec![];
      let p7 = tag_static("hello");
      let result7 = p7.parse_as_result(&input7);
      assert!(result7.is_err());
    }

    // Test with empty tag
    {
      let input8 = "hello".chars().collect::<Vec<_>>();
      let p8 = tag_static("");
      let result8 = p8.parse_as_result(&input8).unwrap();
      assert_eq!(result8, "");
    }

    // Test with map to transform the result
    {
      let input9 = "hello".chars().collect::<Vec<_>>();
      let p9 = tag_static("hello").map(|s| format!("{}!", s));
      let result9 = p9.parse_as_result(&input9).unwrap();
      assert_eq!(result9, "hello!");
    }

    // Test with longer input than tag
    {
      let input10 = "hello world".chars().collect::<Vec<_>>();
      let p10 = tag_static("hello");
      let result10 = p10.parse(&input10);
      assert!(result10.is_ok());
      assert_eq!(result10.unwrap(), &input10[5..]);
    }
  }

  #[test]
  fn test_take_static() {
    init();
    let input = "abcdef".chars().collect::<Vec<_>>();

    // Test taking a specific number of elements
    let p = take_static(3);
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, "abc");

    // Test taking zero elements
    {
      let p_zero = take_static(0);
      let result_zero = p_zero.parse_as_result(&input).unwrap();
      assert_eq!(result_zero, "");
    }

    // Test taking all elements
    {
      let p_all = take_static(6);
      let result_all = p_all.parse_as_result(&input).unwrap();
      assert_eq!(result_all, "abcdef");
    }

    // Test taking more elements than available
    {
      let p_more = take_static(10);
      let result_more = p_more.parse_as_result(&input);
      assert!(result_more.is_err());
    }

    // Test with empty input
    {
      let empty_input: Vec<char> = vec![];
      let p_empty = take_static(0);
      let result_empty = p_empty.parse_as_result(&empty_input).unwrap();
      assert_eq!(result_empty, "");

      // Test with empty input and non-zero count
      let p_empty_nonzero = take_static(1);
      let result_empty_nonzero = p_empty_nonzero.parse_as_result(&empty_input);
      assert!(result_empty_nonzero.is_err());
    }

    // Test with map to transform the result
    {
      let p_mapped = take_static(3).map(|s| s.to_uppercase());
      let result_mapped = p_mapped.parse_as_result(&input).unwrap();
      assert_eq!(result_mapped, "ABC");
    }

    // Test with parse method to check remaining input
    {
      let p_remaining = take_static(3);
      let result_remaining = p_remaining.parse(&input);
      assert!(result_remaining.is_ok());
      assert_eq!(result_remaining.unwrap(), &input[3..]);
    }
  }

  #[test]
  fn test_take_till0_static() {
    init();
    let input = "abc123def".chars().collect::<Vec<_>>();

    // Test taking elements until a digit is encountered
    let p = take_till0_static(|c| c.is_digit(10));
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, "abc");

    // Test with input starting with the predicate character
    {
      let input2 = "123def".chars().collect::<Vec<_>>();
      let p2 = take_till0_static(|c| c.is_digit(10));
      let result2 = p2.parse_as_result(&input2).unwrap();
      assert_eq!(result2, ""); // Should return empty string since the first character satisfies the predicate
    }

    // Test with input not containing any predicate character
    {
      let input3 = "abcdef".chars().collect::<Vec<_>>();
      let p3 = take_till0_static(|c| c.is_digit(10));
      let result3 = p3.parse_as_result(&input3).unwrap();
      assert_eq!(result3, "abcdef"); // Should return the entire input
    }

    // Test with empty input
    {
      let input4: Vec<char> = vec![];
      let p4 = take_till0_static(|c| c.is_digit(10));
      let result4 = p4.parse_as_result(&input4).unwrap();
      assert_eq!(result4, ""); // Should return empty string for empty input
    }

    // Test with map to transform the result
    {
      let p5 = take_till0_static(|c| c.is_digit(10)).map(|s| s.to_uppercase());
      let result5 = p5.parse_as_result(&input).unwrap();
      assert_eq!(result5, "ABC");
    }

    // Test with parse method to check remaining input
    {
      let p6 = take_till0_static(|c| c.is_digit(10));
      let result6 = p6.parse(&input);
      assert!(result6.is_ok());
      assert_eq!(result6.unwrap(), &input[3..]); // Should return the remaining input after "abc"
    }

    // Test with a more complex predicate
    {
      let p7 = take_till0_static(|c| c == 'c' || c.is_digit(10));
      let result7 = p7.parse_as_result(&input).unwrap();
      assert_eq!(result7, "ab"); // Should stop at 'c'
    }
  }

  #[test]
  fn test_take_till1_static() {
    init();
    let input = "abc123def".chars().collect::<Vec<_>>();

    // Test taking elements until a digit is encountered
    let p = take_till1_static(|c| c.is_digit(10));
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, "abc");

    // Test with input starting with the predicate character
    {
      let input2 = "123def".chars().collect::<Vec<_>>();
      let p2 = take_till1_static(|c| c.is_digit(10));
      let result2 = p2.parse_as_result(&input2);
      assert!(result2.is_err()); // Should fail since it requires at least one element before the predicate
    }

    // Test with input not containing any predicate character
    {
      let input3 = "abcdef".chars().collect::<Vec<_>>();
      let p3 = take_till1_static(|c| c.is_digit(10));
      let result3 = p3.parse_as_result(&input3).unwrap();
      assert_eq!(result3, "abcdef"); // Should return the entire input
    }

    // Test with empty input
    {
      let input4: Vec<char> = vec![];
      let p4 = take_till1_static(|c| c.is_digit(10));
      let result4 = p4.parse_as_result(&input4);
      assert!(result4.is_err()); // Should fail since it requires at least one element
    }

    // Test with map to transform the result
    {
      let p5 = take_till1_static(|c| c.is_digit(10)).map(|s| s.to_uppercase());
      let result5 = p5.parse_as_result(&input).unwrap();
      assert_eq!(result5, "ABC");
    }

    // Test with parse method to check remaining input
    {
      let p6 = take_till1_static(|c| c.is_digit(10));
      let result6 = p6.parse(&input);
      assert!(result6.is_ok());
      assert_eq!(result6.unwrap(), &input[3..]); // Should return the remaining input after "abc"
    }

    // Test with a more complex predicate
    {
      let p7 = take_till1_static(|c| c == 'c' || c.is_digit(10));
      let result7 = p7.parse_as_result(&input).unwrap();
      assert_eq!(result7, "ab"); // Should stop at 'c'
    }

    // Test with single character before predicate
    {
      let input8 = "a123def".chars().collect::<Vec<_>>();
      let p8 = take_till1_static(|c| c.is_digit(10));
      let result8 = p8.parse_as_result(&input8).unwrap();
      assert_eq!(result8, "a"); // Should return just the single character
    }
  }

  #[test]
  fn test_take_while0_static() {
    init();
    {
      let input = "123abc".chars().collect::<Vec<_>>();

      // Test taking elements while a predicate is satisfied
      let p = take_while0_static(|c| c.is_digit(10));
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "123");

      // Test with input not starting with a character that satisfies the predicate
      {
        let input2 = "abc123".chars().collect::<Vec<_>>();
        let p2 = take_while0_static(|c| c.is_digit(10));
        let result2 = p2.parse_as_result(&input2).unwrap();
        assert_eq!(result2, ""); // Should return empty string since no characters satisfy the predicate initially
      }

      // Test with input containing only characters that satisfy the predicate
      {
        let input3 = "12345".chars().collect::<Vec<_>>();
        let p3 = take_while0_static(|c| c.is_digit(10));
        let result3 = p3.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "12345"); // Should return the entire input
      }

      // Test with empty input
      {
        let input4: Vec<char> = vec![];
        let p4 = take_while0_static(|c| c.is_digit(10));
        let result4 = p4.parse_as_result(&input4).unwrap();
        assert_eq!(result4, ""); // Should return empty string for empty input
      }

      // Test with map to transform the result
      {
        let p5 = take_while0_static(|c| c.is_digit(10)).map(|s| format!("{}!", s));
        let result5 = p5.parse_as_result(&input).unwrap();
        assert_eq!(result5, "123!");
      }

      // Test with parse method to check remaining input
      {
        let p6 = take_while0_static(|c| c.is_digit(10));
        let result6 = p6.parse(&input);
        assert!(result6.is_ok());
        assert_eq!(result6.unwrap(), &input[3..]); // Should return the remaining input after "123"
      }

      // Test with a more complex predicate
      {
        let p7 = take_while0_static(|c| c.is_digit(10) || c == 'a');
        let result7 = p7.parse_as_result(&input).unwrap();
        assert_eq!(result7, "123a"); // Should include 'a' as well
      }

      // Test with single character satisfying predicate
      {
        let input8 = "1abc".chars().collect::<Vec<_>>();
        let p8 = take_while0_static(|c| c.is_digit(10));
        let result8 = p8.parse_as_result(&input8).unwrap();
        assert_eq!(result8, "1"); // Should return just the single character
      }
    }
  }

  #[test]
  fn test_take_while1_static() {
    init();
    {
      let input = "123abc".chars().collect::<Vec<_>>();

      // Test taking elements while a predicate is satisfied
      let p = take_while1_static(|c| c.is_digit(10));
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "123");

      // Test with input not starting with a character that satisfies the predicate
      {
        let input2 = "abc123".chars().collect::<Vec<_>>();
        let p2 = take_while1_static(|c| c.is_digit(10));
        let result2 = p2.parse_as_result(&input2);
        assert!(result2.is_err()); // Should fail since no characters satisfy the predicate initially
      }

      // Test with input containing only characters that satisfy the predicate
      {
        let input3 = "12345".chars().collect::<Vec<_>>();
        let p3 = take_while1_static(|c| c.is_digit(10));
        let result3 = p3.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "12345"); // Should return the entire input
      }

      // Test with empty input
      {
        let input4: Vec<char> = vec![];
        let p4 = take_while1_static(|c| c.is_digit(10));
        let result4 = p4.parse_as_result(&input4);
        assert!(result4.is_err()); // Should fail since it requires at least one element
      }

      // Test with map to transform the result
      {
        let p5 = take_while1_static(|c| c.is_digit(10)).map(|s| format!("{}!", s));
        let result5 = p5.parse_as_result(&input).unwrap();
        assert_eq!(result5, "123!");
      }

      // Test with parse method to check remaining input
      {
        let p6 = take_while1_static(|c| c.is_digit(10));
        let result6 = p6.parse(&input);
        assert!(result6.is_ok());
        assert_eq!(result6.unwrap(), &input[3..]); // Should return the remaining input after "123"
      }

      // Test with a more complex predicate
      {
        let p7 = take_while1_static(|c| c.is_digit(10) || c == 'a');
        let result7 = p7.parse_as_result(&input).unwrap();
        assert_eq!(result7, "123a"); // Should include 'a' as well
      }

      // Test with single character satisfying predicate
      {
        let input8 = "1abc".chars().collect::<Vec<_>>();
        let p8 = take_while1_static(|c| c.is_digit(10));
        let result8 = p8.parse_as_result(&input8).unwrap();
        assert_eq!(result8, "1"); // Should return just the single character
      }
    }
  }

  #[test]
  fn test_take_while_n_m_static() {
    init();
    {
      let input = "12345abc".chars().collect::<Vec<_>>();

      // Test taking elements with min and max constraints
      let p = take_while_n_m_static(2, 4, |c| c.is_digit(10));
      let result = p.parse_as_result(&input).unwrap();
      assert_eq!(result, "1234"); // Should take at most 4 digits

      // Test with fewer elements than min
      {
        let input2 = "1abc".chars().collect::<Vec<_>>();
        let p2 = take_while_n_m_static(2, 4, |c| c.is_digit(10));
        let result2 = p2.parse_as_result(&input2);
        assert!(result2.is_err()); // Should fail since there's only 1 digit (less than min 2)
      }

      // Test with exactly min elements
      {
        let input3 = "12abc".chars().collect::<Vec<_>>();
        let p3 = take_while_n_m_static(2, 4, |c| c.is_digit(10));
        let result3 = p3.parse_as_result(&input3).unwrap();
        assert_eq!(result3, "12"); // Should take exactly 2 digits
      }

      // Test with more elements than max but stopping at max
      {
        let input4 = "123456abc".chars().collect::<Vec<_>>();
        let p4 = take_while_n_m_static(2, 4, |c| c.is_digit(10));
        let result4 = p4.parse_as_result(&input4).unwrap();
        assert_eq!(result4, "1234"); // Should take at most 4 digits
      }

      // Test with empty input
      {
        let input5: Vec<char> = vec![];
        let p5 = take_while_n_m_static(2, 4, |c| c.is_digit(10));
        let result5 = p5.parse_as_result(&input5);
        assert!(result5.is_err()); // Should fail since there are no elements
      }

      // Test with min=0 and empty input
      {
        let input6: Vec<char> = vec![];
        let p6 = take_while_n_m_static(0, 4, |c| c.is_digit(10));
        let result6 = p6.parse_as_result(&input6).unwrap();
        assert_eq!(result6, ""); // Should succeed with empty result
      }

      // Test with min=0 and non-matching input
      {
        let input7 = "abc".chars().collect::<Vec<_>>();
        let p7 = take_while_n_m_static(0, 4, |c| c.is_digit(10));
        let result7 = p7.parse_as_result(&input7).unwrap();
        assert_eq!(result7, ""); // Should succeed with empty result
      }

      // Test with map to transform the result
      {
        let p8 = take_while_n_m_static(2, 4, |c| c.is_digit(10)).map(|s| format!("{}!", s));
        let result8 = p8.parse_as_result(&input).unwrap();
        assert_eq!(result8, "1234!");
      }

      // Test with parse method to check remaining input
      {
        let p9 = take_while_n_m_static(2, 4, |c| c.is_digit(10));
        let result9 = p9.parse(&input);
        assert!(result9.is_ok());
        assert_eq!(result9.unwrap(), &input[4..]); // Should return the remaining input after "1234"
      }

      // Test with min=max
      {
        let p10 = take_while_n_m_static(3, 3, |c| c.is_digit(10));
        let result10 = p10.parse_as_result(&input).unwrap();
        assert_eq!(result10, "123"); // Should take exactly 3 digits
      }
    }
  }

  #[test]
  fn test_unit_static() {
    init();
    let input = "abc".chars().collect::<Vec<_>>();

    // Test with a simple value
    let p = unit_static(42);
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, 42);

    // Test with a different type
    let p2 = unit_static("hello");
    let result2 = p2.parse_as_result(&input).unwrap();
    assert_eq!(result2, "hello");

    // Test with a complex type
    let p3 = unit_static(vec![1, 2, 3]);
    let result3 = p3.parse_as_result(&input).unwrap();
    assert_eq!(result3, vec![1, 2, 3]);

    // Test with empty input
    let empty_input: Vec<char> = vec![];
    let p4 = unit_static(42);
    let result4 = p4.parse_as_result(&empty_input).unwrap();
    assert_eq!(result4, 42);

    // Test with map to transform the result
    let p5 = unit_static(42).map(|n| n * 2);
    let result5 = p5.parse_as_result(&input).unwrap();
    assert_eq!(result5, 84);

    // Test with parse method to check remaining input
    let p6 = unit_static(42);
    let result6 = p6.parse(&input);
    assert!(result6.is_ok());
    assert_eq!(result6.unwrap(), &input[..]); // Should return the entire input unchanged
  }

  #[test]
  fn test_lazy_static_parser() {
    init();
    let input = "abc".chars().collect::<Vec<_>>();

    // Test with a simple parser
    let p = lazy_static_parser(|| tag_static("abc"));
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, "abc");

    // Test with a different parser
    let p2 = lazy_static_parser(|| take_static(2));
    let result2 = p2.parse_as_result(&input).unwrap();
    assert_eq!(result2, "ab");

    // Test with a parser that fails
    let p3 = lazy_static_parser(|| tag_static("def"));
    let result3 = p3.parse_as_result(&input);
    assert!(result3.is_err());

    // Test with empty input
    let empty_input: Vec<char> = vec![];
    let p4 = lazy_static_parser(|| tag_static(""));
    let result4 = p4.parse_as_result(&empty_input).unwrap();
    assert_eq!(result4, "");

    // Test with map to transform the result
    let p5 = lazy_static_parser(|| tag_static("abc")).map(|s| s.to_uppercase());
    let result5 = p5.parse_as_result(&input).unwrap();
    assert_eq!(result5, "ABC");

    // Test with parse method to check remaining input
    let p6 = lazy_static_parser(|| tag_static("a"));
    let result6 = p6.parse(&input);
    assert!(result6.is_ok());
    assert_eq!(result6.unwrap(), &input[1..]); // Should return the remaining input after "a"
  }

  #[test]
  fn test_lazy_static_str() {
    init();
    let input = "abc".chars().collect::<Vec<_>>();

    // Test with a simple parser
    let p = lazy_static_str("abc");
    let result = p.parse_as_result(&input).unwrap();
    assert_eq!(result, "abc");

    // Test with a different string
    let p2 = lazy_static_str("ab");
    let result2 = p2.parse_as_result(&input).unwrap();
    assert_eq!(result2, "ab");

    // Test with a string that doesn't match
    let p3 = lazy_static_str("def");
    let result3 = p3.parse_as_result(&input);
    assert!(result3.is_err());

    // Test with empty input
    let empty_input: Vec<char> = vec![];
    let p4 = lazy_static_str("");
    let result4 = p4.parse_as_result(&empty_input).unwrap();
    assert_eq!(result4, "");

    // Test with map to transform the result
    let p5 = lazy_static_str("abc").map(|s| s.to_uppercase());
    let result5 = p5.parse_as_result(&input).unwrap();
    assert_eq!(result5, "ABC");

    // Test with parse method to check remaining input
    let p6 = lazy_static_str("a");
    let result6 = p6.parse(&input);
    assert!(result6.is_ok());
    assert_eq!(result6.unwrap(), &input[1..]); // Should return the remaining input after "a"
  }

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
    let patterns_static: &'static _ = Box::leak(Box::new(patterns.clone()));
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = elm_of(patterns_static);

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
    let patterns_static: &'static _ = Box::leak(Box::new(patterns.clone()));
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = none_of(patterns_static);

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

    //   let result = p.parse_as_result(b"def");
    //  assert!(result.is_ok());
  }

  #[test]
  fn test_take_till1() {
    init();
    let p = take_till1(|c| *c == b'c').map_res(std::str::from_utf8);

    let result = p.parse_as_result(b"abcd");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc");

    // let result = p.parse_as_result(b"def");
    // assert!(result.is_err());
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
    let p = (pa + pb + (pc | pd)).map(|((a, b), c)| {
      let mut result = String::new();
      result.push(*a);
      result.push(*b);
      result.push(*c);
      result
    });

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
    let p = (elm_ref('a') + tag("name")).map(|(a, name_chars)| {
      let mut result = String::new();
      result.push(*a);
      for c in name_chars.chars() {
        result.push(c);
      }
      result
    });

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
