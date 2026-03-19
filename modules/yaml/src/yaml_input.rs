use std::collections::HashMap;

use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::str_input::{StrCheckpoint, StrInput};

use crate::value::YamlValue;

/// YAML 専用の Input 型。StrInput に YAML 固有状態を統合する。
///
/// - アンカーマップ: `&name` で登録し `*name` で参照
/// - インデントスタック: `with_indent` コンビネータでスコープ管理
pub struct YamlInput<'a> {
  inner: StrInput<'a>,
  anchors: HashMap<String, YamlValue>,
  indent_stack: Vec<usize>,
}

impl<'a> YamlInput<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      inner: StrInput::new(src),
      anchors: HashMap::new(),
      indent_stack: Vec::new(),
    }
  }

  // ── Anchor methods ──

  pub fn set_anchor(&mut self, name: String, value: YamlValue) {
    self.anchors.insert(name, value);
  }

  pub fn get_anchor(&self, name: &str) -> Option<&YamlValue> {
    self.anchors.get(name)
  }

  // ── Indent stack methods ──

  pub fn push_indent(&mut self, indent: usize) {
    self.indent_stack.push(indent);
  }

  pub fn pop_indent(&mut self) {
    self.indent_stack.pop();
  }

  pub fn current_min_indent(&self) -> usize {
    self.indent_stack.last().copied().unwrap_or(0)
  }

  // ── StrInput delegate methods ──

  #[inline]
  pub fn peek_byte(&self) -> Option<u8> {
    self.inner.peek_byte()
  }

  #[inline]
  pub fn advance(&mut self, n: usize) {
    self.inner.advance(n);
  }

  /// Returns the remaining input as a string slice (same as `remaining()`).
  #[allow(dead_code)]
  pub fn as_str(&self) -> &'a str {
    self.inner.remaining()
  }

  /// Returns a mutable reference to the inner `StrInput`.
  /// Useful for running `StrInput`-specific text parsers (e.g. `char`, `quoted_string`).
  #[inline]
  pub fn inner_mut(&mut self) -> &mut StrInput<'a> {
    &mut self.inner
  }
}

impl<'a> Input for YamlInput<'a> {
  type Checkpoint = StrCheckpoint;
  type Error = ParseError;
  type Slice = &'a str;
  type Token = char;

  #[inline]
  fn next_token(&mut self) -> Option<char> {
    self.inner.next_token()
  }

  #[inline]
  fn peek_token(&self) -> Option<char> {
    self.inner.peek_token()
  }

  #[inline]
  fn slice_since(&self, cp: StrCheckpoint) -> &'a str {
    self.inner.slice_since(cp)
  }

  #[inline]
  fn checkpoint(&self) -> StrCheckpoint {
    self.inner.checkpoint()
  }

  #[inline]
  fn reset(&mut self, checkpoint: StrCheckpoint) {
    self.inner.reset(checkpoint);
  }

  #[inline]
  fn offset(&self) -> usize {
    self.inner.offset()
  }

  #[inline]
  fn remaining(&self) -> &'a str {
    self.inner.remaining()
  }

  #[inline]
  fn is_eof(&self) -> bool {
    self.inner.is_eof()
  }

  #[inline]
  fn line(&self) -> usize {
    self.inner.line()
  }

  #[inline]
  fn column(&self) -> usize {
    self.inner.column()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use oni_comb_parser::input::Input;

  #[test]
  fn input_delegation_works() {
    let mut input = YamlInput::new("abc");
    assert_eq!(input.next_token(), Some('a'));
    assert_eq!(input.remaining(), "bc");
    assert_eq!(input.offset(), 1);
    assert_eq!(input.line(), 1);
    assert_eq!(input.column(), 2);
  }

  #[test]
  fn checkpoint_and_reset() {
    let mut input = YamlInput::new("abc");
    input.next_token();
    input.next_token();
    let cp = input.checkpoint();
    input.next_token();
    assert_eq!(input.remaining(), "");
    input.reset(cp);
    assert_eq!(input.remaining(), "c");
  }

  #[test]
  fn anchor_set_and_get() {
    let mut input = YamlInput::new("");
    input.set_anchor("ref".to_string(), YamlValue::Integer(42));
    assert_eq!(input.get_anchor("ref"), Some(&YamlValue::Integer(42)));
    assert_eq!(input.get_anchor("unknown"), None);
  }

  #[test]
  fn indent_stack_push_pop() {
    let mut input = YamlInput::new("");
    assert_eq!(input.current_min_indent(), 0);
    input.push_indent(2);
    assert_eq!(input.current_min_indent(), 2);
    input.push_indent(4);
    assert_eq!(input.current_min_indent(), 4);
    input.pop_indent();
    assert_eq!(input.current_min_indent(), 2);
    input.pop_indent();
    assert_eq!(input.current_min_indent(), 0);
  }

  #[test]
  fn empty_indent_stack_returns_zero() {
    let mut input = YamlInput::new("");
    input.pop_indent(); // should not panic
    assert_eq!(input.current_min_indent(), 0);
  }
}
