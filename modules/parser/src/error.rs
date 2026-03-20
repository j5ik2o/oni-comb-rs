#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

use crate::input_position::InputPosition;

/// `or` で左右の Backtrack エラーを合成するトレイト。
pub trait MergeError: Sized {
  fn merge(self, other: Self) -> Self;
}

/// `.context()` でコンテキストラベルを積むトレイト。
pub trait ContextError: Sized {
  fn add_context(self, context: &'static str) -> Self;
}

/// パーサーがエラーを生成するための trait。
pub trait ExpectError: Sized {
  fn from_position(position: InputPosition, expected: Expected) -> Self;

  #[inline]
  fn from_expected(position: usize, expected: Expected) -> Self {
    Self::from_position(InputPosition::offset_only(position), expected)
  }
}

/// パース失敗時の期待トークン。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expected {
  /// 特定の文字を期待
  Char(char),
  /// 特定の文字列タグを期待
  Tag(&'static str),
  /// 特定のバイトを期待（将来の bytes 対応用）
  Byte(u8),
  /// 特定のバイト列タグを期待（将来の bytes 対応用）
  ByteTag(&'static [u8]),
  /// 説明的な期待（"digit", "identifier" 等）
  Description(&'static str),
  /// 入力の終端を期待
  Eof,
}

/// core-only 環境用の軽量エラー型。位置のみ保持。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinimalError {
  pub position: usize,
}

impl ExpectError for MinimalError {
  #[inline]
  fn from_position(position: InputPosition, _expected: Expected) -> Self {
    Self {
      position: position.offset,
    }
  }
}

impl MergeError for MinimalError {
  #[inline]
  fn merge(self, other: Self) -> Self {
    if self.position >= other.position {
      self
    } else {
      other
    }
  }
}

impl ContextError for MinimalError {
  #[inline]
  fn add_context(self, _context: &'static str) -> Self {
    self
  }
}

impl fmt::Display for MinimalError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "parse error at position {}", self.position)
  }
}

/// 構造化パースエラー。位置・期待トークン・コンテキストを保持する。
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
  /// 失敗した byte offset
  pub position: usize,
  /// 失敗した行番号 (1-origin)。Input から取得できない場合は 0。
  pub line: usize,
  /// 失敗した列番号 (1-origin)。Input から取得できない場合は 0。
  pub column: usize,
  /// 失敗位置の行頭を指す byte anchor。
  pub line_start: usize,
  /// 期待していたトークンの集合
  pub expected: Vec<Expected>,
  /// コンテキストスタック（外側から内側の順）
  pub context: Vec<&'static str>,
}

#[cfg(feature = "alloc")]
impl ExpectError for ParseError {
  #[inline(always)]
  fn from_position(position: InputPosition, expected: Expected) -> Self {
    ParseError {
      position: position.offset,
      line: position.line,
      column: position.column,
      line_start: position.line_start,
      expected: vec![expected],
      context: Vec::new(),
    }
  }
}

#[cfg(feature = "alloc")]
impl ParseError {
  /// line/column/line_start をまとめて更新する。
  pub fn with_position(mut self, position: InputPosition) -> Self {
    self.position = position.offset;
    self.line = position.line;
    self.column = position.column;
    self.line_start = position.line_start;
    self
  }

  /// position フィールドからソーステキストを走査して line/column を計算し設定する。
  /// line/column が未設定 (0) の場合のみ上書きする。
  pub fn fill_location_from_src(mut self, src: &str) -> Self {
    if self.line == 0 && self.position <= src.len() {
      let mut line = 1;
      let mut col = 1;
      let mut line_start = 0;
      for (i, c) in src.char_indices() {
        if i >= self.position {
          break;
        }
        if c == '\n' {
          line += 1;
          col = 1;
          line_start = i + 1;
        } else {
          col += 1;
        }
      }
      self.line = line;
      self.column = col;
      self.line_start = line_start;
    }
    self
  }
}

#[cfg(feature = "alloc")]
impl MergeError for ParseError {
  fn merge(mut self, other: Self) -> Self {
    use core::cmp::Ordering;
    match self.position.cmp(&other.position) {
      Ordering::Greater => self,
      Ordering::Less => other,
      Ordering::Equal => {
        for e in other.expected {
          if !self.expected.contains(&e) {
            self.expected.push(e);
          }
        }
        if self.line == 0 {
          self.line = other.line;
          self.column = other.column;
          self.line_start = other.line_start;
        }
        if self.context.is_empty() {
          self.context = other.context;
        }
        self
      }
    }
  }
}

#[cfg(feature = "alloc")]
impl ContextError for ParseError {
  fn add_context(mut self, context: &'static str) -> Self {
    self.context.push(context);
    self
  }
}

#[cfg(feature = "alloc")]
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.line > 0 {
      write!(f, "parse error at line {}:{}", self.line, self.column)?;
    } else {
      write!(f, "parse error at position {}", self.position)?;
    }

    if !self.expected.is_empty() {
      write!(f, ": expected ")?;
      for (i, e) in self.expected.iter().enumerate() {
        if i > 0 {
          write!(f, " or ")?;
        }
        match e {
          Expected::Char(c) => write!(f, "'{}'", c)?,
          Expected::Tag(s) => write!(f, "\"{}\"", s)?,
          Expected::Byte(b) => write!(f, "0x{:02X}", b)?,
          Expected::ByteTag(bs) => write!(f, "{:?}", bs)?,
          Expected::Description(d) => write!(f, "{}", d)?,
          Expected::Eof => write!(f, "end of input")?,
        }
      }
    }

    if !self.context.is_empty() {
      write!(f, " in ")?;
      for (i, ctx) in self.context.iter().rev().enumerate() {
        if i > 0 {
          write!(f, " > ")?;
        }
        write!(f, "{}", ctx)?;
      }
    }

    Ok(())
  }
}
