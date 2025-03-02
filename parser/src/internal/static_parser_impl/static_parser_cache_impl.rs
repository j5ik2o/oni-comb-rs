// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseResult, StaticParser};
use crate::extension::parser::CacheParser;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ptr;

use fnv::FnvHashMap;

impl<'a, I: Clone, A: 'a> CacheParser<'a> for StaticParser<'a, I, A> {
  fn cache(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    // FnvHashMapを使用してキャッシュを作成（キーは単純な文字列ではなくタプル）
    let caches = RefCell::new(FnvHashMap::<(usize, usize, usize), ParseResult<'a, I, A>>::default());
    let method = self.method.clone();

    StaticParser::new(move |parser_state| {
      // キーをタプルとして生成（文字列変換なし）
      let key = (
        parser_state as *const _ as usize,
        parser_state.last_offset().unwrap_or(0),
        ptr::addr_of!(method) as usize,
      );

      // キャッシュから結果を取得または計算
      let parse_result = caches
        .borrow_mut()
        .entry(key)
        .or_insert_with(|| (method)(parser_state))
        .clone();

      parse_result
    })
  }
}
