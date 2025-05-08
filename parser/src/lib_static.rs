

use std::fmt::{Debug, Display};
use crate::core::{CommittedStatus, Element, ParseError};
use crate::internal::{StaticParsersImpl};
use crate::prelude::Set;
pub use crate::StaticParser;
use crate::core::ParserFunctor;

/// Returns a [Parser] that does nothing.<br/>
/// 何もしない[Parser]を返します。
///
/// # Example
///
/// ```rust
/// # use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "a";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = unit();
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), ());
/// ```
pub fn unit<'a, I>() -> StaticParser<'a, I, ()> {
    StaticParsersImpl::unit()
}

/// Returns a [Parser] that does nothing. It is an alias for `unit()`.<br/>
/// 何もしない[Parser]を返します。`unit()`のエイリアスです。
///
/// # Example
///
/// ```rust
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "a";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = empty();
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), ());
/// ```
pub fn empty<'a, I>() -> StaticParser<'a, I, ()> {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "a";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = end();
///
/// let result = parser.parse(&input).to_result();
///
/// assert!(result.is_err());
/// ```
pub fn end<'a, I>() -> StaticParser<'a, I, ()>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = successful('a');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), 'a');
/// ```
pub fn successful<'a, I, A>(value: A) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = successful_lazy(|| 'a');
///
/// let result: ParseResult<char, char> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), 'a');
/// ```
pub fn successful_lazy<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parse_error = ParseError::of_in_complete();
///
/// let parser = failed(parse_error.clone(), CommittedStatus::Committed);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_failure());
/// assert_eq!(result.failure().unwrap(), parse_error);
/// ```
pub fn failed<'a, I, A>(value: ParseError<'a, I>, commit: CommittedStatus) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parse_error = ParseError::of_in_complete();
///
/// let parser = failed_with_commit(parse_error.clone());
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_failure());
/// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Committed);
///
/// assert_eq!(result.failure().unwrap(), parse_error);
/// ```
pub fn failed_with_commit<'a, I, A>(value: ParseError<'a, I>) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parse_error = ParseError::of_in_complete();
///
/// let parser = failed_with_uncommit(parse_error.clone());
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_failure());
/// assert_eq!(result.committed_status().unwrap(), CommittedStatus::Uncommitted);
///
/// assert_eq!(result.failure().unwrap(), parse_error);
/// ```
pub fn failed_with_uncommit<'a, I, A>(value: ParseError<'a, I>) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parse_error = ParseError::of_in_complete();
///
/// let parser = failed_lazy(|| (parse_error.clone(), CommittedStatus::Committed));
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_failure());
/// assert_eq!(result.failure().unwrap(), parse_error);
/// ```
pub fn failed_lazy<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_any_ref();
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), &input[0]);
/// ```
pub fn elm_any_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + Clone + 'a + 'static, {
    StaticParsersImpl::elm_any_ref()
}

/// Returns a [Parser] that parses an any element.<br/>
/// 任意の要素を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_any();
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), input[0]);
/// ```
pub fn elm_any<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + PartialEq + Clone + 'a + 'static, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref('x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), &input[0]);
/// ```
pub fn elm_ref<'a, I>(element: I) -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + Clone + 'a + 'static, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm('x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), input[0]);
/// ```
pub fn elm<'a, I>(element: I) -> StaticParser<'a, I, I>
where
    I: Element + PartialEq + Clone + 'a + 'static, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_pred_ref(|c| *c == 'x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), &input[0]);
/// ```
pub fn elm_pred_ref<'a, I, F>(f: F) -> StaticParser<'a, I, &'a I>
where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + PartialEq + Clone + 'a + 'static, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "x";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_pred(|c| *c == 'x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), input[0]);
/// ```
pub fn elm_pred<'a, I, F>(f: F) -> StaticParser<'a, I, I>
where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + Clone + PartialEq + 'a + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref_of("xyz").of_many1().map(|chars| chars.into_iter().map(|c| *c).collect::<String>());
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_ref_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + Clone + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    StaticParsersImpl::elm_ref_of(set)
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_of("xyz").of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, I>
where
    I: Element + PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    StaticParsersImpl::elm_of(set)
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_in_ref('x', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_in_ref<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + PartialOrd + Display + Clone + Debug + 'a + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_in('x', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_in<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
where
    I: Element + PartialEq + PartialOrd + Display + Clone + Debug + 'a + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "wxy";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_from_until_ref('w', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_from_until_ref<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
where
    I: Element + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "wxy";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_from_until('w', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_from_until<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
where
    I: Element + 'a + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = none_ref_of("abc").of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn none_ref_of<'a, I, S>(set: &'a S) -> StaticParser<'a, I, &'a I>
where
    I: Element + 'a,
    S: Set<I> + ?Sized + 'a, {
    StaticParsersImpl::none_ref_of(set)
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = none_of("abc").of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn none_of<'a, I, S>(set: &'a S) -> StaticParser<'a, I, I>
where
    I: Element + 'a,
    S: Set<I> + ?Sized + 'a, {
    StaticParsersImpl::none_of(set)
}

/// Returns a [Parser] that parses the space (' ', '\t'). (for reference)<br/>
/// スペース(' ', '\t')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "   ";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_space_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_space_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_space_ref()
}


/// Returns a [Parser] that parses the space (' ', '\t').<br/>
/// スペース(' ', '\t')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "   ";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_space().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_space<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_space()
}


/// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r'). (for reference)<br/>
/// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = " \n ";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_multi_space_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_multi_space_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_multi_space_ref()
}



/// Returns a [Parser] that parses spaces containing newlines (' ', '\t', '\n', '\r').<br/>
/// 改行を含むスペース(' ', '\t', '\n', '\r')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = " \n ";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_multi_space().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_multi_space<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_multi_space()
}

/// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').(for reference)<br/>
/// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcxyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_alpha_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_alpha_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_alpha_ref()
}


/// Returns a [Parser] that parses alphabets ('A'..='Z', 'a'..='z').<br/>
/// 英字('A'..='Z', 'a'..='z')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcxyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_alpha().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_alpha<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_alpha()
}

/// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').(for reference)<br/>
/// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abc0123xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_alpha_digit_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_alpha_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_alpha_digit_ref()
}


/// Returns a [Parser] that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').<br/>
/// 英数字('0'..='9', 'A'..='Z', 'a'..='z')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abc0123xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_alpha_digit().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_alpha_digit<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_alpha_digit()
}


/// Returns a [Parser] that parses digits ('0'..='9').(for reference)<br/>
/// 数字('0'..='9')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "0123456789";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_digit_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + 'a + 'static, {
    StaticParsersImpl::elm_digit_ref()
}


/// Returns a [Parser] that parses digits ('0'..='9').<br/>
/// 数字('0'..='9')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "0123456789";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_digit().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_digit<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_digit()
}


/// Returns a [Parser] that parses digits ('1'..='9').(for reference)<br/>
/// 数字('1'..='9')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "123456789";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_digit().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_digit_1_9_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + 'a + 'static, {
    elm_digit_ref().with_filter_not(|c: &&I| c.is_ascii_digit_zero())
}

/// Returns a [Parser] that parses digits ('1'..='9').<br/>
/// 数字('1'..='9')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "123456789";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_digit_1_9().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_digit_1_9<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + 'a + 'static, {
    elm_digit_1_9_ref().map(Clone::clone)
}



/// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').(for reference)<br/>
/// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "0123456789ABCDEFabcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_hex_digit_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_hex_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_hex_digit_ref()
}


/// Returns a [Parser] that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').<br/>
/// 16進の数字('0'..='9', 'A'..='F', 'a'..='f')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "0123456789ABCDEFabcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_hex_digit().of_many1().map(String::from_iter);
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_hex_digit<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + 'a + 'static, {
    StaticParsersImpl::elm_hex_digit()
}

/// Returns a [Parser] that parses oct digits ('0'..='8').(for reference)<br/>
/// 8進の数字('0'..='8')を解析する[Parser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "012345678";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser: StaticParser<char, String> = elm_oct_digit_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_oct_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
where
    I: Element + PartialEq + 'a + 'static, {
    StaticParsersImpl::elm_oct_digit_ref()
}


/// Returns a [Parser] that parses oct digits ('0'..='8').<br/>
/// 8進の数字('0'..='8')を解析する[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use std::iter::FromIterator;
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
///
/// let text: &str = "012345678";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_oct_digit().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_oct_digit<'a, I>() -> StaticParser<'a, I, I>
where
    I: Element + PartialEq + Clone + 'a + 'static, {
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
/// use oni_comb_parser_rs::extension::parser::*;
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abc";
/// let input = text.as_bytes();
///
/// let parser = seq(b"abc").collect().map_res(std::str::from_utf8);
///
/// let result = parser.parse(input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn seq<'a, 'b, I>(seq: &'b [I]) -> StaticParser<'a, I, Vec<I>>
where
    I: Element + 'a,
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = tag("abc");
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn tag<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = tag("abc");
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn tag_no_case<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
where
    'b: 'a, {
    StaticParsersImpl::tag_no_case(tag)
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = regex("[abc]+");
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn regex<'a, S>(pattern: S) -> StaticParser<'a, char, String>
where
    S: AsRef<str> + 'a,{
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take(3).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take<'a, I>(n: usize) -> StaticParser<'a, I, &'a [I]>
where
    I: Element + 'a,
{
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while0(|e| match *e {
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
pub fn take_while0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
where
    F: Fn(&I) -> bool + 'a,
    I: Element + 'a, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while1(|e| match *e {
///  'a'..='c' => true,
///   _ => false
/// }).map(String::from_iter);
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_while1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
where
    F: Fn(&I) -> bool + 'a,
    I: Element + 'a, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while_n_m(1, 3, |e| match *e {
///  'a'..='c' => true,
///   _ => false
/// }).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> StaticParser<'a, I, &'a [I]>
where
    F: Fn(&I) -> bool + 'a,
    I: Element + 'a, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_till0(|e| matches!(*e, 'c')).map(String::from_iter);
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_till0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
where
    F: Fn(&I) -> bool + 'a,
    I: Element + 'a, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_till1(|e| matches!(*e, 'c')).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_till1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = skip(3) * tag("def");
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "def");
/// ```
pub fn skip<'a, I>(n: usize) -> StaticParser<'a, I, ()> {
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "(abc)";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = surround(elm('('), tag("abc"), elm(')'));
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn surround<'a, I, A, B, C>(
    lp: StaticParser<'a, I, A>,
    parser: StaticParser<'a, I, B>,
    rp: StaticParser<'a, I, C>,
) -> StaticParser<'a, I, B>
where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a,
    I: Element + 'a,{
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
/// use oni_comb_parser_rs::prelude_static::*;
///
/// let text: &str = "abc";
/// let input = text.chars().collect::<Vec<_>>();
///
/// fn value<'a>() -> StaticParser<'a, char, String> {
///   tag("abc")
/// }
/// let parser = lazy(value);
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn lazy<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
where
    F: Fn() -> StaticParser<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a, {
    StaticParsersImpl::lazy(f)
}

