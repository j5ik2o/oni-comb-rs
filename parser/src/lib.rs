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
  use crate::internal::static_parsers_impl::StaticParsersImpl;
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, &char> = elm_any_ref_static();
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  pub fn elm_any_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, char> = elm_any_static();
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  pub fn elm_any_static<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, &char> = elm_ref_static('x');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  pub fn elm_ref_static<'a, I>(element: I) -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, char> = elm_static('x');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  pub fn elm_static<'a, I>(element: I) -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
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
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, &char> = elm_pred_ref_static(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, &char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), &input[0]);
  /// ```
  pub fn elm_pred_ref_static<'a, I, F>(f: F) -> StaticParser<'a, I, &'a I>
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
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, char> = elm_pred_static(|c| *c == 'x');
  ///
  /// let result: ParseResult<char, char> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), input[0]);
  /// ```
  pub fn elm_pred_static<'a, I, F>(f: F) -> StaticParser<'a, I, I>
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
  /// let parser: StaticParser<char, String> = elm_ref_of_static("xyz").of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_ref_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    // Use a predicate-based approach for all sets
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && set.contains(&input[offset]) {
        ParseResult::successful(&input[offset], 1)
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
  pub fn elm_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
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
  /// let parser: StaticParser<char, String> = elm_of_static("xyz").of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, I>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    // Use a predicate-based approach for all sets
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && set.contains(&input[offset]) {
        ParseResult::successful(input[offset].clone(), 1)
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
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_in_ref_static('x', 'z').of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in_ref_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
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
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_in_static('x', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_in_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
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
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_from_until_ref_static('w', 'z').of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until_ref_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
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
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_from_until_static('w', 'z').of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_from_until_static<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
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
  pub fn none_ref_of<'a, I, S>(set: &'a S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
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
  /// let parser: StaticParser<char, String> = none_ref_of_static("abc").of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_ref_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    // Use a predicate-based approach for all sets
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && !set.contains(&input[offset]) {
        ParseResult::successful(&input[offset], 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("element in excluded set: {:?}", input[offset]);
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
  pub fn none_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
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
  /// let parser: StaticParser<char, String> = none_of_static("abc").of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn none_of_static<'a, I, S>(set: &'a S) -> StaticParser<'a, I, I>
  where
    I: Element + PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    // Use a predicate-based approach for all sets
    StaticParser::new(move |parse_state| {
      let input: &[I] = parse_state.input();
      let offset = parse_state.next_offset();
      if offset < input.len() && !set.contains(&input[offset]) {
        ParseResult::successful(input[offset].clone(), 1)
      } else if offset >= input.len() {
        let msg = format!("unexpected end of input");
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        let msg = format!("element in excluded set: {:?}", input[offset]);
        let pe = ParseError::of_mismatch(input, offset, 0, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    })
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_space_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_space_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_space_static<'a, I>() -> StaticParser<'a, I, I>
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_multi_space_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + Debug + 'a, {
    StaticParsersImpl::elm_multi_space_ref()
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_multi_space_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_multi_space_static<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    StaticParsersImpl::elm_multi_space()
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_alpha_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_alpha_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_static<'a, I>() -> StaticParser<'a, I, I>
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_alpha_digit_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_alpha_digit_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_alpha_digit_static<'a, I>() -> StaticParser<'a, I, I>
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_digit_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_digit_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_static<'a, I>() -> StaticParser<'a, I, I>
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_digit_1_9_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9_ref_static<'a>() -> StaticParser<'a, char, &'a char> {
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_digit_1_9_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_digit_1_9_static<'a>() -> StaticParser<'a, char, char> {
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_hex_digit_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + Clone + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_hex_digit_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_hex_digit_static<'a, I>() -> StaticParser<'a, I, I>
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
    I: Element + PartialEq + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_oct_digit_ref_static().of_many1().collect().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit_ref_static<'a, I>() -> StaticParser<'a, I, &'a I>
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
    I: Element + PartialEq + Clone + 'a, {
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
  /// let parser: StaticParser<char, String> = elm_oct_digit_static().of_many1().map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn elm_oct_digit_static<'a, I>() -> StaticParser<'a, I, I>
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
  pub fn seq<'a, 'b, I>(seq: &'b [I]) -> Parser<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
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
  /// let parser: StaticParser<u8, &str> = seq_static(b"abc").collect().map_res(std::str::from_utf8);
  ///
  /// let result: ParseResult<u8, &str> = parser.parse(input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), text);
  /// ```
  pub fn seq_static<'a, 'b, I>(seq: &'b [I]) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a,
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
  /// let parser: Parser<char, &str> = tag("abc");
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
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
  /// let parser: Parser<char, &str> = tag("abc");
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn tag_no_case<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
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
  /// let s = lazy_static_str("abc");
  /// assert_eq!(s, "abc");
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
  /// let parser: StaticParser<char, String> = take_static(3).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_while0_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_while1_static(|e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_while_n_m_static(1, 3, |e| match *e {
  ///  'a'..='c' => true,
  ///   _ => false
  /// }).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_till0_static(|e| matches!(*e, 'c')).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_till0_static(|e| matches!(*e, 'c')).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_till1_static(|e| matches!(*e, 'c')).map(String::from_iter);
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
  /// let parser: StaticParser<char, String> = take_till1_static(|e| matches!(*e, 'c')).map(String::from_iter);
  ///
  /// let result: ParseResult<char, String> = parser.parse(&input);
  ///
  /// assert!(result.is_failure());
  /// assert_eq!(result.failure().unwrap(), ParseError::of_in_complete());
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
  /// let parser: Parser<char, &str> = (skip(3) * tag("def"));
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
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
  /// let parser: Parser<char, &str> = surround(elm('('), tag("abc"), elm(')'));
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
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
  ///   elm_static('('),
  ///   take_static(3),
  ///   elm_static(')'),
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
  ) -> StaticParser<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
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
  /// fn value<'a>() -> Parser<'a, char, &'a str> {
  ///   tag("abc")
  /// }
  /// let parser: Parser<char, &str> = lazy(value);
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> Parser<'a, I, A> + 'a,
    A: Debug + 'a, {
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
  /// fn value<'a>() -> StaticParser<'a, char, &'a str> {
  ///   tag_static("abc").map(|s| s.as_str())
  /// }
  /// let parser: StaticParser<char, &str> = lazy_static_str(value);
  ///
  /// let result: ParseResult<char, &str> = parser.parse(&input);
  ///
  /// assert!(result.is_success());
  /// assert_eq!(result.success().unwrap(), "abc");
  /// ```
  pub fn lazy_static<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> StaticParser<'a, I, A> + 'a,
    A: Debug + 'a, {
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
