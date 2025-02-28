use crate::core::{Parser, ParserRunner};
use crate::extension::parsers::CacheParsers;
use crate::internal::ParsersImpl;
use std::cell::RefCell;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt::Debug;

impl CacheParsers for ParsersImpl {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    // 高速なハッシュマップを使用
    let caches = RefCell::new(HashMap::with_capacity(32)); // 初期容量を指定
    
    // パーサーのメソッドポインタのハッシュ値を事前に計算
    let parser_method_hash = {
      let mut hasher = DefaultHasher::new();
      format!("{:p}", &parser.method).hash(&mut hasher);
      hasher.finish()
    };
    
    Parser::new(move |parser_state| {
      // キーとしてタプルを使用（文字列フォーマットを避ける）
      let offset = parser_state.next_offset();
      let key = (offset, parser_method_hash);
      
      // キャッシュから結果を取得または計算
      let mut cache_ref = caches.borrow_mut();
      if let Some(result) = cache_ref.get(&key) {
        // キャッシュヒット
        result.clone()
      } else {
        // キャッシュミス - 結果を計算してキャッシュに保存
        let result = parser.run(parser_state);
        cache_ref.insert(key, result.clone());
        result
      }
    })
  }
}
