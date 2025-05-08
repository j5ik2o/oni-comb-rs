use crate::core::{ParseError, ParseResult, Parser};
use crate::extension::parsers::RepeatParsers;
use crate::internal::ParsersImpl;
use crate::utils::{Bound, RangeArgument};
use std::fmt::Debug;

impl RepeatParsers for ParsersImpl {
  #[inline]
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    Self::repeat_sep::<'a, I, A, (), R>(parser, range, None)
  }

  #[inline]
  fn many0<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    Self::repeat_sep(parser, 0.., None as Option<Self::P<'a, I, ()>>)
  }

  #[inline]
  fn many1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    Self::repeat_sep(parser, 1.., None as Option<Self::P<'a, I, ()>>)
  }

  #[inline]
  fn count<'a, I, A>(parser: Self::P<'a, I, A>, count: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    Self::repeat_sep(parser, count..=count, None as Option<Self::P<'a, I, ()>>)
  }

  #[inline]
  fn many0_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    Self::repeat_sep(parser, 0.., Some(sep))
  }

  #[inline]
  fn many1_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    Self::repeat_sep(parser, 1.., Some(sep))
  }

  #[inline]
  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method = parser.method.clone();
    let separator_clone = separator.clone();

    Parser::new(move |parse_state| {
      let mut all_length = 0;
      let mut items = vec![];

      if let ParseResult::Success { value, length } = (method)(parse_state) {
        let mut current_parse_state = parse_state.advance_by(length);
        items.push(value);
        all_length += length;
        loop {
          match range.end() {
            Bound::Included(&max_count) => {
              if items.len() >= max_count {
                break;
              }
            }
            Bound::Excluded(&max_count) => {
              if items.len() + 1 >= max_count {
                break;
              }
            }
            _ => (),
          }

          if let Some(sep) = &separator_clone {
            if let ParseResult::Success { length, .. } = (sep.method)(&current_parse_state) {
              current_parse_state = current_parse_state.advance_by(length);
              all_length += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success { value, length } = (method)(&current_parse_state) {
            current_parse_state = current_parse_state.advance_by(length);
            items.push(value);
            all_length += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let ps = parse_state.advance_by(all_length);
          let pe = ParseError::of_mismatch(
            ps.input(),
            ps.last_offset().unwrap_or(0),
            all_length,
            format!(
              "expect repeat at least {} times, found {} times",
              min_count,
              items.len()
            ),
          );
          return ParseResult::failed_with_uncommitted(pe);
        }
      }
      ParseResult::successful(items, all_length)
    })
  }
}
