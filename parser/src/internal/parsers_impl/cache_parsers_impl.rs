use crate::core::{Parser, ParserRunner};
use crate::extension::parsers::CacheParsers;
use crate::internal::ParsersImpl;
use std::cell::RefCell;

use fnv::FnvHashMap;
use std::fmt::Debug;
use std::ptr;

impl CacheParsers for ParsersImpl {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    // FnvHashMapを使用してキャッシュを作成（キーは単純な文字列ではなくタプル）
    let caches = RefCell::new(FnvHashMap::<(usize, usize, usize), ParseResult<'a, I, A>>::default());

    Parser::new(move |parser_state| {
      // キーをタプルとして生成（文字列変換なし）
      let key = (
        parser_state as *const _ as usize,
        parser_state.last_offset().unwrap_or(0),
        ptr::addr_of!(parser.method) as usize,
      );

      // キャッシュから結果を取得または計算
      let parse_result = caches
        .borrow_mut()
        .entry(key)
        .or_insert_with(|| parser.run(parser_state))
        .clone();

      parse_result
    })
  }
}

// ParseResultの型をインポート
use crate::core::ParseResult;
