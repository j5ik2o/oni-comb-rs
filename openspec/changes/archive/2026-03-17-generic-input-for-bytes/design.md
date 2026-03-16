# Generic Input for Bytes — 設計

## 概要

`Input` トレイトに `Token`/`Slice` を追加してジェネリック化し、`ByteInput<'a>` を新設する。
ジェネリック化可能なパーサー（take, satisfy, take_while 等）を `primitive/` モジュールに移動する。

## Input トレイトの変更

### 現状

```rust
pub trait Input {
  type Checkpoint: Copy + Eq + Ord;
  type Slice<'a> where Self: 'a;  // GAT

  fn checkpoint(&self) -> Self::Checkpoint;
  fn reset(&mut self, checkpoint: Self::Checkpoint);
  fn offset(&self) -> usize;
  fn remaining(&self) -> Self::Slice<'_>;
  fn is_eof(&self) -> bool;
}
```

### 変更後

```rust
pub trait Input {
  type Token: Copy + Eq;
  type Slice;                        // GAT → 通常の associated type
  type Checkpoint: Copy + Eq + Ord;

  // --- トークン操作（新規） ---
  /// 1トークン消費して返す。EOF なら None。
  fn next_token(&mut self) -> Option<Self::Token>;
  /// 次のトークンを消費せずに返す。
  fn peek_token(&self) -> Option<Self::Token>;
  /// checkpoint から現在位置までの Slice を返す。
  fn slice_since(&self, cp: Self::Checkpoint) -> Self::Slice;

  // --- 既存（維持） ---
  fn checkpoint(&self) -> Self::Checkpoint;
  fn reset(&mut self, cp: Self::Checkpoint);
  fn offset(&self) -> usize;
  fn remaining(&self) -> Self::Slice;   // 戻り型が Slice<'_> → Slice に変更
  fn is_eof(&self) -> bool;
}
```

### Slice を GAT から通常の associated type にする理由

`Parser::Output` にはライフタイムパラメータがない。GAT の `Slice<'s>` だと
`type Output = I::Slice<'???>` のライフタイムを表現できない。

`type Slice = &'a str`（`'a` は `Self = StrInput<'a>` から取得）にすれば、
`type Output = I::Slice` と書ける。コンパイラは `&'a str` が `&mut self` から
借用していないと判断でき、ライフタイムの衝突は起きない。

```rust
// StrInput<'a> の as_str() は &'a str を返す（self 借用ではなく src 借用）
pub(crate) fn as_str(&self) -> &'a str {
    &self.src[self.offset..]
}
```

`remaining()` や `slice_since()` も同様に `&'a str` を返す。
`&mut self` を取る `parse_next` 内で `slice_since` を呼んだ後も `input` を使い続けられる。

## StrInput<'a> の変更

```rust
impl<'a> Input for StrInput<'a> {
    type Token = char;
    type Slice = &'a str;
    type Checkpoint = usize;

    #[inline]
    fn next_token(&mut self) -> Option<char> {
        let c = self.as_str().chars().next()?;
        self.offset += c.len_utf8();
        Some(c)
    }

    #[inline]
    fn peek_token(&self) -> Option<char> {
        self.as_str().chars().next()
    }

    #[inline]
    fn slice_since(&self, cp: usize) -> &'a str {
        &self.src[cp..self.offset]
    }

    // remaining, checkpoint, reset, offset, is_eof は既存のまま
    // （remaining の戻り型だけ Slice<'_> → &'a str に変更）
}
```

## ByteInput<'a>（新規）

```rust
pub struct ByteInput<'a> {
    src: &'a [u8],
    offset: usize,
}

impl<'a> ByteInput<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self { src, offset: 0 }
    }

    pub(crate) fn advance(&mut self, n: usize) {
        self.offset += n;
    }

    pub(crate) fn as_bytes(&self) -> &'a [u8] {
        &self.src[self.offset..]
    }

    #[inline]
    pub fn peek_byte(&self) -> Option<u8> {
        self.src.get(self.offset).copied()
    }
}

impl<'a> Input for ByteInput<'a> {
    type Token = u8;
    type Slice = &'a [u8];
    type Checkpoint = usize;

    #[inline]
    fn next_token(&mut self) -> Option<u8> {
        let b = self.src.get(self.offset).copied()?;
        self.offset += 1;
        Some(b)
    }

    #[inline]
    fn peek_token(&self) -> Option<u8> {
        self.src.get(self.offset).copied()
    }

    #[inline]
    fn slice_since(&self, cp: usize) -> &'a [u8] {
        &self.src[cp..self.offset]
    }

    fn checkpoint(&self) -> usize { self.offset }
    fn reset(&mut self, cp: usize) { self.offset = cp; }
    fn offset(&self) -> usize { self.offset }
    fn remaining(&self) -> &'a [u8] { &self.src[self.offset..] }
    fn is_eof(&self) -> bool { self.offset >= self.src.len() }
}
```

## ジェネリックパーサー（primitive/ モジュール）

text/ から以下を primitive/ に移動し、`I: Input` でジェネリック化する。

### take

```rust
pub struct Take { n: usize }

pub fn take(n: usize) -> Take { Take { n } }

impl<I: Input> Parser<I> for Take {
    type Output = I::Slice;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
        let pos = input.offset();
        let cp = input.checkpoint();
        for _ in 0..self.n {
            if input.next_token().is_none() {
                input.reset(cp);
                return Err(Fail::Backtrack(
                    ParseError::expected_description(pos, "enough input"),
                ));
            }
        }
        Ok(input.slice_since(cp))
    }
}
```

### satisfy

```rust
pub struct Satisfy<F>(F);

pub fn satisfy<F>(f: F) -> Satisfy<F> { Satisfy(f) }

impl<I: Input, F> Parser<I> for Satisfy<F>
where
    F: FnMut(I::Token) -> bool,
{
    type Output = I::Token;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, ParseError> {
        let pos = input.offset();
        match input.peek_token() {
            Some(t) if (self.0)(t) => {
                input.next_token();
                Ok(t)
            }
            _ => Err(Fail::Backtrack(
                ParseError::expected_description(pos, "satisfy"),
            )),
        }
    }
}
```

### take_while0

```rust
pub struct TakeWhile0<F>(F);

pub fn take_while0<F>(f: F) -> TakeWhile0<F> { TakeWhile0(f) }

impl<I: Input, F> Parser<I> for TakeWhile0<F>
where
    F: FnMut(I::Token) -> bool,
{
    type Output = I::Slice;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
        let cp = input.checkpoint();
        while let Some(t) = input.peek_token() {
            if (self.0)(t) {
                input.next_token();
            } else {
                break;
            }
        }
        Ok(input.slice_since(cp))
    }
}
```

### take_while1

take_while0 と同じだが、消費0の場合は Backtrack エラー。
`cp == input.checkpoint()` で判定。

### take_while_n_m

```rust
pub struct TakeWhileNM<F> { min: usize, max: usize, f: F }

impl<I: Input, F> Parser<I> for TakeWhileNM<F>
where
    F: FnMut(I::Token) -> bool,
{
    type Output = I::Slice;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
        let pos = input.offset();
        let cp = input.checkpoint();
        let mut count = 0;
        while count < self.max {
            match input.peek_token() {
                Some(t) if (self.f)(t) => {
                    input.next_token();
                    count += 1;
                }
                _ => break,
            }
        }
        if count < self.min {
            input.reset(cp);
            return Err(Fail::Backtrack(
                ParseError::expected_description(pos, "not enough matching tokens"),
            ));
        }
        Ok(input.slice_since(cp))
    }
}
```

### eof

```rust
pub struct Eof;

pub fn eof() -> Eof { Eof }

impl<I: Input> Parser<I> for Eof {
    type Output = ();
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<(), ParseError> {
        if input.is_eof() {
            Ok(())
        } else {
            Err(Fail::Backtrack(ParseError::expected_eof(input.offset())))
        }
    }
}
```

## テキスト専用パーサー（text/ に残す）

以下は `StrInput` 専用のまま。`Parser<StrInput<'_>>` で実装。

| パーサー | 理由 |
|---------|------|
| `tag` | リテラル型が `&'static str`。バイト版は別途 `byte_tag` が必要になれば追加 |
| `char_` | `char` リテラルにマッチ |
| `identifier` | ASCII 文字判定 + chars() イテレーション |
| `integer` | digits + `str::parse::<i64>()` |
| `quoted_string` / `quoted_string_cow` | エスケープシーケンス処理 |
| `escaped` | エスケープシーケンス処理 |
| `whitespace0/1` | `satisfy` ジェネリック化に伴い書き換えが必要だが、`char` の whitespace 判定が前提なので text/ に残す |

### whitespace の変更

`whitespace0/1` は内部で `take_while0/1` を使用。`take_while0/1` がジェネリック化されるため、
呼び出しは変わらないが、`fn(char) -> bool` という型注釈で `StrInput` 用に制約される。

```rust
// 変更不要（take_while0 がジェネリックになっても fn(char) -> bool で Token=char に固定される）
pub fn whitespace0() -> TakeWhile0<fn(char) -> bool> {
    take_while0(is_ws as fn(char) -> bool)
}
```

### lexeme の変更

`lexeme` は `whitespace0()` を使う。`take_while0` のジェネリック化により型は変わるが、
`Token = char` の制約は `fn(char) -> bool` で自然に入る。StrInput 専用のまま。

## Recursive のジェネリック化

```rust
type DynParser<'a, I, O, E> = dyn Parser<I, Output = O, Error = E> + 'a;

struct RecursiveInner<'a, I: Input, O, E> {
    inner: UnsafeCell<Option<Box<DynParser<'a, I, O, E>>>>,
}

pub struct Recursive<'a, I: Input, O, E> {
    shared: Rc<RecursiveInner<'a, I, O, E>>,
}

impl<'a, I: Input, O, E> Parser<I> for Recursive<'a, I, O, E> {
    type Output = O;
    type Error = E;

    #[inline]
    fn parse_next(&mut self, input: &mut I) -> PResult<O, E> {
        unsafe {
            (*self.shared.inner.get())
                .as_mut()
                .expect("recursive parser not initialized")
                .parse_next(input)
        }
    }
}

pub fn recursive<'a, I, O, E, F, P>(f: F) -> Recursive<'a, I, O, E>
where
    I: Input,
    F: FnOnce(Recursive<'a, I, O, E>) -> P,
    P: Parser<I, Output = O, Error = E> + 'a,
{
    // 既存と同じロジック
}
```

## モジュール構成

```
src/
├── lib.rs              byte_input, primitive モジュール追加
├── input.rs            Token, Slice(非GAT), next_token, peek_token, slice_since 追加
├── str_input.rs        新 Input impl に適合
├── byte_input.rs       新規
├── parser.rs           変更なし
├── parser_ext.rs       変更なし
├── fail.rs             変更なし
├── error.rs            変更なし
│
├── primitive/           新規
│   ├── mod.rs
│   ├── take.rs          impl<I: Input> Parser<I>
│   ├── satisfy.rs       impl<I: Input, F> Parser<I>
│   ├── take_while0.rs   impl<I: Input, F> Parser<I>
│   ├── take_while1.rs   impl<I: Input, F> Parser<I>
│   ├── take_while_n_m.rs impl<I: Input, F> Parser<I>
│   └── eof.rs           impl<I: Input> Parser<I>
│
├── text/                StrInput 専用パーサー
│   ├── mod.rs           take_while.rs 削除、satisfy/eof/take 系を primitive/ の re-export に変更
│   ├── char.rs          変更なし
│   ├── tag.rs           変更なし
│   ├── identifier.rs    内部で primitive::satisfy を使うよう変更
│   ├── integer.rs       変更なし（as_str() 直接使用）
│   ├── whitespace.rs    primitive::take_while0/1 を使用（変更軽微）
│   ├── lexeme.rs        変更軽微
│   ├── quoted_string.rs 変更なし
│   ├── quoted_string_cow.rs 変更なし
│   └── escaped.rs       変更なし
│
├── combinator/          既にジェネリック（変更軽微）
│   ├── recursive.rs     I: Input ジェネリック化
│   └── ...              変更なし
│
└── prelude.rs           re-export パス更新 + ByteInput 追加

```

### prelude.rs の変更

```rust
pub use crate::parser::Parser;
pub use crate::parser_ext::ParserExt;
pub use crate::str_input::StrInput;
pub use crate::byte_input::ByteInput;   // 追加

// primitive/ から re-export（text/ からの re-export を置き換え）
pub use crate::primitive::take::take;
pub use crate::primitive::satisfy::satisfy;
pub use crate::primitive::take_while0::take_while0;
pub use crate::primitive::take_while1::take_while1;
pub use crate::primitive::take_while_n_m::take_while_n_m;
pub use crate::primitive::eof::eof;

// text/ 専用パーサー（変更なし）
pub use crate::text::char::char;
pub use crate::text::tag::tag;
pub use crate::text::identifier::identifier;
pub use crate::text::integer::integer;
pub use crate::text::whitespace::{whitespace0, whitespace1};
pub use crate::text::lexeme::lexeme;
pub use crate::text::quoted_string::quoted_string;
pub use crate::text::quoted_string_cow::quoted_string_cow;
pub use crate::text::escaped::escaped;

pub use crate::combinator::fn_parser::fn_parser;
pub use crate::combinator::recursive::recursive;
```

## Fail 意味論

全ジェネリックパーサーの Fail 伝播は既存の text/ 版と同一。
take_while_n_m で min 未満の場合、checkpoint に reset してから Backtrack を返す。

## 既存テストへの影響

text/ のパーサーは StrInput 用のまま re-export されるため、
既存テストは **import パスの変更なし** でコンパイルが通るはず。
型推論で `StrInput` が確定するケースがほとんど。
