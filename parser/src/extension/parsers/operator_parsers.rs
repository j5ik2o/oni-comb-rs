use std::fmt::Debug;

use crate::core::{Parser, Parsers, StaticParser, StaticParsers};

pub trait OperatorParsers: Parsers {
  fn exists<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: Debug + 'a;

  fn not<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a;

  fn opt<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Option<A>>
  where
    A: Clone + Debug + 'a, {
    Self::or(Self::map(Self::attempt(parser), Some), Self::successful(None))
  }

  fn or<'a, I: Clone, A>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn and_then<'a, I: Clone, A, B>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn attempt<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn scan_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a;

  fn chain_right0<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a, {
    Self::or(Self::chain_right1(p, op), Self::successful(x.clone()))
  }

  fn chain_left0<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a, {
    Self::or(Self::chain_left1(p, op), Self::successful(x.clone()))
  }

  fn chain_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a, {
    Self::scan_right1(p, op)
  }

  fn chain_left1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a;

  fn rest_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a;

  fn rest_left1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a;
}

pub trait StaticOperatorParsers: StaticParsers {
  fn exists<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: Debug + 'a + 'static;

  fn not<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a + 'static;

  fn opt<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Option<A>>
  where
    A: Clone + Debug + 'a + 'static, {
    Self::or(Self::map(Self::attempt(parser), Some), Self::successful(None))
  }

  fn or<'a, I: Clone, A>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a + 'static;

  fn and_then<'a, I: Clone, A, B>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static;

  fn attempt<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a + 'static;

  fn scan_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static;

  fn chain_right0<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    Self::or(Self::chain_right1(p, op), Self::successful(x.clone()))
  }

  fn chain_left0<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    Self::or(Self::chain_left1(p, op), Self::successful(x.clone()))
  }

  fn chain_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    Self::scan_right1(p, op)
  }

  fn chain_left1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static;

  fn rest_right1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static;

  fn rest_left1<'a, I: Clone, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn exists<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, bool>
where
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::exists(parser)
}

pub fn not<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, ()>
where
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::not(parser)
}

pub fn opt<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Option<A>>
where
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::opt(parser)
}

pub fn or<'a, I: Clone, A>(parser1: Parser<'a, I, A>, parser2: Parser<'a, I, A>) -> Parser<'a, I, A>
where
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::or(parser1, parser2)
}

pub fn and_then<'a, I: Clone, A, B>(parser1: Parser<'a, I, A>, parser2: Parser<'a, I, B>) -> Parser<'a, I, (A, B)>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::and_then(parser1, parser2)
}

pub fn attempt<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::attempt(parser)
}

pub fn scan_right1<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::scan_right1(p, op)
}

pub fn chain_right0<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>, x: A) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::chain_right0(p, op, x)
}

pub fn chain_left0<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>, x: A) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::chain_left0(p, op, x)
}

pub fn chain_right1<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::chain_right1(p, op)
}

pub fn chain_left1<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::chain_left1(p, op)
}

pub fn rest_right1<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>, x: A) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::rest_right1(p, op, x)
}

pub fn rest_left1<'a, I: Clone, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>, x: A) -> Parser<'a, I, A>
where
  BOP: Fn(A, A) -> A + 'a + Clone,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as OperatorParsers>::rest_left1(p, op, x)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // ヘルパー関数
  pub fn successful<'a, I: Clone, A>(value: A) -> StaticParser<'a, I, A>
  where
    A: Clone + 'a + 'static, {
    StaticParser::new(move |_| crate::core::ParseResult::successful(value.clone(), 0))
  }

  pub fn map<'a, I: Clone, A, B, F>(parser: StaticParser<'a, I, A>, f: F) -> StaticParser<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: Debug + 'a + 'static,
    B: Debug + 'a + 'static, {
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(f(value), length),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  // StaticParserを使用する関数
  pub fn exists<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, bool>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success { value: _, length: _ } => crate::core::ParseResult::successful(true, 0),
        crate::core::ParseResult::Failure {
          error: _,
          committed_status,
        } => crate::core::ParseResult::successful(false, 0),
      }
    })
  }

  pub fn not<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, ()>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success { value: _, length: _ } => crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "not parser failed".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        ),
        crate::core::ParseResult::Failure {
          error: _,
          committed_status: _,
        } => crate::core::ParseResult::successful((), 0),
      }
    })
  }

  pub fn opt<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, Option<A>>
  where
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    or(map(attempt(parser), Some), successful(None))
  }

  pub fn or<'a, I: Clone, A>(parser1: StaticParser<'a, I, A>, parser2: StaticParser<'a, I, A>) -> StaticParser<'a, I, A>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result1 = parser1.run(state);
      match result1 {
        crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(value, length),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => {
          if committed_status == crate::core::CommittedStatus::Uncommitted {
            let result2 = parser2.run(state);
            match result2 {
              crate::core::ParseResult::Success { value, length } => {
                crate::core::ParseResult::successful(value, length)
              }
              crate::core::ParseResult::Failure {
                error: error2,
                committed_status: committed_status2,
              } => crate::core::ParseResult::failed(error2, committed_status2),
            }
          } else {
            crate::core::ParseResult::failed(error, committed_status)
          }
        }
      }
    })
  }

  pub fn and_then<'a, I: Clone, A, B>(
    parser1: StaticParser<'a, I, A>,
    parser2: StaticParser<'a, I, B>,
  ) -> StaticParser<'a, I, (A, B)>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result1 = parser1.run(state);
      match result1 {
        crate::core::ParseResult::Success {
          value: value1,
          length: consumed1,
        } => {
          let next_state = state.next(consumed1);
          let result2 = parser2.run(&next_state);
          match result2 {
            crate::core::ParseResult::Success {
              value: value2,
              length: consumed2,
            } => crate::core::ParseResult::successful((value1, value2), consumed1 + consumed2),
            crate::core::ParseResult::Failure {
              error,
              committed_status,
            } => crate::core::ParseResult::failed(error, committed_status),
          }
        }
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn attempt<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, A>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(value, length),
        crate::core::ParseResult::Failure {
          error,
          committed_status: _,
        } => crate::core::ParseResult::failed(error, crate::core::CommittedStatus::Uncommitted),
      }
    })
  }

  pub fn scan_right1<'a, I: Clone, A, BOP>(p: StaticParser<'a, I, A>, op: StaticParser<'a, I, BOP>) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = p.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: x,
          length: consumed1,
        } => {
          let next_state = state.next(consumed1);
          let result2 = op.run(&next_state);
          match result2 {
            crate::core::ParseResult::Success {
              value: f,
              length: consumed2,
            } => {
              let next_state2 = next_state.next(consumed2);
              let result3 = scan_right1(p.clone(), op.clone()).run(&next_state2);
              match result3 {
                crate::core::ParseResult::Success {
                  value: y,
                  length: consumed3,
                } => crate::core::ParseResult::successful(f(x, y), consumed1 + consumed2 + consumed3),
                crate::core::ParseResult::Failure {
                  error: _,
                  committed_status: _,
                } => crate::core::ParseResult::successful(x, consumed1),
              }
            }
            crate::core::ParseResult::Failure {
              error: _,
              committed_status: _,
            } => crate::core::ParseResult::successful(x, consumed1),
          }
        }
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn chain_right0<'a, I: Clone, A, BOP>(
    p: StaticParser<'a, I, A>,
    op: StaticParser<'a, I, BOP>,
    x: A,
  ) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    or(chain_right1(p, op), successful(x))
  }

  pub fn chain_left0<'a, I: Clone, A, BOP>(
    p: StaticParser<'a, I, A>,
    op: StaticParser<'a, I, BOP>,
    x: A,
  ) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    or(chain_left1(p, op), successful(x))
  }

  pub fn chain_right1<'a, I: Clone, A, BOP>(
    p: StaticParser<'a, I, A>,
    op: StaticParser<'a, I, BOP>,
  ) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    scan_right1(p, op)
  }

  pub fn chain_left1<'a, I: Clone, A, BOP>(p: StaticParser<'a, I, A>, op: StaticParser<'a, I, BOP>) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = p.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: x,
          length: consumed1,
        } => {
          let mut current_value = x;
          let mut current_consumed = consumed1;
          let mut current_state = state.next(consumed1);

          loop {
            let op_result = op.run(&current_state);
            match op_result {
              crate::core::ParseResult::Success {
                value: f,
                length: op_consumed,
              } => {
                let next_state = current_state.next(op_consumed);
                let p_result = p.run(&next_state);
                match p_result {
                  crate::core::ParseResult::Success {
                    value: y,
                    length: p_consumed,
                  } => {
                    current_value = f(current_value, y);
                    current_consumed += op_consumed + p_consumed;
                    current_state = next_state.next(p_consumed);
                  }
                  crate::core::ParseResult::Failure {
                    error,
                    committed_status,
                  } => {
                    return crate::core::ParseResult::failed(error, committed_status);
                  }
                }
              }
              crate::core::ParseResult::Failure {
                error: _,
                committed_status: _,
              } => {
                return crate::core::ParseResult::successful(current_value, current_consumed);
              }
            }
          }
        }
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn rest_right1<'a, I: Clone, A, BOP>(
    p: StaticParser<'a, I, A>,
    op: StaticParser<'a, I, BOP>,
    x: A,
  ) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = p.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => crate::core::ParseResult::successful(value, consumed),
        crate::core::ParseResult::Failure {
          error: _,
          committed_status: _,
        } => crate::core::ParseResult::successful(x.clone(), 0),
      }
    })
  }

  pub fn rest_left1<'a, I: Clone, A, BOP>(
    p: StaticParser<'a, I, A>,
    op: StaticParser<'a, I, BOP>,
    x: A,
  ) -> StaticParser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = p.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => crate::core::ParseResult::successful(value, consumed),
        crate::core::ParseResult::Failure {
          error: _,
          committed_status: _,
        } => crate::core::ParseResult::successful(x.clone(), 0),
      }
    })
  }
}
