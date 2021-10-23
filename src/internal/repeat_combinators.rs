use crate::core::{ParseError, ParseResult, Parser, ParserRunner};
use crate::extension::parsers::RepeatCombinators;
use crate::internal::ParsersImpl;
use crate::utils::{Bound, RangeArgument};
use std::fmt::Debug;
use std::rc::Rc;

impl RepeatCombinators for ParsersImpl {
  fn repeat_sep<'a, I, A, B, R>(
    pa: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| {
      let mut ps = Rc::clone(&parse_state);
      let mut all_length = 0;
      let mut items = vec![];

      if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
        ps = Rc::new(ps.add_offset(length));
        items.push(get);
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
            if let ParseResult::Success { length, .. } = sep.run(Rc::clone(&ps)) {
              ps = Rc::new(ps.add_offset(length));
              all_length += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
            ps = Rc::new(ps.add_offset(length));
            items.push(get);
            all_length += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let ps = ps.add_offset(all_length);
          let pe = ParseError::of_mismatch(
            ps.input(),
            ps.last_offset().unwrap_or(0),
            format!(
              "expect repeat at least {} times, found {} times",
              min_count,
              items.len()
            ),
          );
          return ParseResult::failed_with_un_commit(pe);
        }
      }
      ParseResult::successful(items, all_length)
    })
  }
}
