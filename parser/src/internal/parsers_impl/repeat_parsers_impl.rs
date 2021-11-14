use crate::core::{ParsedError, ParsedResult, Parser, ParserRunner};
use crate::extension::parsers::RepeatParsers;
use crate::internal::ParsersImpl;
use crate::utils::{Bound, RangeArgument};
use std::fmt::Debug;

impl RepeatParsers for ParsersImpl {
  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| {
      let mut all_length = 0;
      let mut items = vec![];

      if let ParsedResult::Success { value, length } = parser.run(parse_state) {
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

          if let Some(sep) = &separator {
            if let ParsedResult::Success { length, .. } = sep.run(&current_parse_state) {
              current_parse_state = current_parse_state.add_offset(length);
              all_length += length;
            } else {
              break;
            }
          }
          if let ParsedResult::Success { value, length } = parser.run(&current_parse_state) {
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
          let pe = ParsedError::of_mismatch(
            ps.input(),
            ps.last_offset().unwrap_or(0),
            all_length,
            format!(
              "expect repeat at least {} times, found {} times",
              min_count,
              items.len()
            ),
          );
          return ParsedResult::failed_with_un_commit(pe);
        }
      }
      ParsedResult::successful(items, all_length)
    })
  }
}
