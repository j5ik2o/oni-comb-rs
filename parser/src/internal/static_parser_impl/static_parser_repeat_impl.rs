// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseError, ParseResult, StaticParser};
use crate::extension::parser::RepeatParser;
use crate::utils::{Bound, RangeArgument};
use std::fmt::Debug;

impl<'a, I, A: 'a> RepeatParser<'a> for StaticParser<'a, I, A> {
  fn repeat<R>(self, range: R) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    Self: Sized, {
    self.of_rep_sep(range, None as Option<StaticParser<'a, I, ()>>)
  }

  fn of_many0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    self.of_rep_sep(0.., None as Option<StaticParser<'a, I, ()>>)
  }

  fn of_many1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    self.of_rep_sep(1.., None as Option<StaticParser<'a, I, ()>>)
  }

  fn of_many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    self.of_rep_sep(n..=m, None as Option<StaticParser<'a, I, ()>>)
  }

  fn of_count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    self.of_rep_sep(n, None as Option<StaticParser<'a, I, ()>>)
  }

  fn of_rep_sep<B, R>(
    self,
    range: R,
    separator: Option<Self::P<'a, Self::Input, B>>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    let method = self.method.clone();
    let separator_clone = separator.clone();

    StaticParser::new(move |parse_state| {
      let mut all_length = 0;
      let mut items = vec![];

      if let ParseResult::Success { value, length } = (method)(parse_state) {
        let mut current_parse_state = parse_state.add_offset(length);
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
              current_parse_state = current_parse_state.add_offset(length);
              all_length += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success { value, length } = (method)(&current_parse_state) {
            current_parse_state = current_parse_state.add_offset(length);
            items.push(value);
            all_length += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let ps = parse_state.add_offset(all_length);
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

  fn of_many0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    self.of_rep_sep(0.., Some(separator))
  }

  fn of_many1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    self.of_rep_sep(1.., Some(separator))
  }

  fn of_many_n_m_sep<B>(
    self,
    n: usize,
    m: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    self.of_rep_sep(n..=m, Some(separator))
  }

  fn of_count_sep<B>(
    self,
    n: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    self.of_rep_sep(n, Some(separator))
  }
}
