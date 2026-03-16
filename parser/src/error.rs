use std::fmt;

/// `or` で左右の Backtrack エラーを合成するトレイト。
pub trait MergeError: Sized {
    fn merge(self, other: Self) -> Self;
}

/// `.context()` でコンテキストラベルを積むトレイト。
pub trait ContextError: Sized {
    fn add_context(self, context: &'static str) -> Self;
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

/// 構造化パースエラー。位置・期待トークン・コンテキストを保持する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// 失敗した byte offset
    pub position: usize,
    /// 期待していたトークンの集合
    pub expected: Vec<Expected>,
    /// コンテキストスタック（外側から内側の順）
    pub context: Vec<&'static str>,
}

impl ParseError {
    pub fn new(position: usize, expected: Expected) -> Self {
        ParseError {
            position,
            expected: vec![expected],
            context: Vec::new(),
        }
    }

    pub fn expected_char(position: usize, c: char) -> Self {
        Self::new(position, Expected::Char(c))
    }

    pub fn expected_tag(position: usize, tag: &'static str) -> Self {
        Self::new(position, Expected::Tag(tag))
    }

    pub fn expected_description(position: usize, desc: &'static str) -> Self {
        Self::new(position, Expected::Description(desc))
    }

    pub fn expected_eof(position: usize) -> Self {
        Self::new(position, Expected::Eof)
    }
}

impl MergeError for ParseError {
    fn merge(mut self, other: Self) -> Self {
        use std::cmp::Ordering;
        match self.position.cmp(&other.position) {
            Ordering::Greater => self,
            Ordering::Less => other,
            Ordering::Equal => {
                for e in other.expected {
                    if !self.expected.contains(&e) {
                        self.expected.push(e);
                    }
                }
                self
            }
        }
    }
}

impl ContextError for ParseError {
    fn add_context(mut self, context: &'static str) -> Self {
        self.context.push(context);
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at position {}", self.position)?;

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
