use crate::core::{ParseResult, ParseState, Parser};
use std::marker::PhantomData;
use std::rc::Rc;

/// 静的ディスパッチを使用した最適化されたパーサー実装
pub struct StaticParser<'a, I, A, F>
where
  F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a, {
  pub(crate) method: F,
  _phantom: PhantomData<&'a (I, A)>,
}

impl<'a, I, A, F> Clone for StaticParser<'a, I, A, F>
where
  F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn clone(&self) -> Self {
    Self {
      method: self.method.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A, F> StaticParser<'a, I, A, F>
where
  F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  /// 新しいStaticParserを作成
  pub fn new(parse: F) -> Self {
    StaticParser {
      method: parse,
      _phantom: PhantomData,
    }
  }

  /// パース状態を受け取り、パース結果を返す
  pub fn run(&self, param: &ParseState<'a, I>) -> ParseResult<'a, I, A> {
    (self.method)(param)
  }

  /// 入力を受け取り、パース結果を返す
  pub fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
    let parse_state = ParseState::new(input, 0);
    self.run(&parse_state)
  }

  /// 関数を適用して新しい型に変換
  pub fn map<B, G>(
    self,
    f: G,
  ) -> StaticParser<'a, I, B, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a>
  where
    G: Fn(A) -> B + Clone + 'a,
    B: Clone + 'a, {
    let method = self.method;
    let f_clone = f.clone();

    StaticParser::new(move |state| match method(state) {
      ParseResult::Success { value, length } => ParseResult::successful(f_clone(value), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  /// 条件に基づいてフィルタリング
  pub fn with_filter<G>(
    self,
    f: G,
  ) -> StaticParser<'a, I, A, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a>
  where
    G: Fn(&A) -> bool + Clone + 'a,
    I: Clone + 'a, {
    let method = self.method;
    let f_clone = f.clone();

    StaticParser::new(move |state| match method(state) {
      ParseResult::Success { value, length } => {
        if f_clone(&value) {
          ParseResult::successful(value, length)
        } else {
          let input = state.input();
          let offset = state.last_offset().unwrap_or(0);
          let msg = format!("no matched to predicate: last offset: {}", offset);
          let ps = state.add_offset(length);
          let pe = crate::core::ParseError::of_mismatch(input, ps.next_offset(), length, msg);
          ParseResult::failed_with_uncommitted(pe)
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  /// 条件の否定に基づいてフィルタリング
  pub fn with_filter_not<G>(
    self,
    f: G,
  ) -> StaticParser<'a, I, A, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a>
  where
    G: Fn(&A) -> bool + Clone + 'a,
    I: Clone + 'a, {
    let f_clone = f.clone();
    self.with_filter(move |a| !f_clone(a))
  }

  /// フラットマップ操作
  pub fn flat_map<B, G, H>(
    self,
    f: G,
  ) -> StaticParser<'a, I, B, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a>
  where
    G: Fn(A) -> StaticParser<'a, I, B, H> + Clone + 'a,
    H: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a,
    B: Clone + 'a, {
    let method = self.method;
    let f_clone = f.clone();

    StaticParser::new(move |state| match method(state) {
      ParseResult::Success { value: a, length: n } => {
        let ps = state.add_offset(n);
        let parser_b = f_clone(a);
        parser_b.run(&ps).with_committed_fallback(n != 0).with_add_length(n)
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  /// 区切り文字を使用して複数の要素をパースする
  pub fn of_many0_sep<B, G>(
    self,
    sep: G,
  ) -> StaticParser<'a, I, Vec<A>, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, Vec<A>> + Clone + 'a>
  where
    G: Clone + 'a,
    G: for<'b> Fn(&'b ParseState<'a, I>) -> ParseResult<'a, I, B>,
    A: Clone + 'a,
    B: 'a, {
    let method = self.method;
    let sep_clone = sep.clone();

    StaticParser::new(move |state| {
      let mut result = Vec::new();
      let mut total_length = 0;
      let mut current_offset = 0;

      // 最初の要素をパース
      match method(state) {
        ParseResult::Success { value, length } => {
          result.push(value);
          total_length += length;
          current_offset += length;
        }
        ParseResult::Failure {
          error: _,
          committed_status: _,
        } => {
          // 最初の要素がない場合は空のベクターを返す
          return ParseResult::successful(Vec::new(), 0);
        }
      }

      // 残りの要素をパース
      loop {
        // 現在の状態を計算
        let current_state = state.add_offset(current_offset);

        // 区切り文字をパース
        match sep_clone(&current_state) {
          ParseResult::Success { length: sep_length, .. } => {
            let next_state = current_state.add_offset(sep_length);

            // 次の要素をパース
            match method(&next_state) {
              ParseResult::Success { value, length } => {
                result.push(value);
                total_length += sep_length + length;
                current_offset += sep_length + length;
              }
              ParseResult::Failure {
                error,
                committed_status,
              } => {
                // 区切り文字の後に要素がない場合はエラー
                if committed_status.is_committed() {
                  return ParseResult::failed(error, committed_status);
                }
                break;
              }
            }
          }
          ParseResult::Failure {
            error,
            committed_status,
          } => {
            // 区切り文字がない場合は終了
            if committed_status.is_committed() {
              return ParseResult::failed(error, committed_status);
            }
            break;
          }
        }
      }

      ParseResult::successful(result, total_length)
    })
  }

  /// StaticParserをParserに変換する
  pub fn to_parser(self) -> Parser<'a, I, A> {
    let method = self.method;
    Parser::new(move |state| method(state))
  }
}

/// ParserからStaticParserへの変換
impl<'a, I, A> Parser<'a, I, A> {
  pub fn to_static_parser(
    self,
  ) -> StaticParser<'a, I, A, impl Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a> {
    let method = self.method.clone();
    StaticParser::new(move |state| method(state))
  }
}
