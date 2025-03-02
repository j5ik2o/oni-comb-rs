use crate::core::{ParseResult, ParseState, Parser};
use std::marker::PhantomData;
use std::rc::Rc;

/// StaticParserの内部関数型
pub type StaticParseFn<'a, I, A> = dyn Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a;

/// 静的ディスパッチを使用した最適化されたパーサー実装
pub struct StaticParser<'a, I, A: 'a> {
  pub(crate) method: Rc<StaticParseFn<'a, I, A>>,
  _phantom: PhantomData<&'a I>,
}

impl<'a, I, A: 'a> Clone for StaticParser<'a, I, A> {
  fn clone(&self) -> Self {
    Self {
      method: Rc::clone(&self.method),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A: 'a> StaticParser<'a, I, A> {
  /// 新しいStaticParserを作成
  pub fn new<F>(parse: F) -> Self
  where
    F: Fn(&ParseState<'a, I>) -> ParseResult<'a, I, A> + 'a, {
    StaticParser {
      method: Rc::new(parse),
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
  pub fn map<B, G>(self, f: G) -> StaticParser<'a, I, B>
  where
    G: Fn(A) -> B + Clone + 'a,
    B: Clone + 'a, {
    let method = self.method.clone();
    let f_clone = f.clone();

    StaticParser::new(move |state| match (method)(state) {
      ParseResult::Success { value, length } => ParseResult::successful(f_clone(value), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  /// 条件に基づいてフィルタリング
  pub fn filter<G>(self, f: G) -> StaticParser<'a, I, A>
  where
    G: Fn(&A) -> bool + Clone + 'a,
    I: Clone + 'a, {
    self.with_filter(f)
  }

  /// 条件に基づいてフィルタリング
  pub fn with_filter<G>(self, f: G) -> StaticParser<'a, I, A>
  where
    G: Fn(&A) -> bool + Clone + 'a,
    I: Clone + 'a, {
    let method = self.method.clone();
    let f_clone = f.clone();

    StaticParser::new(move |state| match (method)(state) {
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
  pub fn with_filter_not<G>(self, f: G) -> StaticParser<'a, I, A>
  where
    G: Fn(&A) -> bool + Clone + 'a,
    I: Clone + 'a, {
    let f_clone = f.clone();
    self.with_filter(move |a| !f_clone(a))
  }

  /// フラットマップ操作
  pub fn flat_map<B, G>(self, f: G) -> StaticParser<'a, I, B>
  where
    G: Fn(A) -> StaticParser<'a, I, B> + Clone + 'a,
    B: Clone + 'a, {
    let method = self.method.clone();
    let f_clone = f.clone();

    StaticParser::new(move |state| match (method)(state) {
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
  pub fn of_many0_sep<B, G>(self, sep: G) -> StaticParser<'a, I, Vec<A>>
  where
    G: Clone + 'a,
    G: for<'b> Fn(&'b ParseState<'a, I>) -> ParseResult<'a, I, B>,
    A: Clone + 'a,
    B: 'a, {
    let method = self.method.clone();
    let sep_clone = sep.clone();

    StaticParser::new(move |state| {
      let mut result = Vec::new();
      let mut total_length = 0;
      let mut current_offset = 0;

      // 最初の要素をパース
      match (method)(state) {
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
        match (sep_clone)(&current_state) {
          ParseResult::Success { length: sep_length, .. } => {
            let next_state = current_state.add_offset(sep_length);

            // 次の要素をパース
            match (method)(&next_state) {
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

  /// 1回以上の繰り返しを解析する
  pub fn of_many1(self) -> StaticParser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |state| {
      let mut result = Vec::new();
      let mut total_length = 0;
      let mut current_offset = 0;
      let mut first = true;

      loop {
        let current_state = state.add_offset(current_offset);
        match (method)(&current_state) {
          ParseResult::Success { value, length } => {
            result.push(value);
            total_length += length;
            current_offset += length;
            first = false;
          }
          ParseResult::Failure {
            error,
            committed_status,
          } => {
            if first {
              return ParseResult::failed(error, committed_status);
            }
            break;
          }
        }
      }

      ParseResult::successful(result, total_length)
    })
  }

  /// 結果を収集する
  pub fn collect(self) -> StaticParser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |state| match (method)(state) {
      ParseResult::Success { value, length } => {
        let mut result = Vec::new();
        result.push(value);
        ParseResult::successful(result, length)
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  /// StaticParserをParserに変換する
  pub fn to_parser(self) -> Parser<'a, I, A> {
    let method = self.method.clone();
    Parser::new(move |state| (method)(state))
  }
}

/// ParserからStaticParserへの変換
impl<'a, I, A: 'a> Parser<'a, I, A> {
  #[deprecated(
    since = "1.0.0",
    note = "直接StaticParserを使用してください。将来のバージョンで削除される予定です。"
  )]
  pub fn to_static_parser(self) -> StaticParser<'a, I, A> {
    let method = self.method;
    StaticParser::new(move |state| (method)(state))
  }
}
